use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QuotaResetSchedule {
    None,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone)]
pub struct UserQuotaTracker {
    pub user_id: Uuid,
    pub quota_bytes: Option<u64>,
    pub bytes_used: Arc<AtomicU64>,
    pub reset_schedule: QuotaResetSchedule,
    pub expires_at: Option<DateTime<Utc>>,
}

impl UserQuotaTracker {
    pub fn new(
        user_id: Uuid,
        quota_bytes: Option<u64>,
        expires_at: Option<DateTime<Utc>>,
        reset_schedule: QuotaResetSchedule,
    ) -> Self {
        Self {
            user_id,
            quota_bytes,
            bytes_used: Arc::new(AtomicU64::new(0)),
            reset_schedule,
            expires_at,
        }
    }

    pub fn record_bytes(&self, bytes: u64) {
        self.bytes_used.fetch_add(bytes, Ordering::Relaxed);
    }

    pub fn current_usage(&self) -> u64 {
        self.bytes_used.load(Ordering::Relaxed)
    }

    pub fn is_quota_exceeded(&self) -> bool {
        if let Some(quota) = self.quota_bytes {
            self.current_usage() >= quota
        } else {
            false
        }
    }

    pub fn is_expired(&self, current_time: DateTime<Utc>) -> bool {
        if let Some(exp) = self.expires_at {
            current_time >= exp
        } else {
            false
        }
    }

    pub fn is_active(&self, current_time: DateTime<Utc>) -> bool {
        !self.is_quota_exceeded() && !self.is_expired(current_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_quota_and_expiration() {
        let now = Utc::now();
        let tracker = UserQuotaTracker::new(
            Uuid::new_v4(),
            Some(1000),
            Some(now + Duration::hours(1)),
            QuotaResetSchedule::None,
        );

        assert!(tracker.is_active(now));

        tracker.record_bytes(500);
        assert!(!tracker.is_quota_exceeded());
        assert!(tracker.is_active(now));

        tracker.record_bytes(600);
        assert!(tracker.is_quota_exceeded());
        assert!(!tracker.is_active(now));

        // Test expiration
        let expired_tracker = UserQuotaTracker::new(
            Uuid::new_v4(),
            None,
            Some(now - Duration::seconds(1)),
            QuotaResetSchedule::None,
        );
        assert!(expired_tracker.is_expired(now));
        assert!(!expired_tracker.is_active(now));
    }
}
