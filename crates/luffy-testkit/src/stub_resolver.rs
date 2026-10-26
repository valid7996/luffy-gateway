use luffy_core::CoreError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolverProfile {
    pub max_udp_payload: u16,
    pub edns: bool,
    pub truncate_over: Option<u16>,
    pub tcp53: bool,
    pub qps_limit_per_client: Option<u32>,
    pub max_qname_len: u16,
    pub block_qtypes: Vec<u16>,
    pub nxdomain_hijack: bool,
    pub case_randomization: bool,
    pub drop_ratio: f32,
    pub delay_ms: u32,
}

impl Default for ResolverProfile {
    fn default() -> Self {
        Self {
            max_udp_payload: 1232,
            edns: true,
            truncate_over: None,
            tcp53: true,
            qps_limit_per_client: None,
            max_qname_len: 255,
            block_qtypes: Vec::new(),
            nxdomain_hijack: false,
            case_randomization: false,
            drop_ratio: 0.0,
            delay_ms: 0,
        }
    }
}

pub struct StubResolver {
    pub profile: ResolverProfile,
}

impl StubResolver {
    pub fn new(profile: ResolverProfile) -> Self {
        Self { profile }
    }

    /// Evaluates if an incoming DNS query should be dropped or truncated according to the Iran-like scenario profile.
    pub fn evaluate_query(&self, qname: &str, qtype: u16, payload_len: usize) -> Result<(), CoreError> {
        if qname.len() > self.profile.max_qname_len as usize {
            return Err(CoreError::Validation(format!(
                "QNAME length {} exceeds resolver maximum {}",
                qname.len(),
                self.profile.max_qname_len
            )));
        }

        if self.profile.block_qtypes.contains(&qtype) {
            return Err(CoreError::Validation(format!(
                "QTYPE {qtype} is blocked by resolver policy"
            )));
        }

        if payload_len > self.profile.max_udp_payload as usize {
            return Err(CoreError::Validation(format!(
                "UDP payload size {} exceeds resolver maximum {}",
                payload_len, self.profile.max_udp_payload
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub_resolver_qname_limit() {
        let mut profile = ResolverProfile::default();
        profile.max_qname_len = 100;

        let resolver = StubResolver::new(profile);
        assert!(resolver.evaluate_query("short.com", 1, 50).is_ok());

        let long_name = "a".repeat(101) + ".com";
        assert!(resolver.evaluate_query(&long_name, 1, 50).is_err());
    }

    #[test]
    fn test_stub_resolver_blocked_qtype() {
        let mut profile = ResolverProfile::default();
        profile.block_qtypes = vec![16]; // TXT record

        let resolver = StubResolver::new(profile);
        assert!(resolver.evaluate_query("example.com", 1, 50).is_ok()); // A record
        assert!(resolver.evaluate_query("example.com", 16, 50).is_err()); // TXT record
    }
}
