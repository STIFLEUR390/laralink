//! Tests unitaires des services et de la couche données (hors Tauri).

use std::path::Path;

use rusqlite::Connection;

use crate::db::MIGRATIONS;
use crate::services::{password, port_scanner, slugify, unique_slug};

fn test_conn() -> Connection {
	let conn = Connection::open_in_memory().unwrap();
	conn.pragma_update(None, "foreign_keys", "ON").unwrap();
	for (_, sql) in MIGRATIONS {
		conn.execute_batch(sql).unwrap();
	}
	conn
}

#[test]
fn migrations_apply_and_idempotent() {
	let conn = test_conn();
	for table in [
		"app_settings",
		"projects",
		"project_runtimes",
		"project_networks",
		"project_databases",
		"project_prelaunch_apps",
		"runtime_sessions",
		"runtime_logs",
		"diagnostic_checks",
	] {
		let count: i64 = conn
			.query_row(
				"SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
				[table],
				|r| r.get(0),
			)
			.unwrap();
		assert_eq!(count, 1, "table {table} manquante");
	}
	let n: i64 = conn
		.query_row("SELECT COUNT(*) FROM app_settings", [], |r| r.get(0))
		.unwrap();
	assert_eq!(n, 1);
}

#[test]
fn project_crud_cascade() {
	let conn = test_conn();
	let now = crate::services::now_iso();
	conn.execute(
		"INSERT INTO projects (name, slug, laravel_path, is_default, created_at, updated_at) VALUES ('Demo', 'demo', '/tmp/demo', 1, ?1, ?1)",
		[&now],
	)
	.unwrap();
	let pid = conn.last_insert_rowid();
	conn.execute(
		"INSERT INTO project_runtimes (project_id, runtime_type, display_name, is_active, created_at, updated_at) VALUES (?1, 'system_php', 'PHP système', 1, ?2, ?2)",
		rusqlite::params![pid, now],
	)
	.unwrap();
	conn.execute(
		"INSERT INTO project_networks (project_id, bind_host, preferred_port, created_at, updated_at) VALUES (?1, '0.0.0.0', 8000, ?2, ?2)",
		rusqlite::params![pid, now],
	)
	.unwrap();

	conn.execute("DELETE FROM projects WHERE id = ?1", [pid]).unwrap();
	let runtimes: i64 = conn
		.query_row("SELECT COUNT(*) FROM project_runtimes WHERE project_id = ?1", [pid], |r| r.get(0))
		.unwrap();
	let networks: i64 = conn
		.query_row("SELECT COUNT(*) FROM project_networks WHERE project_id = ?1", [pid], |r| r.get(0))
		.unwrap();
	assert_eq!(runtimes, 0);
	assert_eq!(networks, 0);
}

#[test]
fn slugify_works() {
	assert_eq!(slugify("Mon Projet Laravel"), "mon-projet-laravel");
	assert_eq!(slugify("!!!"), "projet");
	assert_eq!(slugify("Déjà Vu!"), "d-j-vu");
}

#[test]
fn unique_slug_avoids_collisions() {
	let conn = test_conn();
	let now = crate::services::now_iso();
	let s1 = unique_slug(&conn, "Mon Projet", None).unwrap();
	assert_eq!(s1, "mon-projet");
	conn.execute(
		"INSERT INTO projects (name, slug, laravel_path, created_at, updated_at) VALUES ('Mon Projet', ?1, '/tmp/x', ?2, ?2)",
		rusqlite::params![s1, now],
	)
	.unwrap();
	let s2 = unique_slug(&conn, "Mon Projet", None).unwrap();
	assert_eq!(s2, "mon-projet-2");
	let pid = conn.last_insert_rowid();
	let s3 = unique_slug(&conn, "Mon Projet", Some(pid)).unwrap();
	assert_eq!(s3, "mon-projet");
}

#[test]
fn password_hash_roundtrip() {
	let hash = password::hash_password("secret42").unwrap();
	assert!(hash.starts_with("$argon2"));
	assert!(password::verify_password("secret42", &hash).unwrap());
	assert!(!password::verify_password("wrong", &hash).unwrap());
	assert!(password::hash_password("ab").is_err());
}

#[test]
fn port_scanner_finds_port() {
	let p = port_scanner::find_free_port(Some(8123));
	assert!(p > 0);
	assert!(port_scanner::is_port_free(p));
	let listener = std::net::TcpListener::bind(("0.0.0.0", 0)).unwrap();
	let used = listener.local_addr().unwrap().port();
	assert!(!port_scanner::is_port_free(used));
	drop(listener);
}

