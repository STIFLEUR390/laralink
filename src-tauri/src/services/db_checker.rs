use std::path::Path;
use std::time::Duration;

use crate::error::{AppError, AppResult};
use crate::models::DatabaseInput;

/// Vérifie que la base de données cible est joignable.
pub fn check_database(input: &DatabaseInput, project_path: &Path) -> AppResult<()> {
	let timeout = input.timeout_seconds.unwrap_or(15).max(1) as u64;

	match input.driver.as_str() {
		"sqlite" => {
			let raw = input
				.sqlite_path
				.clone()
				.or_else(|| Some(input.database_name.clone()))
				.unwrap_or_default();
			let raw = raw.trim();
			if raw.is_empty() {
				return Err(AppError::Message("Aucun chemin SQLite configuré.".into()));
			}
			let path = if Path::new(raw).is_absolute() {
				std::path::PathBuf::from(raw)
			} else {
				project_path.join(raw)
			};
			if !path.exists() {
				return Err(AppError::Message(format!(
					"Fichier SQLite introuvable : {}",
					path.display()
				)));
			}
			// Test d'ouverture réel.
			let conn = rusqlite::Connection::open(&path)?;
			let _ = conn.query_row("SELECT 1", [], |r| r.get::<_, i64>(0))?;
			Ok(())
		}
		"mysql" | "mariadb" => check_mysql(input, timeout),
		"pgsql" => check_pgsql(input, timeout),
		other => Err(AppError::Message(format!("Driver inconnu : {other}"))),
	}
}

fn check_mysql(input: &DatabaseInput, timeout: u64) -> AppResult<()> {
	use mysql::prelude::Queryable;

	let opts = mysql::OptsBuilder::new()
		.ip_or_hostname(Some(input.host.clone().unwrap_or_else(|| "127.0.0.1".into())))
		.tcp_port(input.port.unwrap_or(3306) as u16)
		.user(Some(input.username.clone().unwrap_or_default()))
		.pass(Some(input.password.clone().unwrap_or_default()))
		.db_name(Some(input.database_name.clone()))
		.tcp_connect_timeout(Some(Duration::from_secs(timeout)))
		.read_timeout(Some(Duration::from_secs(timeout)));

	let pool = mysql::Pool::new(opts)
		.map_err(|e| AppError::Message(format!("Connexion MySQL impossible : {e}")))?;
	let mut conn = pool
		.get_conn()
		.map_err(|e| AppError::Message(format!("Connexion MySQL impossible : {e}")))?;
	conn.query_first::<i64, _>("SELECT 1")
		.map_err(|e| AppError::Message(format!("Requête MySQL en échec : {e}")))?;
	Ok(())
}

fn check_pgsql(input: &DatabaseInput, timeout: u64) -> AppResult<()> {
	let mut config = postgres::Config::new();
	config
		.host(&input.host.clone().unwrap_or_else(|| "127.0.0.1".into()))
		.port(input.port.unwrap_or(5432) as u16)
		.user(&input.username.clone().unwrap_or_default())
		.password(&input.password.clone().unwrap_or_default())
		.dbname(&input.database_name)
		.connect_timeout(Duration::from_secs(timeout));

	let mut conn = config
		.connect(postgres::NoTls)
		.map_err(|e| AppError::Message(format!("Connexion PostgreSQL impossible : {e}")))?;
	let _ = conn
		.query_one("SELECT 1", &[])
		.map_err(|e| AppError::Message(format!("Requête PostgreSQL en échec : {e}")))?;
	Ok(())
}
