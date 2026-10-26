//! Enforcement manager handling stream caps, token buckets, and session cutoffs.

use crate::quota_tracker::UserQuotaTracker;
use crate::token_bucket::TokenBucket;
use chrono::Utc;
use luffy_core::CoreError;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct StreamTracker {
    active_streams: AtomicU32,
}

impl StreamTracker {
    pub fn try_acquire(&self, cap: u32) -> bool {
        let mut current = self.active_streams.load(Ordering::Relaxed);
        loop {
            if current >= cap {
                return false;
            }
            match self.active_streams.compare_exchange_weak(
                current,
                current + 1,
                Ordering::SeqCst,
                Ordering::Relaxed,
            ) {
                Ok(_) => return true,
                Err(actual) => current = actual,
            }
        }
    }

    pub fn release(&self) {
        self.active_streams.fetch_sub(1, Ordering::SeqCst);
    }

    pub fn active(&self) -> u32 {
        self.active_streams.load(Ordering::Relaxed)
    }
}

pub struct EnforcementManager {
    users: Mutex<HashMap<Uuid, UserQuotaTracker>>,
    speed_limiters: Mutex<HashMap<Uuid, TokenBucket>>,
    stream_trackers: Mutex<HashMap<Uuid, Arc<StreamTracker>>>,
}

impl Default for EnforcementManager {
    fn default() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
            speed_limiters: Mutex::new(HashMap::new()),
            stream_trackers: Mutex::new(HashMap::new()),
        }
    }
}

impl EnforcementManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn register_user(&self, tracker: UserQuotaTracker, speed_kbps: Option<u32>) {
        let user_id = tracker.user_id;
        let mut users = self.users.lock().await;
        users.insert(user_id, tracker);

        if let Some(speed) = speed_kbps {
            let mut limiters = self.speed_limiters.lock().await;
            limiters.insert(user_id, TokenBucket::new(speed));
        }

        let mut streams = self.stream_trackers.lock().await;
        streams.insert(user_id, Arc::new(StreamTracker::default()));
    }

    /// Verifies if user can proceed with traffic and stream request
    pub async fn check_access(&self, user_id: &Uuid, stream_cap: u32) -> Result<(), CoreError> {
        let users = self.users.lock().await;
        let tracker = users
            .get(user_id)
            .ok_or_else(|| CoreError::NotFound("User not registered in meter".into()))?;

        if !tracker.is_active(Utc::now()) {
            return Err(CoreError::Validation("User quota exceeded or expired".into()));
        }

        let streams = self.stream_trackers.lock().await;
        if let Some(stream_tracker) = streams.get(user_id) {
            if !stream_tracker.try_acquire(stream_cap) {
                return Err(CoreError::Validation(format!(
                    "Concurrent stream limit ({stream_cap}) reached"
                )));
            }
        }

        Ok(())
    }

    pub async fn release_stream(&self, user_id: &Uuid) {
        let streams = self.stream_trackers.lock().await;
        if let Some(stream_tracker) = streams.get(user_id) {
            stream_tracker.release();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quota_tracker::QuotaResetSchedule;

    #[tokio::test]
    async fn test_enforcement_stream_cap() {
        let manager = EnforcementManager::new();
        let user_id = Uuid::new_v4();

        let tracker = UserQuotaTracker::new(user_id, None, None, QuotaResetSchedule::None);
        manager.register_user(tracker, None).await;

        let cap = 2;
        assert!(manager.check_access(&user_id, cap).await.is_ok());
        assert!(manager.check_access(&user_id, cap).await.is_ok());
        // Third concurrent stream should be rejected
        assert!(manager.check_access(&user_id, cap).await.is_err());

        // Release one stream and try again
        manager.release_stream(&user_id).await;
        assert!(manager.check_access(&user_id, cap).await.is_ok());
    }
}
