use rusqlite::{params, Connection, OptionalExtension};
use tauri::State;

use crate::error::{AppError, AppResult};
use crate::models::*;
use crate::services::{now_iso, unique_slug};
use crate::AppState;

// ---------------------------------------------------------------------------
// Lecture
// ---------------------------------------------------------------------------

pub fn load_summaries(conn: &Connection) -> AppResult<Vec<ProjectSummary>> {
	let mut stmt = conn.prepare(
		"SELECT p.id, p.name, p.slug, p.laravel_path, p.description, p.is_default, p.status, p.updated_at
		 FROM projects p ORDER BY p.is_default DESC, p.name COLLATE NOCASE ASC",
	)?;
	let rows = stmt.query_map([], |row| {
		Ok((
			row.get::<_, i64>(0)?,
			row.get::<_, String>(1)?,
			row.get::<_, String>(2)?,
			row.get::<_, String>(3)?,
			row.get::<_, Option<String>>(4)?,
			row.get::<_, i64>(5)? != 0,
			row.get::<_, String>(6)?,
			row.get::<_, String>(7)?,
		))
	})?;

	let mut out = Vec::new();
	for row in rows {
		let (id, name, slug, laravel_path, description, is_default, status, updated_at) = row?;
		let (runtime_type, runtime_label) = conn
			.query_row(
				"SELECT runtime_type, display_name FROM project_runtimes WHERE project_id = ?1 AND is_active = 1",
				[id],
				|r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)),
			)
			.optional()?
			.unwrap_or_else(|| ("none".into(), "Aucun runtime".into()));
		let (last_url, port) = conn
			.query_row(
				"SELECT last_public_url, COALESCE(last_used_port, preferred_port) FROM project_networks WHERE project_id = ?1",
				[id],
				|r| Ok((r.get::<_, Option<String>>(0)?, r.get::<_, Option<i64>>(1)?)),
			)
			.optional()?
			.unwrap_or((None, None));

		out.push(ProjectSummary {
			id,
			name,
			slug,
			laravel_path,
			description,
			is_default,
			status,
			runtime_type,
			runtime_label,
			last_url,
			port,
			updated_at,
		});
	}
	Ok(out)
}

pub fn load_project_detail(conn: &Connection, id: i64) -> AppResult<ProjectDetail> {
	let row = conn
		.query_row(
			"SELECT id, name, slug, laravel_path, description, is_default, auto_start, auto_open_browser, status, created_at, updated_at
			 FROM projects WHERE id = ?1",
			[id],
			|r| {
				Ok(ProjectDetail {
					id: r.get(0)?,
					name: r.get(1)?,
					slug: r.get(2)?,
					laravel_path: r.get(3)?,
					description: r.get(4)?,
					is_default: r.get::<_, i64>(5)? != 0,
					auto_start: r.get::<_, i64>(6)? != 0,
					auto_open_browser: r.get::<_, i64>(7)? != 0,
					status: r.get(8)?,
					created_at: r.get(9)?,
					updated_at: r.get(10)?,
					runtimes: Vec::new(),
					network: NetworkInfo {
						id: 0,
						project_id: id,
						bind_host: "0.0.0.0".into(),
						preferred_port: None,
						last_used_port: None,
						last_local_ip: None,
						last_public_url: None,
					},
					database: DatabaseInfo {
						id: 0,
						project_id: id,
						driver: "mysql".into(),
						host: None,
						port: None,
						database_name: String::new(),
						username: None,
						password: None,
						sqlite_path: None,
						timeout_seconds: 15,
						is_required: true,
					},
					prelaunch: None,
				})
			},
		)
		.optional()?
		.ok_or_else(|| AppError::Message("Projet introuvable.".into()))?;

	let mut detail = row;
	let mut stmt = conn.prepare(
		"SELECT id, project_id, runtime_type, display_name, binary_path, version_label, is_active, extra_args
		 FROM project_runtimes WHERE project_id = ?1 ORDER BY is_active DESC, id ASC",
	)?;
	detail.runtimes = stmt
		.query_map([id], |r| {
			Ok(RuntimeInfo {
				id: r.get(0)?,
				project_id: r.get(1)?,
				runtime_type: r.get(2)?,
				display_name: r.get(3)?,
				binary_path: r.get(4)?,
				version_label: r.get(5)?,
				is_active: r.get::<_, i64>(6)? != 0,
				extra_args: r.get(7)?,
			})
		})?
		.collect::<Result<Vec<_>, _>>()?;

	if let Some(network) = conn
		.query_row(
			"SELECT id, project_id, bind_host, preferred_port, last_used_port, last_local_ip, last_public_url
			 FROM project_networks WHERE project_id = ?1",
			[id],
			|r| {
				Ok(NetworkInfo {
					id: r.get(0)?,
					project_id: r.get(1)?,
					bind_host: r.get(2)?,
					preferred_port: r.get(3)?,
					last_used_port: r.get(4)?,
					last_local_ip: r.get(5)?,
					last_public_url: r.get(6)?,
				})
			},
		)
		.optional()?
	{
		detail.network = network;
	}

	if let Some(database) = conn
		.query_row(
			"SELECT id, project_id, driver, host, port, database_name, username, password, sqlite_path, timeout_seconds, is_required
			 FROM project_databases WHERE project_id = ?1",
			[id],
			|r| {
				Ok(DatabaseInfo {
					id: r.get(0)?,
					project_id: r.get(1)?,
					driver: r.get(2)?,
					host: r.get(3)?,
					port: r.get(4)?,
					database_name: r.get(5)?,
					username: r.get(6)?,
					password: r.get(7)?,
					sqlite_path: r.get(8)?,
					timeout_seconds: r.get(9)?,
					is_required: r.get::<_, i64>(10)? != 0,
				})
			},
		)
		.optional()?
	{
		detail.database = database;
	}

	if let Some(prelaunch) = conn
		.query_row(
			"SELECT id, project_id, app_path, app_args, is_enabled, wait_after_launch_ms
			 FROM project_prelaunch_apps WHERE project_id = ?1",
			[id],
			|r| {
				Ok(PrelaunchInfo {
					id: r.get(0)?,
					project_id: r.get(1)?,
					app_path: r.get(2)?,
					app_args: r.get(3)?,
					is_enabled: r.get::<_, i64>(4)? != 0,
					wait_after_launch_ms: r.get(5)?,
				})
			},
		)
		.optional()?
	{
		detail.prelaunch = Some(prelaunch);
	}

	Ok(detail)
}

