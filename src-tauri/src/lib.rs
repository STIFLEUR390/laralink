#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod error;
mod models;
mod services;
#[cfg(test)]
mod tests;

use std::sync::Mutex;
use std::time::Instant;

use rusqlite::Connection;
use tauri::Manager;
use tauri_plugin_cli::CliExt;

pub use error::{AppError, AppResult};

/// État global de l'application, partagé entre les commandes.
pub struct AppState {
	pub db: Mutex<Connection>,
	pub processes: services::process_manager::ProcessManager,
	pub attempts: Mutex<Vec<Instant>>,
}

#[cfg(desktop)]
use tauri::{
	menu::{Menu, MenuItem},
	tray::TrayIconBuilder,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.setup(|app| {
			// Base de données SQLite + migrations versionnées.
			let conn = db::open(app.handle())?;
			app.manage(AppState {
				db: Mutex::new(conn),
				processes: services::process_manager::ProcessManager::new(),
				attempts: Mutex::new(Vec::new()),
			});

			// Projet par défaut : auto-démarrage au lancement.
			let auto_launch = {
				let state = app.state::<AppState>();
				let db = state.db.lock().unwrap();
				let (auto, project_id): (i64, Option<i64>) = db
					.query_row(
						"SELECT auto_launch_default_project, default_project_id FROM app_settings WHERE id = 1",
						[],
						|r| Ok((r.get(0)?, r.get(1)?)),
					)
					.unwrap_or((0, None));
				(auto != 0, project_id)
			};

			// CLI : `laralink start <slug>` / `laralink stop <slug>`.
			let cli_action = parse_cli_action(app.handle())?;

			if let Some((action, project_id)) = cli_action {
				let app_handle = app.handle().clone();
				std::thread::spawn(move || {
					std::thread::sleep(std::time::Duration::from_millis(1500));
					let state_app = app_handle.clone();
					let state = state_app.state::<AppState>();
					match action.as_str() {
						"start" => {
							let _ = crate::commands::launcher::start_project(app_handle, state, project_id);
						}
						"stop" => {
							let _ = crate::commands::launcher::stop_project(app_handle, state, project_id);
						}
						_ => {}
					}
				});
			} else if auto_launch.0 {
				if let Some(pid) = auto_launch.1 {
					let app_handle = app.handle().clone();
					std::thread::spawn(move || {
						std::thread::sleep(std::time::Duration::from_millis(1200));
						let state_app = app_handle.clone();
						let state = state_app.state::<AppState>();
						let _ = crate::commands::launcher::start_project(app_handle, state, pid);
					});
				}
			}

			// Tray icon : afficher / quitter.
			#[cfg(desktop)]
			{
				let show_i = MenuItem::with_id(app, "show", "Afficher Laralink", true, None::<&str>)?;
				let quit_i = MenuItem::with_id(app, "quit", "Quitter", true, None::<&str>)?;
				let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

				let _tray = TrayIconBuilder::new()
					.menu(&menu)
					.show_menu_on_left_click(true)
					.icon(app.default_window_icon().unwrap().clone())
					.on_menu_event(|app, event| match event.id.as_ref() {
						"quit" => {
							app.exit(0);
						}
						"show" => {
							if let Some(window) = app.get_webview_window("main") {
								let _ = window.show();
								let _ = window.unminimize();
								let _ = window.set_focus();
							}
						}
						other => {
							println!("menu item {} not handled", other);
						}
					})
					.build(app)?;
			}

			Ok(())
		})
		.plugin(tauri_plugin_dialog::init())
		.plugin(tauri_plugin_clipboard_manager::init())
		.plugin(tauri_plugin_cli::init())
		.invoke_handler(tauri::generate_handler![
			// Projets
			commands::projects::list_projects,
			commands::projects::get_project,
			commands::projects::create_project,
			commands::projects::update_project,
			commands::projects::delete_project,
			commands::projects::duplicate_project,
			commands::projects::set_active_project,
			// Runtime PHP
			commands::runtime::detect_system_php,
			commands::runtime::validate_custom_php,
			commands::runtime::test_project_runtime,
			// Launcher
			commands::launcher::start_project,
			commands::launcher::stop_project,
			commands::launcher::restart_project,
			commands::launcher::get_status,
			commands::launcher::get_sessions,
			commands::launcher::get_logs,
			// Réseau
			commands::network::get_network_info,
			commands::network::get_project_network,
			commands::network::open_url,
			// Sécurité / réglages
			commands::security::get_app_settings,
			commands::security::save_settings,
			commands::security::verify_password,
			commands::security::set_password,
			// Diagnostics
			commands::diagnostics::run_diagnostics,
			commands::diagnostics::get_diagnostics,
			commands::diagnostics::test_database_connection,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

/// Analyse les arguments CLI : `laralink start <slug>` / `laralink stop <slug>`.
/// Retourne (action, project_id) ou None.
fn parse_cli_action(app: &tauri::AppHandle) -> AppResult<Option<(String, i64)>> {
	let matches = match app.cli().matches() {
		Ok(m) => m,
		Err(_) => return Ok(None),
	};

	let (action, slug) = if let Some(sc) = &matches.subcommand {
		let slug = sc
			.matches
			.args
			.get("project")
			.and_then(|a| a.value.as_str())
			.map(|s| s.to_string());
		(sc.name.clone(), slug)
	} else {
		// Pas de sous-commande : arg racine `--project`.
		let slug = matches
			.args
			.get("project")
			.and_then(|a| a.value.as_str())
			.map(|s| s.to_string());
		("start".into(), slug)
	};

	let Some(slug) = slug else { return Ok(None) };
	if !matches!(action.as_str(), "start" | "stop") {
		return Ok(None);
	}

	let state = app.state::<AppState>();
	let db = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
	let project_id = db
		.query_row(
			"SELECT id FROM projects WHERE slug = ?1",
			[&slug],
			|r| r.get::<_, i64>(0),
		)
		.ok();
	Ok(project_id.map(|id| (action, id)))
}
