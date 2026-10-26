//! Luffy Meter: Traffic counters, token-bucket speed limiters, and stream caps.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct TrafficCounter {
    bytes_in: Arc<AtomicU64>,
    bytes_out: Arc<AtomicU64>,
}

impl TrafficCounter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_in(&self, bytes: u64) {
        self.bytes_in.fetch_add(bytes, Ordering::Relaxed);
    }

    pub fn record_out(&self, bytes: u64) {
        self.bytes_out.fetch_add(bytes, Ordering::Relaxed);
    }

    pub fn total(&self) -> u64 {
        self.bytes_in.load(Ordering::Relaxed) + self.bytes_out.load(Ordering::Relaxed)
    }
}