fn display_name_for(input: &RuntimeInput) -> String {
	match input.runtime_type.as_str() {
		"system_php" => "PHP système".into(),
		"custom_php" => input
			.binary_path
			.as_deref()
			.map(|p| {
				std::path::Path::new(p)
					.file_name()
					.map(|f| f.to_string_lossy().to_string())
					.unwrap_or_else(|| p.to_string())
			})
			.unwrap_or_else(|| "PHP personnalisé".into()),
		"phprs_experimental" => "phprs (expérimental)".into(),
		_ => "Runtime".into(),
	}
}

fn validate_runtime_input(input: &RuntimeInput) -> AppResult<()> {
	match input.runtime_type.as_str() {
		"custom_php" => {
			let path = input.binary_path.clone().unwrap_or_default();
			let info = crate::services::php_runtime::validate_custom_php(&path);
			if !info.found {
				return Err(AppError::Message(format!(
					"Exécutable PHP invalide : {}",
					info.message
				)));
			}
		}
		"system_php" => {
			// Simple test informatif, ne bloque pas l'enregistrement.
			let _ = crate::services::php_runtime::detect_system_php();
		}
		_ => {}
	}
	Ok(())
}

// ---------------------------------------------------------------------------
// Commandes
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn list_projects(state: State<'_, AppState>) -> AppResult<Vec<ProjectSummary>> {
	let conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
	load_summaries(&conn)
}

#[tauri::command]
pub fn get_project(state: State<'_, AppState>, id: i64) -> AppResult<ProjectDetail> {
	let conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
	load_project_detail(&conn, id)
}

#[tauri::command]
pub fn create_project(state: State<'_, AppState>, input: ProjectInput) -> AppResult<ProjectDetail> {
	validate_project_input(&input)?;
	validate_runtime_input(&input.runtime)?;

	let mut conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
	let tx = conn.transaction()?;
	let now = now_iso();
	let slug = unique_slug(&tx, input.name.as_str(), None)?;

	tx.execute(
		"INSERT INTO projects (name, slug, laravel_path, description, is_default, auto_start, auto_open_browser, status, created_at, updated_at)
		 VALUES (?1, ?2, ?3, ?4, 0, ?5, ?6, 'stopped', ?7, ?7)",
		params![input.name, slug, input.laravel_path, input.description, input.auto_start, input.auto_open_browser, now],
	)?;
	let project_id = tx.last_insert_rowid();

	insert_runtime(&tx, project_id, &input.runtime, true, &now)?;
	tx.execute(
		"INSERT INTO project_networks (project_id, bind_host, preferred_port, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?4)",
		params![project_id, input.network.bind_host.as_deref().unwrap_or("0.0.0.0"), input.network.preferred_port, now],
	)?;
	insert_database(&tx, project_id, &input.database, &now)?;
	if let Some(pre) = &input.prelaunch {
		insert_prelaunch(&tx, project_id, pre, &now)?;
	}
	tx.commit()?;

	load_project_detail(&conn, project_id)
}

