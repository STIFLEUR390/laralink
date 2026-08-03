use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

use tauri::{AppHandle, Emitter, Manager};

use crate::error::{AppError, AppResult};
use crate::models::{DatabaseInput, LogEvent, PrelaunchInput, RuntimeInput, StatusEvent};
use crate::services::db_checker;
use crate::services::network_detector;
use crate::services::php_runtime;
use crate::services::port_scanner;
use crate::services::process_manager::kill_pid;
use crate::services::now_iso;
use crate::AppState;

/// Configuration complète nécessaire au lancement d'un projet.
#[derive(Debug, Clone)]
pub struct LaunchConfig {
	pub project_id: i64,
	pub laravel_path: PathBuf,
	pub runtime: RuntimeInput,
	pub bind_host: String,
	pub network_preferred_port: Option<i64>,
	pub database: DatabaseInput,
	pub prelaunch: Option<PrelaunchInput>,
	pub auto_open_browser: bool,
	pub override_ip: Option<String>,
}

// ---------------------------------------------------------------------------
// Construction de la commande PHP artisan serve
// ---------------------------------------------------------------------------

pub fn build_command(cfg: &LaunchConfig, port: u16) -> AppResult<Command> {
	let mut extra_args: Vec<String> = Vec::new();
	if let Some(args) = &cfg.runtime.extra_args {
		if !args.trim().is_empty() {
			extra_args = args.split_whitespace().map(|s| s.to_string()).collect();
		}
	}

	let binary: String = match cfg.runtime.runtime_type.as_str() {
		"system_php" => {
			let info = php_runtime::detect_system_php();
			if !info.found {
				return Err(AppError::Php(info.message));
			}
			"php".into()
		}
		"custom_php" | "phprs_experimental" => {
			let path = cfg
				.runtime
				.binary_path
				.clone()
				.ok_or_else(|| AppError::Php("Aucun chemin d'exécutable PHP configuré.".into()))?;
			let info = php_runtime::validate_custom_php(&path);
			if !info.found {
				return Err(AppError::Php(info.message));
			}
			path
		}
		other => return Err(AppError::Php(format!("Type de runtime inconnu : {other}"))),
	};

	let mut cmd = Command::new(&binary);
	cmd.args(&extra_args);
	cmd.arg("artisan")
		.arg("serve")
		.arg(format!("--host={}", cfg.bind_host))
		.arg(format!("--port={port}"))
		.current_dir(&cfg.laravel_path);

	#[cfg(windows)]
	{
		use std::os::windows::process::CommandExt;
		cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
	}
	Ok(cmd)
}

fn is_pid_alive(pid: u32) -> bool {
	#[cfg(windows)]
	{
		let out = Command::new("tasklist")
			.args(["/FI", &format!("PID eq {pid}"), "/NH"])
			.output();
		if let Ok(out) = out {
			let text = String::from_utf8_lossy(&out.stdout).to_lowercase();
			return text.contains(&pid.to_string());
		}
		false
	}
	#[cfg(not(windows))]
	{
		Command::new("kill")
			.args(["-0", &pid.to_string()])
			.status()
			.map(|s| s.success())
			.unwrap_or(false)
	}
}

// ---------------------------------------------------------------------------
// Flux de lancement (thread dédié)
// ---------------------------------------------------------------------------

