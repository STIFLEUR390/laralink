use tauri::State;

use crate::error::AppResult;
use crate::models::PhpInfo;
use crate::services::php_runtime;
use crate::AppState;

#[tauri::command]
pub fn detect_system_php() -> AppResult<PhpInfo> {
	Ok(php_runtime::detect_system_php())
}

#[tauri::command]
pub fn validate_custom_php(path: String) -> AppResult<PhpInfo> {
	Ok(php_runtime::validate_custom_php(&path))
}

/// Teste le runtime actif d'un projet (version PHP résolue).
#[tauri::command]
pub fn test_project_runtime(state: State<'_, AppState>, project_id: i64) -> AppResult<PhpInfo> {
	let conn = state.db.lock().map_err(|_| crate::error::AppError::Message("Verrou DB".into()))?;
	let (runtime_type, binary_path) = conn
		.query_row(
			"SELECT runtime_type, binary_path FROM project_runtimes WHERE project_id = ?1 AND is_active = 1",
			[project_id],
			|r| Ok((r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?)),
		)
		.map_err(|_| crate::error::AppError::Message("Aucun runtime actif pour ce projet.".into()))?;

	let info = match runtime_type.as_str() {
		"custom_php" | "phprs_experimental" => php_runtime::validate_custom_php(binary_path.as_deref().unwrap_or("")),
		_ => php_runtime::detect_system_php(),
	};
	Ok(info)
}