#[tauri::command]
pub fn update_project(state: State<'_, AppState>, id: i64, input: ProjectInput) -> AppResult<ProjectDetail> {
	validate_project_input(&input)?;
	validate_runtime_input(&input.runtime)?;

	let mut conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
	let tx = conn.transaction()?;
	let now = now_iso();
	let slug = unique_slug(&tx, input.name.as_str(), Some(id))?;

	tx.execute(
		"UPDATE projects SET name = ?1, slug = ?2, laravel_path = ?3, description = ?4, auto_start = ?5, auto_open_browser = ?6, updated_at = ?7 WHERE id = ?8",
		params![input.name, slug, input.laravel_path, input.description, input.auto_start, input.auto_open_browser, now, id],
	)?;

	// Runtime : on met à jour le runtime actif, ou on en crée un.
	let active_id: Option<i64> = tx
		.query_row(
			"SELECT id FROM project_runtimes WHERE project_id = ?1 AND is_active = 1",
			[id],
			|r| r.get(0),
		)
		.optional()?;
	if let Some(rid) = active_id {
		tx.execute(
			"UPDATE project_runtimes SET runtime_type = ?1, display_name = ?2, binary_path = ?3, version_label = ?4, extra_args = ?5, updated_at = ?6 WHERE id = ?7",
			params![
				input.runtime.runtime_type,
				display_name_for(&input.runtime),
				input.runtime.binary_path,
				input.runtime.version_label,
				input.runtime.extra_args,
				now,
				rid
			],
		)?;
	} else {
		insert_runtime(&tx, id, &input.runtime, true, &now)?;
	}

	tx.execute(
		"UPDATE project_networks SET bind_host = ?1, preferred_port = ?2, updated_at = ?3 WHERE project_id = ?4",
		params![
			input.network.bind_host.as_deref().unwrap_or("0.0.0.0"),
			input.network.preferred_port,
			now,
			id
		],
	)?;
	tx.execute(
		"UPDATE project_databases SET driver = ?1, host = ?2, port = ?3, database_name = ?4, username = ?5, password = ?6, sqlite_path = ?7, timeout_seconds = ?8, is_required = ?9, updated_at = ?10 WHERE project_id = ?11",
		params![
			input.database.driver,
			input.database.host,
			input.database.port,
			input.database.database_name,
			input.database.username,
			input.database.password,
			input.database.sqlite_path,
			input.database.timeout_seconds.unwrap_or(15),
			input.database.is_required,
			now,
			id
		],
	)?;

	// Pré-lancement : upsert ou suppression.
	tx.execute("DELETE FROM project_prelaunch_apps WHERE project_id = ?1", [id])?;
	if let Some(pre) = &input.prelaunch {
		insert_prelaunch(&tx, id, pre, &now)?;
	}

	tx.commit()?;
	load_project_detail(&conn, id)
}

#[tauri::command]
pub fn delete_project(app: tauri::AppHandle, state: State<'_, AppState>, id: i64) -> AppResult<()> {
	let _ = crate::services::laravel_launcher::stop_project_process(&app, id);
	let conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
	let default_check: Option<i64> = conn
		.query_row("SELECT id FROM app_settings WHERE default_project_id = ?1", [id], |r| r.get(0))
		.optional()?;
	if default_check.is_some() {
		conn.execute("UPDATE app_settings SET default_project_id = NULL, updated_at = ?1 WHERE id = 1", [now_iso()])?;
	}
	conn.execute("DELETE FROM projects WHERE id = ?1", [id])?;
	Ok(())
}

