use std::net::TcpListener;

pub fn is_port_free(port: u16) -> bool {
	TcpListener::bind(("0.0.0.0", port)).is_ok()
}

/// Choisit un port libre : préféré → plage 8000..9100 → port éphémère.
pub fn find_free_port(preferred: Option<i64>) -> u16 {
	if let Some(p) = preferred {
		if p > 0 && p <= 65535 {
			let p = p as u16;
			if is_port_free(p) {
				return p;
			}
		}
	}
	for p in 8000..=9100 {
		if is_port_free(p) {
			return p;
		}
	}
	// Port éphémère attribué par l'OS.
	TcpListener::bind(("0.0.0.0", 0))
		.and_then(|l| l.local_addr())
		.map(|a| a.port())
		.unwrap_or(8000)
}

/// Attend qu'un port accepte des connexions TCP (serveur prêt).
pub fn wait_for_port(port: u16, timeout_ms: u64) -> bool {
	let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms);
	while std::time::Instant::now() < deadline {
		if std::net::TcpStream::connect_timeout(
			&std::net::SocketAddr::from(([127, 0, 0, 1], port)),
			std::time::Duration::from_millis(300),
		)
		.is_ok()
		{
			return true;
		}
		std::thread::sleep(std::time::Duration::from_millis(400));
	}
	false
}