#[test]
fn network_detector_finds_ip() {
	let ips = crate::services::network_detector::detect_local_ips();
	assert!(!ips.is_empty(), "aucune IP locale détectée");
	for ip in &ips {
		assert!(!ip.addr.starts_with("127."));
		assert!(!ip.addr.starts_with("169.254."));
	}
}

#[test]
fn db_checker_sqlite_file() {
	let dir = std::env::temp_dir().join(format!("laralink-test-{}", std::process::id()));
	std::fs::create_dir_all(&dir).unwrap();
	let dbfile = dir.join("test.sqlite");
	let conn = rusqlite::Connection::open(&dbfile).unwrap();
	conn.execute("CREATE TABLE t (id INTEGER)", []).unwrap();
	drop(conn);

	let input = crate::models::DatabaseInput {
		driver: "sqlite".into(),
		host: None,
		port: None,
		database_name: "test.sqlite".into(),
		username: None,
		password: None,
		sqlite_path: Some(dbfile.to_string_lossy().into_owned()),
		timeout_seconds: Some(5),
		is_required: true,
	};
	assert!(crate::services::db_checker::check_database(&input, Path::new(&dir)).is_ok());

	let bad = crate::models::DatabaseInput {
		sqlite_path: Some(dir.join("absent.sqlite").to_string_lossy().into_owned()),
		..input
	};
	assert!(crate::services::db_checker::check_database(&bad, Path::new(&dir)).is_err());

	let _ = std::fs::remove_dir_all(&dir);
}

/// Test de bout en bout du flux de lancement (exige php + projet Laravel).
/// Lancer avec : cargo test -- --ignored --nocapture
#[test]
#[ignore]
fn launch_flow_e2e() {
	use crate::models::{DatabaseInput, PrelaunchInput, RuntimeInput};
	use crate::services::laravel_launcher::build_command;
	use crate::services::php_runtime;
	use crate::services::port_scanner::{find_free_port, wait_for_port};
	use crate::services::process_manager::kill_pid;
	use std::io::Read;
	use std::process::Stdio;

	// Prérequis
	let info = php_runtime::detect_system_php();
	assert!(info.found, "PHP requis : {}", info.message);
	let project = std::env::var("LARALINK_E2E_PROJECT").unwrap_or_else(|_| "/tmp/laralink-e2e".into());
	assert!(std::path::Path::new(&project).join("artisan").exists(), "artisan manquant dans {project}");

	let cfg = crate::services::laravel_launcher::LaunchConfig {
		project_id: 1,
		project_name: "E2E".into(),
		laravel_path: std::path::PathBuf::from(&project),
		runtime: RuntimeInput {
			runtime_type: "system_php".into(),
			display_name: Some("PHP système".into()),
			binary_path: None,
			version_label: Some(info.version.clone().unwrap_or_default()),
			extra_args: None,
		},
		bind_host: "127.0.0.1".into(),
		network_preferred_port: Some(8199),
		database: DatabaseInput {
			driver: "sqlite".into(),
			host: None,
			port: None,
			database_name: "database/database.sqlite".into(),
			username: None,
			password: None,
			sqlite_path: None,
			timeout_seconds: Some(5),
			is_required: false,
		},
		prelaunch: Some(PrelaunchInput {
			app_path: String::new(),
			app_args: None,
			is_enabled: false,
			wait_after_launch_ms: Some(0),
		}),
		auto_open_browser: false,
		override_ip: None,
	};

	let port = find_free_port(Some(8199));
	let mut cmd = build_command(&cfg, port).expect("construction commande");
	cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).stdin(Stdio::null());
	let mut child = cmd.spawn().expect("spawn php artisan serve");
	let pid = child.id();
	println!("serveur démarré pid={pid} port={port}");

	let ready = wait_for_port(port, 25_000);
	assert!(ready, "le serveur n'a pas répondu sur le port {port}");

	// Vérifie une réponse HTTP.
	let response = std::net::TcpStream::connect(("127.0.0.1", port)).expect("connexion tcp");
	let mut response = response;
	let _ = std::io::Write::write_all(&mut response, b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
	let mut buf = String::new();
	let _ = response.read_to_string(&mut buf);
	assert!(buf.starts_with("HTTP/1.1 200") || buf.contains("HTTP/1.0 200") || buf.contains(" 200 "), "réponse HTTP inattendue : {buf:?}");

	kill_pid(pid);
	let _ = child.wait();
	println!("serveur arrêté proprement");
}