#[tauri::command]
pub fn duplicate_project(state: State<'_, AppState>, id: i64) -> AppResult<ProjectDetail> {
	let conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
	let src = load_project_detail(&conn, id)?;
	let now = now_iso();

	let tx = conn.unchecked_transaction()?;
	let slug = unique_slug(&tx, format!("{} copie", src.name).as_str(), None)?;
	tx.execute(
		"INSERT INTO projects (name, slug, laravel_path, description, is_default, auto_start, auto_open_browser, status, created_at, updated_at)
		 VALUES (?1, ?2, ?3, ?4, 0, ?5, ?6, 'stopped', ?7, ?7)",
		params![format!("{} (copie)", src.name), slug, src.laravel_path, src.description, src.auto_start, src.auto_open_browser, now],
	)?;
	let new_id = tx.last_insert_rowid();

	if let Some(rt) = src.runtimes.iter().find(|r| r.is_active) {
		tx.execute(
			"INSERT INTO project_runtimes (project_id, runtime_type, display_name, binary_path, version_label, is_active, extra_args, created_at, updated_at)
			 VALUES (?1, ?2, ?3, ?4, ?5, 1, ?6, ?7, ?7)",
			params![new_id, rt.runtime_type, rt.display_name, rt.binary_path, rt.version_label, rt.extra_args, now],
		)?;
	} else {
		insert_runtime(&tx, new_id, &RuntimeInput {
			runtime_type: "system_php".into(),
			display_name: None,
			binary_path: None,
			version_label: None,
			extra_args: None,
		}, true, &now)?;
	}

	tx.execute(
		"INSERT INTO project_networks (project_id, bind_host, preferred_port, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?4)",
		params![new_id, src.network.bind_host, src.network.preferred_port, now],
	)?;
	tx.execute(
		"INSERT INTO project_databases (project_id, driver, host, port, database_name, username, password, sqlite_path, timeout_seconds, is_required, created_at, updated_at)
		 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?11)",
		params![
			new_id,
			src.database.driver,
			src.database.host,
			src.database.port,
			src.database.database_name,
			src.database.username,
			src.database.password,
			src.database.sqlite_path,
			src.database.timeout_seconds,
			src.database.is_required,
			now
		],
	)?;
	if let Some(pre) = &src.prelaunch {
		tx.execute(
			"INSERT INTO project_prelaunch_apps (project_id, app_path, app_args, is_enabled, wait_after_launch_ms, created_at, updated_at)
			 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6)",
			params![new_id, pre.app_path, pre.app_args, pre.is_enabled, pre.wait_after_launch_ms, now],
		)?;
	}
	tx.commit()?;
	load_project_detail(&conn, new_id)
}

#[tauri::command]
pub fn set_active_project(state: State<'_, AppState>, id: i64) -> AppResult<()> {
	let conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
	let exists: bool = conn
		.query_row("SELECT EXISTS(SELECT 1 FROM projects WHERE id = ?1)", [id], |r| r.get(0))?;
	if !exists {
		return Err(AppError::Message("Projet introuvable.".into()));
	}
	let now = now_iso();
	conn.execute("UPDATE projects SET is_default = 0", [])?;
	conn.execute("UPDATE projects SET is_default = 1, updated_at = ?1 WHERE id = ?2", params![now, id])?;
	Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn validate_project_input(input: &ProjectInput) -> AppResult<()> {
	if input.name.trim().is_empty() {
		return Err(AppError::Message("Le nom du projet est requis.".into()));
	}
	if input.laravel_path.trim().is_empty() {
		return Err(AppError::Message("Le chemin du projet Laravel est requis.".into()));
	}
	if !["system_php", "custom_php", "phprs_experimental"].contains(&input.runtime.runtime_type.as_str()) {
		return Err(AppError::Message("Type de runtime invalide.".into()));
	}
	Ok(())
}

fn insert_runtime(
	tx: &rusqlite::Transaction<'_>,
	project_id: i64,
	input: &RuntimeInput,
	active: bool,
	now: &str,
) -> AppResult<()> {
	let version = match input.runtime_type.as_str() {
		"custom_php" | "phprs_experimental" => input
			.binary_path
			.as_deref()
			.and_then(|p| crate::services::php_runtime::validate_custom_php(p).version),
		_ => crate::services::php_runtime::detect_system_php().version,
	};
	tx.execute(
		"INSERT INTO project_runtimes (project_id, runtime_type, display_name, binary_path, version_label, is_active, extra_args, created_at, updated_at)
		 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)",
		params![
			project_id,
			input.runtime_type,
			display_name_for(input),
			input.binary_path,
			version.or_else(|| input.version_label.clone()),
			active,
			input.extra_args,
			now
		],
	)?;
	Ok(())
}

fn insert_database(tx: &rusqlite::Transaction<'_>, project_id: i64, input: &DatabaseInput, now: &str) -> AppResult<()> {
	tx.execute(
		"INSERT INTO project_databases (project_id, driver, host, port, database_name, username, password, sqlite_path, timeout_seconds, is_required, created_at, updated_at)
		 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?11)",
		params![
			project_id,
			input.driver,
			input.host,
			input.port,
			input.database_name,
			input.username,
			input.password,
			input.sqlite_path,
			input.timeout_seconds.unwrap_or(15),
			input.is_required,
			now
		],
	)?;
	Ok(())
}

fn insert_prelaunch(tx: &rusqlite::Transaction<'_>, project_id: i64, input: &PrelaunchInput, now: &str) -> AppResult<()> {
	tx.execute(
		"INSERT INTO project_prelaunch_apps (project_id, app_path, app_args, is_enabled, wait_after_launch_ms, created_at, updated_at)
		 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6)",
		params![
			project_id,
			input.app_path,
			input.app_args,
			input.is_enabled,
			input.wait_after_launch_ms.unwrap_or(5000),
			now
		],
	)?;
	Ok(())
}
