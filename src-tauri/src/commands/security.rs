use std::time::{Duration, Instant};

use tauri::State;

use crate::error::{AppError, AppResult};
use crate::models::{AppSettingsView, VerifyResult};
use crate::services::{now_iso, password};
use crate::AppState;

const MAX_ATTEMPTS: usize = 5;
const LOCK_WINDOW_MS: u64 = 30_000;
const ATTEMPT_WINDOW_MS: u64 = 600_000;

fn load_settings_view(conn: &rusqlite::Connection) -> AppResult<AppSettingsView> {
	let row = conn.query_row(
		"SELECT settings_password_hash, theme, language, default_project_id, auto_launch_default_project FROM app_settings WHERE id = 1",
		[],
		|r| {
			Ok((
				r.get::<_, Option<String>>(0)?,
				r.get::<_, String>(1)?,
				r.get::<_, String>(2)?,
				r.get::<_, Option<i64>>(3)?,
				r.get::<_, i64>(4)? != 0,
			))
		},
	)?;
	Ok(AppSettingsView {
		has_password: row.0.as_deref().map(|h| !h.is_empty()).unwrap_or(false),
		theme: row.1,
		language: row.2,
		default_project_id: row.3,
		auto_launch_default_project: row.4,
	})
}

#[tauri::command]
pub fn get_app_settings(state: State<'_, AppState>) -> AppResult<AppSettingsView> {
	let conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
	load_settings_view(&conn)
}

#[tauri::command]
pub fn save_settings(
	state: State<'_, AppState>,
	theme: Option<String>,
	language: Option<String>,
	default_project_id: Option<i64>,
	auto_launch_default_project: Option<bool>,
) -> AppResult<AppSettingsView> {
	let conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
	let current = load_settings_view(&conn)?;
	conn.execute(
		"UPDATE app_settings SET theme = ?1, language = ?2, default_project_id = ?3, auto_launch_default_project = ?4, updated_at = ?5 WHERE id = 1",
		rusqlite::params![
			theme.unwrap_or(current.theme),
			language.unwrap_or(current.language),
			default_project_id.or(current.default_project_id),
			auto_launch_default_project.unwrap_or(current.auto_launch_default_project) as i64,
			now_iso()
		],
	)?;
	load_settings_view(&conn)
}

#[tauri::command]
pub fn verify_password(state: State<'_, AppState>, password: String) -> AppResult<VerifyResult> {
	let conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
	let hash: Option<String> = conn.query_row(
		"SELECT settings_password_hash FROM app_settings WHERE id = 1",
		[],
		|r| r.get(0),
	)?;
	drop(conn);

	let mut attempts = state.attempts.lock().map_err(|_| AppError::Message("Verrou tentatives".into()))?;
	attempts.retain(|t| t.elapsed() < Duration::from_millis(ATTEMPT_WINDOW_MS));

	if attempts.len() >= MAX_ATTEMPTS {
		let window_start = attempts[attempts.len() - MAX_ATTEMPTS];
		let remaining = LOCK_WINDOW_MS.saturating_sub(window_start.elapsed().as_millis() as u64);
		return Ok(VerifyResult {
			ok: false,
			locked: remaining > 0,
			remaining_seconds: (remaining / 1000) as i64,
		});
	}

	let Some(hash) = hash else {
		// Pas de mot de passe configuré : accès libre.
		return Ok(VerifyResult { ok: true, locked: false, remaining_seconds: 0 });
	};

	let valid = password::verify_password(&password, &hash)?;
	if valid {
		attempts.clear();
		Ok(VerifyResult { ok: true, locked: false, remaining_seconds: 0 })
	} else {
		attempts.push(Instant::now());
		Ok(VerifyResult { ok: false, locked: false, remaining_seconds: 0 })
	}
}

#[tauri::command]
pub fn set_password(state: State<'_, AppState>, current_password: Option<String>, new_password: String) -> AppResult<()> {
	let conn = state.db.lock().map_err(|_| AppError::Message("Verrou DB".into()))?;
	let hash: Option<String> = conn.query_row(
		"SELECT settings_password_hash FROM app_settings WHERE id = 1",
		[],
		|r| r.get(0),
	)?;

	if let Some(hash) = hash {
		if hash.is_empty() {
			// aucun mot de passe défini
		} else {
			let current = current_password.unwrap_or_default();
			if !password::verify_password(&current, &hash)? {
				return Err(AppError::Security("Mot de passe actuel incorrect.".into()));
			}
		}
	}

	let new_hash = if new_password.trim().is_empty() {
		// Champ vide : suppression de la protection.
		None
	} else {
		Some(password::hash_password(&new_password)?)
	};
	conn.execute(
		"UPDATE app_settings SET settings_password_hash = ?1, updated_at = ?2 WHERE id = 1",
		rusqlite::params![new_hash, now_iso()],
	)?;
	Ok(())
}