pub fn run_launch(app: AppHandle, cfg: LaunchConfig, session_id: i64) {
	log_and_emit(&app, cfg.project_id, Some(session_id), "info", "start", "Démarrage du projet…");

	// 1. Application de pré-lancement
	if let Some(pre) = cfg.prelaunch.clone() {
		if pre.is_enabled && !pre.app_path.trim().is_empty() {
			log_and_emit(&app, cfg.project_id, Some(session_id), "info", "prelaunch", format!("Lancement de l'application préalable : {}", pre.app_path));
			let mut cmd = Command::new(pre.app_path.trim());
			if let Some(args) = &pre.app_args {
				if !args.trim().is_empty() {
					cmd.args(args.split_whitespace());
				}
			}
			#[cfg(windows)]
			{
				use std::os::windows::process::CommandExt;
				cmd.creation_flags(0x0800_0000);
			}
			match cmd.spawn() {
				Ok(_) => {
					let wait = pre.wait_after_launch_ms.unwrap_or(5000).max(0) as u64;
					log_and_emit(&app, cfg.project_id, Some(session_id), "info", "prelaunch", format!("Attente de {wait} ms avant les vérifications…"));
					std::thread::sleep(std::time::Duration::from_millis(wait));
				}
				Err(e) => {
					log_and_emit(&app, cfg.project_id, Some(session_id), "warning", "prelaunch", format!("Impossible de lancer l'application préalable : {e}"));
				}
			}
		}
	}

	// 2. Vérification base de données
	if cfg.database.is_required {
		log_and_emit(&app, cfg.project_id, Some(session_id), "info", "database", format!("Vérification de la base de données ({})…", cfg.database.driver));
		match db_checker::check_database(&cfg.database, &cfg.laravel_path) {
			Ok(()) => {
				log_and_emit(&app, cfg.project_id, Some(session_id), "info", "database", "Base de données OK.");
			}
			Err(e) => {
				let msg = format!("Base de données injoignable : {e}");
				log_and_emit(&app, cfg.project_id, Some(session_id), "error", "database", msg.clone());
				fail_session(&app, &cfg, session_id, Some(msg));
				return;
			}
		}
	} else {
		log_and_emit(&app, cfg.project_id, Some(session_id), "info", "database", "Vérification base de données désactivée (non requise).");
	}

	// 3. Runtime PHP
	log_and_emit(&app, cfg.project_id, Some(session_id), "info", "runtime", format!("Runtime : {}", cfg.runtime.display_name.as_deref().unwrap_or("?")));
	if cfg.runtime.runtime_type == "phprs_experimental" {
		log_and_emit(&app, cfg.project_id, Some(session_id), "warning", "runtime", "phprs est expérimental : le support Laravel n'est pas garanti (statut « planned »).");
	}

	// 4. Choix du port
	let port = port_scanner::find_free_port(cfg.network_preferred_port);
	log_and_emit(&app, cfg.project_id, Some(session_id), "info", "port", format!("Port choisi : {port}"));

	// 5. Lancement du serveur
	let mut cmd = match build_command(&cfg, port) {
		Ok(cmd) => cmd,
		Err(e) => {
			log_and_emit(&app, cfg.project_id, Some(session_id), "error", "runtime", e.to_string());
			fail_session(&app, &cfg, session_id, Some(e.to_string()));
			return;
		}
	};
	cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).stdin(Stdio::null());

	let mut child: Child = match cmd.spawn() {
		Ok(c) => c,
		Err(e) => {
			let msg = format!("Impossible de lancer PHP : {e}");
			log_and_emit(&app, cfg.project_id, Some(session_id), "error", "launch", msg.clone());
			fail_session(&app, &cfg, session_id, Some(msg));
			return;
		}
	};
	let pid = child.id();
	log_and_emit(&app, cfg.project_id, Some(session_id), "info", "launch", format!("Serveur PHP démarré (pid {pid}). Attente de la disponibilité…"));

	{
		let state = app.state::<AppState>();
		state.processes.update(session_id, |r| {
			r.pid = Some(pid);
			r.port = Some(port);
			r.status = "starting".into();
		});
	}
	emit_status(&app, cfg.project_id, session_id, "starting");

	// 6. Lecture de la sortie (stdout/stderr)
	let out_stream = child.stdout.take().map(|s| Box::new(s) as Box<dyn std::io::Read + Send>);
	let err_stream = child.stderr.take().map(|s| Box::new(s) as Box<dyn std::io::Read + Send>);
	let out_reader = spawn_reader(app.clone(), cfg.project_id, session_id, "info", out_stream);
	let err_reader = spawn_reader(app.clone(), cfg.project_id, session_id, "error", err_stream);

	// 7. Attente de la disponibilité du port
	let ready = port_scanner::wait_for_port(port, 30_000);
	if !ready {
		if !is_pid_alive(pid) {
			let msg = "Le serveur Laravel s'est arrêté pendant le démarrage.".to_string();
			log_and_emit(&app, cfg.project_id, Some(session_id), "error", "launch", msg.clone());
			fail_session(&app, &cfg, session_id, Some(msg));
			return;
		}
		log_and_emit(&app, cfg.project_id, Some(session_id), "warning", "launch", "Le serveur tarde à répondre, mais le processus est toujours actif.");
	}

	// 8. Détection IP + URL
	let ip = cfg
		.override_ip
		.clone()
		.or_else(|| network_detector::best_local_ip().map(|i| i.addr));
	let url = ip
		.as_ref()
		.map(|ip| network_detector::build_url(ip, port))
		.unwrap_or_else(|| format!("http://127.0.0.1:{port}"));
	log_and_emit(&app, cfg.project_id, Some(session_id), "info", "network", format!("URL réseau : {url}"));

	{
		let state = app.state::<AppState>();
		state.processes.update(session_id, |r| {
			r.status = "running".into();
			r.url = Some(url.clone());
			r.ip = ip.clone();
		});
		persist_network_state(&state, cfg.project_id, port, ip.clone(), url.clone());
	}
	emit_status(&app, cfg.project_id, session_id, "running");

	// 9. Ouverture navigateur si demandé
	if cfg.auto_open_browser {
		let _ = open::that(&url);
	}

	// 10. Attente de la fin du processus
	let code = match child.wait() {
		Ok(s) => s.code(),
		Err(_) => {
			let _ = out_reader.join();
			let _ = err_reader.join();
			return;
		}
	};
	let _ = out_reader.join();
	let _ = err_reader.join();

	log_and_emit(&app, cfg.project_id, Some(session_id), "info", "stop", format!("Le processus s'est terminé (code {code:?})."));
	finish_session(&app, cfg.project_id, session_id);
}

