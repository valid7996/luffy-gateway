//! Port status and firewall rule inspector.

use serde::{Deserialize, Serialize};

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
}
