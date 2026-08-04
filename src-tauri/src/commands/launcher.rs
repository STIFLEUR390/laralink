use tauri::{AppHandle, State};
use rusqlite::OptionalExtension;

use crate::error::{AppError, AppResult};
use crate::models::*;
use crate::services::laravel_launcher::{self, LaunchConfig};
use crate::services::process_manager::SessionRuntime;
use crate::services::now_iso;
use crate::AppState;

fn load_launch_config(conn: &rusqlite::Connection, project_id: i64) -> AppResult<LaunchConfig> {
	let project = conn
		.query_row(
			"SELECT name, laravel_path, auto_open_browser FROM projects WHERE id = ?1",
			[project_id],
			|r| {
				Ok((
					r.get::<_, String>(0)?,
					r.get::<_, String>(1)?,
					r.get::<_, i64>(2)? != 0,
				))
			},
		)
		.map_err(|_| AppError::Message("Projet introuvable.".into()))?;

	let runtime = conn
		.query_row(
			"SELECT runtime_type, display_name, binary_path, version_label, extra_args
			 FROM project_runtimes WHERE project_id = ?1 AND is_active = 1",
			[project_id],
			|r| {
				Ok(RuntimeInput {
					runtime_type: r.get(0)?,
					display_name: Some(r.get(1)?),
					binary_path: r.get(2)?,
					version_label: r.get(3)?,
					extra_args: r.get(4)?,
				})
			},
		)
		.optional()?
		.ok_or_else(|| AppError::Message("Aucun runtime actif pour ce projet.".into()))?;

	let (bind_host, preferred_port) = conn
		.query_row(
			"SELECT bind_host, preferred_port FROM project_networks WHERE project_id = ?1",
			[project_id],
			|r| Ok((r.get::<_, String>(0)?, r.get::<_, Option<i64>>(1)?)),
		)
		.optional()?
		.unwrap_or(("0.0.0.0".into(), None));

	let database = conn
		.query_row(
			"SELECT driver, host, port, database_name, username, password, sqlite_path, timeout_seconds, is_required
			 FROM project_databases WHERE project_id = ?1",
			[project_id],
			|r| {
				Ok(DatabaseInput {
					driver: r.get(0)?,
					host: r.get(1)?,
					port: r.get(2)?,
					database_name: r.get(3)?,
					username: r.get(4)?,
					password: r.get(5)?,
					sqlite_path: r.get(6)?,
					timeout_seconds: Some(r.get(7)?),
					is_required: r.get::<_, i64>(8)? != 0,
				})
			},
		)
		.optional()?;

	let prelaunch = conn
		.query_row(
			"SELECT app_path, app_args, is_enabled, wait_after_launch_ms
			 FROM project_prelaunch_apps WHERE project_id = ?1",
			[project_id],
			|r| {
				Ok(PrelaunchInput {
					app_path: r.get(0)?,
					app_args: r.get(1)?,
					is_enabled: r.get::<_, i64>(2)? != 0,
					wait_after_launch_ms: Some(r.get(3)?),
				})
			},
		)
		.optional()?;

	let (name, laravel_path, auto_open_browser) = project;
	Ok(LaunchConfig {
		project_id,
		project_name: name,
		laravel_path: std::path::PathBuf::from(laravel_path),
		runtime,
		bind_host,
		network_preferred_port: preferred_port,
		database: database.unwrap_or(DatabaseInput {
			driver: "sqlite".into(),
			host: None,
			port: None,
			database_name: "database/database.sqlite".into(),
			username: None,
			password: None,
			sqlite_path: None,
			timeout_seconds: Some(15),
			is_required: false,
		}),
		prelaunch,
		auto_open_browser,
		override_ip: None,
	})
}