fn spawn_reader(
	app: AppHandle,
	project_id: i64,
	session_id: i64,
	level: &'static str,
	stream: Option<Box<dyn std::io::Read + Send>>,
) -> std::thread::JoinHandle<()> {
	std::thread::spawn(move || {
		let Some(stream) = stream else { return };
		let reader = BufReader::new(stream);
		for line in reader.lines() {
			let Ok(line) = line else { break };
			let line = line.trim().to_string();
			if line.is_empty() {
				continue;
			}
			log_and_emit(&app, project_id, Some(session_id), level, "process", line);
		}
	})
}

// ---------------------------------------------------------------------------
// Persistance / événements
// ---------------------------------------------------------------------------

pub fn log_and_emit(app: &AppHandle, project_id: i64, session_id: Option<i64>, level: &str, step: &str, message: impl Into<String>) {
	let message = message.into();
	let created_at = now_iso();
	{
		let state = app.state::<AppState>();
		let locked = state.db.lock();
		if let Ok(db) = locked {
			let _ = db.execute(
				"INSERT INTO runtime_logs (session_id, project_id, level, step, message, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
				rusqlite::params![session_id, project_id, level, step, message, created_at],
			);
		}
	}
	let _ = app.emit(
		"laralink://log",
		LogEvent {
			session_id,
			project_id,
			level: level.into(),
			step: step.into(),
			message,
			created_at,
		},
	);
}

