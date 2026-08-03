use tauri::State;

use crate::error::AppResult;
use crate::models::NetworkInfoResult;
use crate::services::network_detector;
use crate::AppState;

#[tauri::command]
pub fn get_network_info() -> AppResult<NetworkInfoResult> {
	let ips = network_detector::detect_local_ips();
	let selected = ips.first().cloned();
	Ok(NetworkInfoResult { local_ips: ips, selected })
}

#[tauri::command]
pub fn get_project_network(state: State<'_, AppState>, project_id: i64) -> AppResult<crate::models::NetworkInfo> {
	let conn = state.db.lock().map_err(|_| crate::error::AppError::Message("Verrou DB".into()))?;
	let info = conn
		.query_row(
			"SELECT id, project_id, bind_host, preferred_port, last_used_port, last_local_ip, last_public_url
			 FROM project_networks WHERE project_id = ?1",
			[project_id],
			|r| {
				Ok(crate::models::NetworkInfo {
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
		.map_err(|_| crate::error::AppError::Message("Aucune configuration réseau pour ce projet.".into()))?;
	Ok(info)
}

#[tauri::command]
pub fn open_url(url: String) -> AppResult<()> {
	open::that(&url).map_err(|e| crate::error::AppError::Message(format!("Impossible d'ouvrir le navigateur : {e}")))?;
	Ok(())
}
