use std::net::Ipv4Addr;

use crate::models::LocalIp;

/// Score une adresse IPv4 : plus le score est élevé, plus elle est pertinente
/// pour un partage LAN (adresse privée, interface réelle).
fn score_ip(ip: Ipv4Addr, iface: &str) -> i32 {
	let name = iface.to_lowercase();
	if name.contains("docker") || name.contains("veth") || name.contains("virbr") || name.contains("vmnet") || name.contains("tailscale") || name.contains("utun") || name.contains("wsl") {
		return -10;
	}
	let octets = ip.octets();
	match octets[0] {
		192 if octets[1] == 168 => 30,
		10 => 25,
		172 if (16..=31).contains(&octets[1]) => 20,
		_ => 5,
	}
}

pub fn detect_local_ips() -> Vec<LocalIp> {
	let mut ips: Vec<LocalIp> = Vec::new();
	if let Ok(interfaces) = get_if_addrs::get_if_addrs() {
		for iface in interfaces {
			if let get_if_addrs::IfAddr::V4(v4) = iface.addr {
				let ip = v4.ip;
				if ip.is_loopback() || ip.is_unspecified() || ip.is_link_local() || ip.is_multicast() {
					continue;
				}
				let score = score_ip(ip, &iface.name);
				if score < 0 {
					continue;
				}
				ips.push(LocalIp {
					addr: ip.to_string(),
					iface: iface.name.clone(),
					score,
				});
			}
		}
	}
	ips.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.addr.cmp(&b.addr)));
	ips
}

pub fn best_local_ip() -> Option<LocalIp> {
	detect_local_ips().into_iter().next()
}

/// Construit l'URL publique à partir de l'IP et du port.
pub fn build_url(ip: &str, port: u16) -> String {
	format!("http://{ip}:{port}")
}