fn emit_status(app: &AppHandle, project_id: i64, session_id: i64, status: &str) {
	let runtime = {
		let state = app.state::<AppState>();
		state.processes.get(session_id)
	};
	let evt = StatusEvent {
		project_id,
		status: status.into(),
		pid: runtime.as_ref().and_then(|r| r.pid).map(i64::from),
		port: runtime.as_ref().and_then(|r| r.port).map(i64::from),
		url: runtime.as_ref().and_then(|r| r.url.clone()),
		ip: runtime.as_ref().and_then(|r| r.ip.clone()),
	};
	let _ = app.emit("laralink://status", evt);

	let state = app.state::<AppState>();
	let locked = state.db.lock();
		if let Ok(db) = locked {
		let _ = db.execute(
			"UPDATE projects SET status = ?1, updated_at = ?2 WHERE id = ?3",
			rusqlite::params![status, now_iso(), project_id],
		);
		if status != "starting" {
			let _ = db.execute(
				"UPDATE runtime_sessions SET status = ?1, port = COALESCE(?2, port), url = COALESCE(?3, url), local_ip = COALESCE(?4, local_ip), ended_at = CASE WHEN ?1 IN ('stopped', 'error') THEN ?5 ELSE ended_at END WHERE id = ?6",
				rusqlite::params![
					status,
					runtime.as_ref().and_then(|r| r.port).map(i64::from),
					runtime.as_ref().and_then(|r| r.url.clone()),
					runtime.as_ref().and_then(|r| r.ip.clone()),
					now_iso(),
					session_id
				],
			);
		}
	}
}

fn fail_session(app: &AppHandle, cfg: &LaunchConfig, session_id: i64, error: Option<String>) {
	if let Some(msg) = &error {
		log_and_emit(app, cfg.project_id, Some(session_id), "error", "error", msg.clone());
	}
	{
		let state = app.state::<AppState>();
		let locked = state.db.lock();
		if let Ok(db) = locked {
			let _ = db.execute(
				"UPDATE runtime_sessions SET status = 'error', ended_at = ?1, error_message = ?2 WHERE id = ?3",
				rusqlite::params![now_iso(), error, session_id],
			);
		}
		state.processes.remove(session_id);
	}
	emit_status(app, cfg.project_id, session_id, "error");
}

fn finish_session(app: &AppHandle, project_id: i64, session_id: i64) {
	{
		let state = app.state::<AppState>();
		let locked = state.db.lock();
		if let Ok(db) = locked {
			let _ = db.execute(
				"UPDATE runtime_sessions SET status = 'stopped', ended_at = ?1 WHERE id = ?2 AND status != 'error'",
				rusqlite::params![now_iso(), session_id],
			);
		}
		state.processes.remove(session_id);
	}
	emit_status(app, project_id, session_id, "stopped");
}

fn persist_network_state(
	state: &tauri::State<'_, AppState>,
	project_id: i64,
	port: u16,
	ip: Option<String>,
	url: String,
) {
	let Ok(db) = state.db.lock() else { return };
	let _ = db.execute(
		"UPDATE project_networks SET last_used_port = ?1, last_local_ip = ?2, last_public_url = ?3, updated_at = ?4 WHERE project_id = ?5",
		rusqlite::params![i64::from(port), ip, url, now_iso(), project_id],
	);
}

pub fn stop_project_process(app: &AppHandle, project_id: i64) -> AppResult<()> {
	let sessions: Vec<(i64, Option<u32>)> = {
		let state = app.state::<AppState>();
		state
			.processes
			.running_for_project(project_id)
			.into_iter()
			.map(|r| (r.session_id, r.pid))
			.collect()
	};

	if sessions.is_empty() {
		return Err(AppError::Message("Aucun serveur en cours d'exécution pour ce projet.".into()));
	}

	for (session_id, pid) in sessions {
		if let Some(pid) = pid {
			log_and_emit(app, project_id, Some(session_id), "info", "stop", format!("Arrêt du processus {pid}…"));
			kill_pid(pid);
		}
	}
	Ok(())
}

