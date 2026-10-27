//! Port status and firewall rule inspector.

use serde::{Deserialize, Serialize};
use std::net::{TcpListener, UdpSocket};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortEntry {
    pub port: u16,
    pub proto: String,
    pub service_name: String,
    pub is_listening: bool,
    pub firewall_open: bool,
    pub reachability_proven: bool,
}

pub struct PortsPageManager;

impl PortsPageManager {
    pub fn get_standard_ports() -> Vec<PortEntry> {
        vec![
            PortEntry {
                port: 53,
                proto: "UDP/TCP".to_string(),
                service_name: "Luffy DNS Router (Soar/Master/Slip)".to_string(),
                is_listening: true,
                firewall_open: true,
                reachability_proven: true,
            },
            PortEntry {
                port: 80,
                proto: "TCP".to_string(),
                service_name: "ACME Let's Encrypt HTTP Challenge".to_string(),
                is_listening: true,
                firewall_open: true,
                reachability_proven: false,
            },
            PortEntry {
                port: 443,
                proto: "TCP".to_string(),
                service_name: "Panel HTTPS / NaiveProxy".to_string(),
                is_listening: true,
                firewall_open: true,
                reachability_proven: false,
            },
            PortEntry {
                port: 8443,
                proto: "TCP".to_string(),
                service_name: "GooseRelay Exit Server".to_string(),
                is_listening: true,
                firewall_open: true,
                reachability_proven: false,
            },
            PortEntry {
                port: 22,
                proto: "TCP".to_string(),
                service_name: "SSH Server (Protected)".to_string(),
                is_listening: true,
                firewall_open: true,
                reachability_proven: true,
            },
        ]
    }

    /// Generates idempotent UFW command marked with Luffy ownership
    pub fn generate_ufw_allow(port: u16, proto: &str) -> String {
        format!("ufw allow {port}/{proto} comment 'luffy-managed'")
    }

    /// Real local bind-probe: reports whether anything currently occupies the
    /// port on this host (TCP or UDP, loopback scope).
    pub fn is_port_in_use(port: u16, proto: &str) -> bool {
        match proto {
            "UDP" => UdpSocket::bind(("127.0.0.1", port)).is_err(),
            "TCP" => TcpListener::bind(("127.0.0.1", port)).is_err(),
            // UDP/TCP combined: busy when either transport cannot be bound.
            _ => UdpSocket::bind(("127.0.0.1", port)).is_err()
                || TcpListener::bind(("127.0.0.1", port)).is_err(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ports_inspector() {
        let ports = PortsPageManager::get_standard_ports();
        assert_eq!(ports.len(), 5);
        assert!(ports.iter().any(|p| p.port == 53));
        assert!(ports.iter().any(|p| p.port == 8443));

        let cmd = PortsPageManager::generate_ufw_allow(53, "udp");
        assert_eq!(cmd, "ufw allow 53/udp comment 'luffy-managed'");
    }

    #[test]
    fn test_bind_probe_reflects_reality() {
        // Ephemeral listener: must be reported as in-use while bound.
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        assert!(PortsPageManager::is_port_in_use(port, "TCP"));
        assert!(!PortsPageManager::is_port_in_use(0, "TCP"));
    }
}