fn create_session(conn: &rusqlite::Connection, project_id: i64, runtime_id: Option<i64>) -> AppResult<i64> {
	conn.execute(
		"INSERT INTO runtime_sessions (project_id, runtime_id, status, started_at, created_at) VALUES (?1, ?2, 'starting', ?3, ?3)",
		rusqlite::params![project_id, runtime_id, now_iso()],
	)?;
	Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn start_project(app: AppHandle, state: State<'_, AppState>, project_id: i64) -> AppResult<SessionInfo> {
	// Un seul projet actif en v1 : on arrête les autres.
	let running_others: Vec<i64> = {
		let pm = &state.processes;
		let lock = pm.sessions.lock().map_err(|_| AppError::Message("Verrou processus".into()))?;
		lock.values()
			.filter(|r| r.project_id != project_id && (r.status == "running" || r.status == "starting"))
			.map(|r| r.project_id)
			.collect()
	};
	for other in running_others {
		let _ = laravel_launcher::stop_project_process(&app, other);
	}

	let (cfg, runtime_id, session_id) = {
		let conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
		let cfg = load_launch_config(&conn, project_id)?;
		let runtime_id = conn
			.query_row(
				"SELECT id FROM project_runtimes WHERE project_id = ?1 AND is_active = 1",
				[project_id],
				|r| r.get(0),
			)
			.ok();
		let session_id = create_session(&conn, project_id, runtime_id)?;
		(cfg, runtime_id, session_id)
	};

	state.processes.register(SessionRuntime {
		session_id,
		project_id,
		pid: None,
		port: None,
		url: None,
		ip: None,
		status: "starting".into(),
	});

	let app2 = app.clone();
	std::thread::spawn(move || {
		laravel_launcher::run_launch(app2, cfg, session_id);
	});

	Ok(SessionInfo {
		id: session_id,
		project_id,
		runtime_id,
		pid: None,
		status: "starting".into(),
		started_at: Some(now_iso()),
		ended_at: None,
		local_ip: None,
		port: None,
		url: None,
		error_message: None,
	})
}

#[tauri::command]
pub fn stop_project(app: AppHandle, project_id: i64) -> AppResult<()> {
	match laravel_launcher::stop_project_process(&app, project_id) {
		Ok(()) => Ok(()),
		Err(_) => {
			// Aucun processus actif : on remet simplement l'état à jour.
			laravel_launcher::mark_project_stopped(&app, project_id);
			Ok(())
		}
	}
}

#[tauri::command]
pub fn restart_project(app: AppHandle, state: State<'_, AppState>, project_id: i64) -> AppResult<SessionInfo> {
	let _ = laravel_launcher::stop_project_process(&app, project_id);
	// Laisse le temps au port de se libérer.
	std::thread::sleep(std::time::Duration::from_millis(600));
	start_project(app, state, project_id)
}

#[tauri::command]
pub fn get_status(state: State<'_, AppState>, project_id: i64) -> AppResult<StatusInfo> {
	let conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;

	let (status, runtime_type, runtime_label, version_label): (String, String, String, Option<String>) = conn
		.query_row(
			"SELECT p.status, COALESCE(pr.runtime_type, 'none'), COALESCE(pr.display_name, 'Aucun runtime'), pr.version_label
			 FROM projects p
			 LEFT JOIN project_runtimes pr ON pr.project_id = p.id AND pr.is_active = 1
			 WHERE p.id = ?1",
			[project_id],
			|r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
		)
		.optional()?
		.ok_or_else(|| AppError::Message("Projet introuvable.".into()))?;

	let (pid, port, url, ip): (Option<i64>, Option<i64>, Option<String>, Option<String>) = {
		let lock = state.processes.sessions.lock().map_err(|_| AppError::Message("Verrou processus".into()))?;
		lock.values()
			.find(|r| r.project_id == project_id && (r.status == "running" || r.status == "starting"))
			.map(|r| (r.pid.map(i64::from), r.port.map(i64::from), r.url.clone(), r.ip.clone()))
			.unwrap_or((None, None, None, None))
	};

	let checks: Vec<CheckResult> = conn
		.prepare(
			"SELECT check_type, is_success, message, checked_at FROM diagnostic_checks
			 WHERE project_id = ?1
			 AND id IN (SELECT MAX(id) FROM diagnostic_checks GROUP BY check_type)
			 ORDER BY id",
		)?
		.query_map([project_id], |r| {
			Ok(CheckResult {
				check_type: r.get(0)?,
				is_success: r.get::<_, i64>(1)? != 0,
				message: r.get(2)?,
				checked_at: r.get(3)?,
			})
		})?
		.collect::<Result<Vec<_>, _>>()?;

	Ok(StatusInfo {
		project_id,
		status,
		pid,
		port,
		url,
		ip,
		runtime_type,
		runtime_label,
		php_version: version_label,
		checks,
	})
}

#[tauri::command]
pub fn get_sessions(state: State<'_, AppState>, project_id: i64, limit: Option<i64>) -> AppResult<Vec<SessionInfo>> {
	let conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
	let limit = limit.unwrap_or(20).max(1).min(200);
	let mut stmt = conn.prepare(
		"SELECT id, project_id, runtime_id, pid, status, started_at, ended_at, local_ip, port, url, error_message
		 FROM runtime_sessions WHERE project_id = ?1 ORDER BY id DESC LIMIT ?2",
	)?;
	let rows = stmt.query_map(rusqlite::params![project_id, limit], |r| {
		Ok(SessionInfo {
			id: r.get(0)?,
			project_id: r.get(1)?,
			runtime_id: r.get(2)?,
			pid: r.get(3)?,
			status: r.get(4)?,
			started_at: r.get(5)?,
			ended_at: r.get(6)?,
			local_ip: r.get(7)?,
			port: r.get(8)?,
			url: r.get(9)?,
			error_message: r.get(10)?,
		})
	})?;
	Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

#[tauri::command]
pub fn get_logs(
	state: State<'_, AppState>,
	project_id: i64,
	session_id: Option<i64>,
	limit: Option<i64>,
) -> AppResult<Vec<LogEntry>> {
	let conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
	let limit = limit.unwrap_or(200).max(1).min(1000);
	let mut stmt = conn.prepare(
		"SELECT id, session_id, project_id, level, step, message, created_at FROM runtime_logs
		 WHERE project_id = ?1 AND (?2 IS NULL OR session_id = ?2)
		 ORDER BY id DESC LIMIT ?3",
	)?;
	let rows = stmt.query_map(rusqlite::params![project_id, session_id, limit], |r| {
		Ok(LogEntry {
			id: r.get(0)?,
			session_id: r.get(1)?,
			project_id: r.get(2)?,
			level: r.get(3)?,
			step: r.get(4)?,
			message: r.get(5)?,
			created_at: r.get(6)?,
		})
	})?;
	let mut out: Vec<LogEntry> = rows.collect::<Result<Vec<_>, _>>()?;
	out.reverse();
	Ok(out)
}
