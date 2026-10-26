//! Token Bucket speed limiter for bandwidth throttling.

use std::time::Instant;

#[derive(Debug)]
pub struct TokenBucket {
    rate_bytes_per_sec: u64,
    capacity: u64,
    tokens: f64,
    last_update: Instant,
}

impl TokenBucket {
    pub fn new(speed_kbps: u32) -> Self {
        let rate_bytes_per_sec = (speed_kbps as u64) * 1024 / 8;
        let capacity = rate_bytes_per_sec * 2; // 2 seconds burst capacity
        Self {
            rate_bytes_per_sec,
            capacity,
            tokens: capacity as f64,
            last_update: Instant::now(),
        }
    }

    /// Checks if requested bytes can be transferred immediately and consumes tokens
    pub fn consume(&mut self, bytes: u64) -> bool {
        self.refill();

        if self.tokens >= bytes as f64 {
            self.tokens -= bytes as f64;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();
        self.last_update = now;

        self.tokens = (self.tokens + elapsed * (self.rate_bytes_per_sec as f64))
            .min(self.capacity as f64);
    }

    pub fn available_tokens(&mut self) -> u64 {
        self.refill();
        self.tokens.max(0.0) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_bucket_rate_limiting() {
        // 800 kbps = 102,400 bytes/sec, capacity = 204,800
        let mut bucket = TokenBucket::new(800);

        // Initial burst should succeed
        assert!(bucket.consume(150_000));
        // Immediate second large chunk exceeding capacity should fail
        assert!(!bucket.consume(100_000));
    }
}
