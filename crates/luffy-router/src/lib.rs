//! Luffy Router: DNS Front Router on UDP/TCP port 53.
//! Dispatches queries based on longest-suffix match to loopback engine backends.

use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Debug, Default)]
pub struct DnsRouter {
    pub routes: HashMap<String, SocketAddr>,
}

impl DnsRouter {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, suffix: impl Into<String>, backend: SocketAddr) {
        self.routes.insert(suffix.into().to_lowercase(), backend);
    }

    pub fn find_route(&self, qname: &str) -> Option<SocketAddr> {
        let name = qname.trim_end_matches('.').to_lowercase();
        let mut best_match: Option<(&str, SocketAddr)> = None;

        for (suffix, &backend) in &self.routes {
            let s = suffix.trim_end_matches('.');
            if name == s || name.ends_with(&format!(".{s}")) {
                match best_match {
                    Some((prev, _)) if s.len() > prev.len() => {
                        best_match = Some((s, backend));
                    }
                    None => {
                        best_match = Some((s, backend));
                    }
                    _ => {}
                }
            }
        }

        best_match.map(|(_, backend)| backend)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_suffix_match() {
        let mut router = DnsRouter::new();
        router.add_route("example.com", "127.0.0.1:5301".parse().unwrap());
        router.add_route("sub.example.com", "127.0.0.1:5302".parse().unwrap());

        assert_eq!(
            router.find_route("test.sub.example.com"),
            Some("127.0.0.1:5302".parse().unwrap())
        );
        assert_eq!(
            router.find_route("other.example.com"),
            Some("127.0.0.1:5301".parse().unwrap())
        );
        assert_eq!(router.find_route("example.org"), None);
    }
}
