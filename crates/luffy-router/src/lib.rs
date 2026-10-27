//! Luffy Router: DNS Front Router on UDP/TCP port 53.
//! Dispatches queries based on longest-suffix match to loopback engine backends.

pub mod dns;
pub mod server;

pub use server::{DnsFrontHandle, DnsFrontServer};

use dns::DnsPacketParser;
use luffy_core::CoreError;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct RouteStats {
    pub queries: AtomicU64,
    pub bytes_in: AtomicU64,
    pub bytes_out: AtomicU64,
    pub drops: AtomicU64,
}

#[derive(Debug, Clone)]
pub struct BackendRoute {
    pub suffix: String,
    pub backend: SocketAddr,
    pub stats: Arc<RouteStats>,
}

#[derive(Debug, Default)]
pub struct DnsRouter {
    routes: HashMap<String, BackendRoute>,
    default_backend: Option<SocketAddr>,
}

impl DnsRouter {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            default_backend: None,
        }
    }

    pub fn set_default_backend(&mut self, backend: SocketAddr) {
        self.default_backend = Some(backend);
    }

    pub fn default_backend(&self) -> Option<SocketAddr> {
        self.default_backend
    }

    pub fn add_route(
        &mut self,
        suffix: impl Into<String>,
        backend: SocketAddr,
    ) -> Result<(), CoreError> {
        let s = suffix.into().trim_end_matches('.').to_lowercase();
        if self.routes.contains_key(&s) {
            return Err(CoreError::Validation(format!(
                "Duplicate route suffix already registered: {s}"
            )));
        }

        self.routes.insert(
            s.clone(),
            BackendRoute {
                suffix: s,
                backend,
                stats: Arc::new(RouteStats::default()),
            },
        );
        Ok(())
    }

    pub fn find_route(&self, qname: &str) -> Option<&BackendRoute> {
        let name = qname.trim_end_matches('.').to_lowercase();
        let mut best: Option<(&str, &BackendRoute)> = None;

        for (suffix, route) in &self.routes {
            let matches = name == *suffix
                || (name.len() > suffix.len()
                    && name.ends_with(suffix.as_str())
                    && name.as_bytes()[name.len() - suffix.len() - 1] == b'.');
            if matches {
                match best {
                    Some((prev_s, _)) if suffix.len() > prev_s.len() => {
                        best = Some((suffix.as_str(), route));
                    }
                    None => {
                        best = Some((suffix.as_str(), route));
                    }
                    _ => {}
                }
            }
        }

        best.map(|(_, route)| route)
    }

    pub fn route_packet(&self, packet: &[u8]) -> Result<SocketAddr, CoreError> {
        let (_header, question, _offset) = DnsPacketParser::parse_first_question(packet)?;
        if let Some(route) = self.find_route(&question.qname) {
            route.stats.queries.fetch_add(1, Ordering::Relaxed);
            route.stats.bytes_in.fetch_add(packet.len() as u64, Ordering::Relaxed);
            Ok(route.backend)
        } else {
            Err(CoreError::NotFound(format!(
                "No backend route found for QNAME '{}'",
                question.qname
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_suffix_match() {
        let mut router = DnsRouter::new();
        router.add_route("example.com", "127.0.0.1:5310".parse().unwrap()).unwrap();
        router.add_route("soar.example.com", "127.0.0.1:5311".parse().unwrap()).unwrap();
        router.add_route("master.example.com", "127.0.0.1:5320".parse().unwrap()).unwrap();

        assert_eq!(
            router.find_route("client.soar.example.com").unwrap().backend,
            "127.0.0.1:5311".parse().unwrap()
        );
        assert_eq!(
            router.find_route("client.master.example.com").unwrap().backend,
            "127.0.0.1:5320".parse().unwrap()
        );
        assert_eq!(
            router.find_route("general.example.com").unwrap().backend,
            "127.0.0.1:5310".parse().unwrap()
        );
        assert!(router.find_route("random.org").is_none());
    }

    #[test]
    fn test_duplicate_route_rejection() {
        let mut router = DnsRouter::new();
        assert!(router.add_route("example.com", "127.0.0.1:5310".parse().unwrap()).is_ok());
        assert!(router.add_route("example.com", "127.0.0.1:5320".parse().unwrap()).is_err());
    }
}
