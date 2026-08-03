use std::process::Command;

use crate::models::PhpInfo;

fn parse_version(output: &str) -> Option<String> {
	let re = regex::Regex::new(r"PHP\s+(\d+\.\d+(?:\.\d+)?)").ok()?;
	re.captures(output)
		.and_then(|c| c.get(1))
		.map(|m| m.as_str().to_string())
}

/// Détecte PHP dans le PATH système (`php -v`).
pub fn detect_system_php() -> PhpInfo {
	match run_php_command("php", &[], None, 8000) {
		Ok((version, output)) => PhpInfo {
			found: true,
			path: "php".into(),
			version: version.or_else(|| parse_version(&output)),
			message: "PHP détecté dans le PATH système.".into(),
		},
		Err(e) => PhpInfo {
			found: false,
			path: "php".into(),
			version: None,
			message: format!("PHP introuvable dans le PATH : {e}"),
		},
	}
}

/// Valide un exécutable PHP personnalisé.
pub fn validate_custom_php(path: &str) -> PhpInfo {
	let trimmed = path.trim();
	if trimmed.is_empty() {
		return PhpInfo {
			found: false,
			path: trimmed.into(),
			version: None,
			message: "Chemin vide.".into(),
		};
	}
	if !std::path::Path::new(trimmed).exists() {
		return PhpInfo {
			found: false,
			path: trimmed.into(),
			version: None,
			message: format!("Fichier introuvable : {trimmed}"),
		};
	}
	match run_php_command(trimmed, &[], None, 8000) {
		Ok((version, output)) => PhpInfo {
			found: true,
			path: trimmed.into(),
			version: version.or_else(|| parse_version(&output)),
			message: "Exécutable PHP valide.".into(),
		},
		Err(e) => PhpInfo {
			found: false,
			path: trimmed.into(),
			version: None,
			message: format!("Impossible d'exécuter PHP : {e}"),
		},
	}
}

/// Exécute une commande PHP et retourne (version, sortie).
pub fn run_php_command(
	binary: &str,
	args: &[&str],
	current_dir: Option<&std::path::Path>,
	timeout_ms: u64,
) -> Result<(Option<String>, String), String> {
	let mut cmd = Command::new(binary);
	cmd.args(args);
	if let Some(dir) = current_dir {
		cmd.current_dir(dir);
	}
	#[cfg(windows)]
	{
		// Sur Windows, évite la popup de console.
		use std::os::windows::process::CommandExt;
		cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
	}
	let output = cmd
		.output()
		.map_err(|e| format!("échec d'exécution : {e}"))?;
	let stdout = String::from_utf8_lossy(&output.stdout).to_string();
	let stderr = String::from_utf8_lossy(&output.stderr).to_string();
	let combined = format!("{stdout}\n{stderr}");
	let version = parse_version(&combined);
	if !output.status.success() {
		return Err(format!("commande en échec : {stderr}"));
	}
	let _ = timeout_ms;
	Ok((version, combined))
}
