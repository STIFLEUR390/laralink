use std::path::PathBuf;

use rusqlite::Connection;
use tauri::Manager;

use crate::error::AppResult;

pub(crate) const MIGRATIONS: &[(&str, &str)] = &[
	("0001_init", include_str!("../migrations/0001_init.sql")),
	("0002_projects", include_str!("../migrations/0002_projects.sql")),
	("0003_sessions", include_str!("../migrations/0003_sessions.sql")),
];

pub fn db_path(app: &tauri::AppHandle) -> AppResult<PathBuf> {
	let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
	std::fs::create_dir_all(&dir)?;
	Ok(dir.join("laralink.db"))
}

pub fn open(app: &tauri::AppHandle) -> AppResult<Connection> {
	let conn = Connection::open(db_path(app)?)?;
	conn.pragma_update(None, "journal_mode", "WAL")?;
	conn.pragma_update(None, "foreign_keys", "ON")?;
	conn.pragma_update(None, "busy_timeout", "3000")?;
	run_migrations(&conn)?;
	Ok(conn)
}

fn run_migrations(conn: &Connection) -> AppResult<()> {
	conn.execute(
		"CREATE TABLE IF NOT EXISTS schema_migrations (
			version TEXT PRIMARY KEY,
			applied_at TEXT NOT NULL
		)",
		[],
	)?;

	for (version, sql) in MIGRATIONS {
		let applied: bool = conn
			.query_row(
				"SELECT EXISTS(SELECT 1 FROM schema_migrations WHERE version = ?1)",
				[version],
				|row| row.get(0),
			)?;
		if applied {
			continue;
		}
		let tx = conn.unchecked_transaction()?;
		tx.execute_batch(sql)?;
		tx.execute(
			"INSERT INTO schema_migrations (version, applied_at) VALUES (?1, ?2)",
			rusqlite::params![version, crate::services::now_iso()],
		)?;
		tx.commit()?;
	}
	Ok(())
}
