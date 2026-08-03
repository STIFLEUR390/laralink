use std::path::Path;

use rusqlite::OptionalExtension;
use tauri::{AppHandle, Manager, State};

use crate::error::{AppError, AppResult};
use crate::models::CheckResult;
use crate::services::{db_checker, network_detector, php_runtime, port_scanner, now_iso};
use crate::AppState;

fn run_single_check(
	app: &AppHandle,
	project_id: i64,
	check_type: &str,
	message: String,
	is_success: bool,
) -> AppResult<CheckResult> {
	let checked_at = now_iso();
	{
		let state = app.state::<AppState>();
		let locked = state.db.lock();
		if let Ok(db) = locked {
			let _ = db.execute(
				"INSERT INTO diagnostic_checks (project_id, check_type, is_success, message, checked_at) VALUES (?1, ?2, ?3, ?4, ?5)",
				rusqlite::params![project_id, check_type, is_success, message, checked_at],
			);
		}
	}
	Ok(CheckResult {
		check_type: check_type.into(),
		is_success,
		message,
		checked_at,
	})
}

#[tauri::command]
pub fn run_diagnostics(app: AppHandle, state: State<'_, AppState>, project_id: i64) -> AppResult<Vec<CheckResult>> {
	let (laravel_path, runtime_type, binary_path, driver, database_is_required, preferred_port) = {
		let conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
		let project = conn
			.query_row("SELECT laravel_path FROM projects WHERE id = ?1", [project_id], |r| r.get::<_, String>(0))
			.optional()?
			.ok_or_else(|| AppError::Message("Projet introuvable.".into()))?;
		let (rt, bin): (Option<String>, Option<String>) = conn
			.query_row(
				"SELECT runtime_type, binary_path FROM project_runtimes WHERE project_id = ?1 AND is_active = 1",
				[project_id],
				|r| Ok((r.get(0)?, r.get(1)?)),
			)
			.optional()?
			.unwrap_or((None, None));
		let (db_driver, db_required, port): (Option<String>, Option<i64>, Option<i64>) = conn
			.query_row(
				"SELECT driver, is_required, preferred_port FROM project_databases pd LEFT JOIN project_networks pn ON pn.project_id = pd.project_id WHERE pd.project_id = ?1",
				[project_id],
				|r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
			)
			.optional()?
			.unwrap_or((None, None, None));
		(
			project,
			rt.unwrap_or_else(|| "none".into()),
			bin,
			db_driver.unwrap_or_else(|| "none".into()),
			db_required.map(|v| v != 0).unwrap_or(false),
			port,
		)
	};

	let path = Path::new(&laravel_path);
	let mut results = Vec::new();

	// 1. Chemin Laravel
	if path.exists() && path.is_dir() {
		results.push(run_single_check(&app, project_id, "laravel_path", "Chemin du projet valide.".into(), true)?);
	} else {
		results.push(run_single_check(&app, project_id, "laravel_path", format!("Chemin introuvable : {laravel_path}"), false)?);
	}

	// 2. Fichier artisan
	let artisan = path.join("artisan");
	if artisan.exists() {
		results.push(run_single_check(&app, project_id, "artisan_file", "Fichier artisan présent.".into(), true)?);
	} else {
		results.push(run_single_check(&app, project_id, "artisan_file", "Fichier artisan introuvable (projet Laravel invalide ?)".into(), false)?);
	}

	// 3. Runtime PHP
	let php = match runtime_type.as_str() {
		"custom_php" | "phprs_experimental" => php_runtime::validate_custom_php(binary_path.as_deref().unwrap_or("")),
		_ => php_runtime::detect_system_php(),
	};
	if php.found {
		results.push(run_single_check(
			&app,
			project_id,
			"php_runtime",
			format!("PHP {} ({})", php.version.as_deref().unwrap_or("?"), php.path),
			true,
		)?);
	} else {
		results.push(run_single_check(&app, project_id, "php_runtime", php.message, false)?);
	}

	// 4. Base de données
	if database_is_required && driver != "none" {
		let (host, port, database_name, username, password, sqlite_path, timeout) = {
			let conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
			conn.query_row(
				"SELECT host, port, database_name, username, password, sqlite_path, timeout_seconds FROM project_databases WHERE project_id = ?1",
				[project_id],
				|r| {
					Ok((
						r.get::<_, Option<String>>(0)?,
						r.get::<_, Option<i64>>(1)?,
						r.get::<_, String>(2)?,
						r.get::<_, Option<String>>(3)?,
						r.get::<_, Option<String>>(4)?,
						r.get::<_, Option<String>>(5)?,
						r.get::<_, i64>(6)?,
					))
				},
			)?
		};
		let input = crate::models::DatabaseInput {
			driver: driver.clone(),
			host,
			port,
			database_name,
			username,
			password,
			sqlite_path,
			timeout_seconds: Some(timeout),
			is_required: true,
		};
		match db_checker::check_database(&input, path) {
			Ok(()) => {
				results.push(run_single_check(&app, project_id, "database", "Base de données joignable.".into(), true)?);
			}
			Err(e) => {
				results.push(run_single_check(&app, project_id, "database", format!("Base de données injoignable : {e}"), false)?);
			}
		}
	} else {
		results.push(run_single_check(&app, project_id, "database", "Vérification base de données désactivée.".into(), true)?);
	}

	// 5. Port
	match preferred_port {
		Some(p) if p > 0 && p <= 65535 => {
			if port_scanner::is_port_free(p as u16) {
				results.push(run_single_check(&app, project_id, "port", format!("Port {p} libre."), true)?);
			} else {
				results.push(run_single_check(&app, project_id, "port", format!("Port {p} occupé — un autre port sera utilisé."), false)?);
			}
		}
		_ => {
			results.push(run_single_check(&app, project_id, "port", "Port automatique (aucun port préféré).".into(), true)?);
		}
	}

	// 6. Réseau
	let ips = network_detector::detect_local_ips();
	if let Some(ip) = ips.first() {
		results.push(run_single_check(
			&app,
			project_id,
			"network",
			format!("Adresse LAN détectée : {} ({})", ip.addr, ip.iface),
			true,
		)?);
	} else {
		results.push(run_single_check(&app, project_id, "network", "Aucune adresse IPv4 locale détectée.".into(), false)?);
	}

	Ok(results)
}

#[tauri::command]
pub fn get_diagnostics(state: State<'_, AppState>, project_id: i64, limit: Option<i64>) -> AppResult<Vec<CheckResult>> {
	let conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
	let limit = limit.unwrap_or(12).max(1).min(200);
	let mut stmt = conn.prepare(
		"SELECT check_type, is_success, message, checked_at FROM diagnostic_checks
		 WHERE project_id = ?1 ORDER BY id DESC LIMIT ?2",
	)?;
	let rows = stmt.query_map(rusqlite::params![project_id, limit], |r| {
		Ok(CheckResult {
			check_type: r.get(0)?,
			is_success: r.get::<_, i64>(1)? != 0,
			message: r.get(2)?,
			checked_at: r.get(3)?,
		})
	})?;
	Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestDbResult {
	pub ok: bool,
	pub message: String,
}

/// Teste une configuration base de données sans l'enregistrer.
#[tauri::command]
pub fn test_database_connection(input: crate::models::DatabaseInput) -> AppResult<TestDbResult> {
	let path = Path::new(".");
	match db_checker::check_database(&input, path) {
		Ok(()) => Ok(TestDbResult {
			ok: true,
			message: "Connexion réussie.".into(),
		}),
		Err(e) => Ok(TestDbResult {
			ok: false,
			message: format!("Échec de la connexion : {e}"),
		}),
	}
}
