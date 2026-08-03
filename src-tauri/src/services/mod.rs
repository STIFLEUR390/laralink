pub mod db_checker;
pub mod laravel_launcher;
pub mod network_detector;
pub mod password;
pub mod php_runtime;
pub mod port_scanner;
pub mod process_manager;

use chrono::Utc;

pub fn now_iso() -> String {
	Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

pub fn slugify(input: &str) -> String {
	let mut out = String::new();
	let mut last_dash = false;
	for c in input.trim().to_lowercase().chars() {
		if c.is_ascii_alphanumeric() {
			out.push(c);
			last_dash = false;
		} else if !last_dash && !out.is_empty() {
			out.push('-');
			last_dash = true;
		}
	}
	while out.ends_with('-') {
		out.pop();
	}
	if out.is_empty() {
		out.push_str("projet");
	}
	out
}

pub fn unique_slug(conn: &rusqlite::Connection, base: &str, exclude_id: Option<i64>) -> AppResult<String> {
	let mut candidate = slugify(base);
	let mut n = 2;
	loop {
		let exists: bool = match exclude_id {
			Some(id) => conn.query_row(
				"SELECT EXISTS(SELECT 1 FROM projects WHERE slug = ?1 AND id != ?2)",
				rusqlite::params![candidate.as_str(), id],
				|row| row.get(0),
			)?,
			None => conn.query_row(
				"SELECT EXISTS(SELECT 1 FROM projects WHERE slug = ?1)",
				[candidate.as_str()],
				|row| row.get(0),
			)?,
		};
		if !exists {
			return Ok(candidate);
		}
		candidate = format!("{base_slug}-{n}", base_slug = slugify(base));
		n += 1;
	}
}

use crate::error::AppResult;
