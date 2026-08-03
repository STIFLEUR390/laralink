use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Entrées (formulaires frontend)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInput {
	pub runtime_type: String,
	pub display_name: Option<String>,
	pub binary_path: Option<String>,
	pub version_label: Option<String>,
	pub extra_args: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInput {
	pub bind_host: Option<String>,
	pub preferred_port: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseInput {
	pub driver: String,
	pub host: Option<String>,
	pub port: Option<i64>,
	pub database_name: String,
	pub username: Option<String>,
	pub password: Option<String>,
	pub sqlite_path: Option<String>,
	pub timeout_seconds: Option<i64>,
	pub is_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrelaunchInput {
	pub app_path: String,
	pub app_args: Option<String>,
	pub is_enabled: bool,
	pub wait_after_launch_ms: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInput {
	pub name: String,
	pub slug: Option<String>,
	pub laravel_path: String,
	pub description: Option<String>,
	pub auto_start: bool,
	pub auto_open_browser: bool,
	pub runtime: RuntimeInput,
	pub network: NetworkInput,
	pub database: DatabaseInput,
	pub prelaunch: Option<PrelaunchInput>,
}

// ---------------------------------------------------------------------------
// Sorties (lecture frontend)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSummary {
	pub id: i64,
	pub name: String,
	pub slug: String,
	pub laravel_path: String,
	pub description: Option<String>,
	pub is_default: bool,
	pub status: String,
	pub runtime_type: String,
	pub runtime_label: String,
	pub last_url: Option<String>,
	pub port: Option<i64>,
	pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInfo {
	pub id: i64,
	pub project_id: i64,
	pub runtime_type: String,
	pub display_name: String,
	pub binary_path: Option<String>,
	pub version_label: Option<String>,
	pub is_active: bool,
	pub extra_args: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInfo {
	pub id: i64,
	pub project_id: i64,
	pub bind_host: String,
	pub preferred_port: Option<i64>,
	pub last_used_port: Option<i64>,
	pub last_local_ip: Option<String>,
	pub last_public_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseInfo {
	pub id: i64,
	pub project_id: i64,
	pub driver: String,
	pub host: Option<String>,
	pub port: Option<i64>,
	pub database_name: String,
	pub username: Option<String>,
	pub password: Option<String>,
	pub sqlite_path: Option<String>,
	pub timeout_seconds: i64,
	pub is_required: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrelaunchInfo {
	pub id: i64,
	pub project_id: i64,
	pub app_path: String,
	pub app_args: Option<String>,
	pub is_enabled: bool,
	pub wait_after_launch_ms: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDetail {
	pub id: i64,
	pub name: String,
	pub slug: String,
	pub laravel_path: String,
	pub description: Option<String>,
	pub is_default: bool,
	pub auto_start: bool,
	pub auto_open_browser: bool,
	pub status: String,
	pub created_at: String,
	pub updated_at: String,
	pub runtimes: Vec<RuntimeInfo>,
	pub network: NetworkInfo,
	pub database: DatabaseInfo,
	pub prelaunch: Option<PrelaunchInfo>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettingsView {
	pub has_password: bool,
	pub theme: String,
	pub language: String,
	pub default_project_id: Option<i64>,
	pub auto_launch_default_project: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhpInfo {
	pub found: bool,
	pub path: String,
	pub version: Option<String>,
	pub message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckResult {
	pub check_type: String,
	pub is_success: bool,
	pub message: String,
	pub checked_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionInfo {
	pub id: i64,
	pub project_id: i64,
	pub runtime_id: Option<i64>,
	pub pid: Option<i64>,
	pub status: String,
	pub started_at: Option<String>,
	pub ended_at: Option<String>,
	pub local_ip: Option<String>,
	pub port: Option<i64>,
	pub url: Option<String>,
	pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
	pub id: i64,
	pub session_id: Option<i64>,
	pub project_id: i64,
	pub level: String,
	pub step: String,
	pub message: String,
	pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalIp {
	pub addr: String,
	pub iface: String,
	pub score: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInfoResult {
	pub local_ips: Vec<LocalIp>,
	pub selected: Option<LocalIp>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusInfo {
	pub project_id: i64,
	pub status: String,
	pub pid: Option<i64>,
	pub port: Option<i64>,
	pub url: Option<String>,
	pub ip: Option<String>,
	pub runtime_type: String,
	pub runtime_label: String,
	pub php_version: Option<String>,
	pub checks: Vec<CheckResult>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyResult {
	pub ok: bool,
	pub locked: bool,
	pub remaining_seconds: i64,
}

// ---------------------------------------------------------------------------
// Événements émis vers le frontend
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusEvent {
	pub project_id: i64,
	pub status: String,
	pub pid: Option<i64>,
	pub port: Option<i64>,
	pub url: Option<String>,
	pub ip: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEvent {
	pub session_id: Option<i64>,
	pub project_id: i64,
	pub level: String,
	pub step: String,
	pub message: String,
	pub created_at: String,
}
