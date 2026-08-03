use std::collections::HashMap;
use std::process::Command;
use std::sync::Mutex;


/// Informations d'exécution d'une session de lancement, indexées par session id.
#[derive(Debug, Clone)]
pub struct SessionRuntime {
	pub session_id: i64,
	pub project_id: i64,
	pub pid: Option<u32>,
	pub port: Option<u16>,
	pub url: Option<String>,
	pub ip: Option<String>,
	pub status: String,
}

/// Gère les processus lancés (serveurs Laravel) et l'état des sessions.
pub struct ProcessManager {
	pub sessions: Mutex<HashMap<i64, SessionRuntime>>,
}

impl ProcessManager {
	pub fn new() -> Self {
		Self {
			sessions: Mutex::new(HashMap::new()),
		}
	}

	pub fn register(&self, runtime: SessionRuntime) {
		self.sessions
			.lock()
			.map(|mut m| {
				m.insert(runtime.session_id, runtime);
			})
			.ok();
	}

	pub fn get(&self, session_id: i64) -> Option<SessionRuntime> {
		self.sessions
			.lock()
			.ok()
			.and_then(|m| m.get(&session_id).cloned())
	}

	pub fn update<F: FnOnce(&mut SessionRuntime)>(&self, session_id: i64, f: F) {
		if let Ok(mut m) = self.sessions.lock() {
			if let Some(r) = m.get_mut(&session_id) {
				f(r);
			}
		}
	}

	pub fn remove(&self, session_id: i64) -> Option<SessionRuntime> {
		self.sessions.lock().ok().and_then(|mut m| m.remove(&session_id))
	}

	pub fn running_for_project(&self, project_id: i64) -> Vec<SessionRuntime> {
		self.sessions
			.lock()
			.ok()
			.map(|m| {
				m.values()
					.filter(|r| {
						r.project_id == project_id && (r.status == "running" || r.status == "starting")
					})
					.cloned()
					.collect()
			})
			.unwrap_or_default()
	}

	pub fn count_active(&self) -> usize {
		self.sessions
			.lock()
			.ok()
			.map(|m| {
				m.values()
					.filter(|r| r.status == "running" || r.status == "starting")
					.count()
			})
			.unwrap_or(0)
	}
}

/// Tue un processus par PID (arbre complet sur Windows).
pub fn kill_pid(pid: u32) {
	#[cfg(windows)]
	{
		let _ = Command::new("taskkill")
			.args(["/PID", &pid.to_string(), "/T", "/F"])
			.output();
	}
	#[cfg(not(windows))]
	{
		let _ = Command::new("kill").arg("-9").arg(pid.to_string()).output();
	}
}
