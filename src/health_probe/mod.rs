use moka::future::Cache;
use std::time::Duration;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub alive: bool,
    pub last_check: i64,
    pub consecutive_failures: u32,
    pub response_time_ms: u64,
}

pub struct HealthProbe {
    cache: Cache<String, HealthStatus>,
    probe_interval_secs: u64,
}

impl HealthProbe {
    pub fn new(probe_interval_secs: u64, ttl_secs: u64) -> Self {
        Self {
            cache: Cache::builder()
                .time_to_live(Duration::from_secs(ttl_secs))
                .max_capacity(10000)
                .build(),
            probe_interval_secs,
        }
    }

    pub async fn get_status(&self, domain: &str) -> Option<HealthStatus> {
        self.cache.get(domain).await
    }

    pub async fn set_alive(&self, domain: &str, response_time_ms: u64) {
        let status = HealthStatus {
            alive: true,
            last_check: chrono::Local::now().timestamp(),
            consecutive_failures: 0,
            response_time_ms,
        };
        self.cache.insert(domain.to_string(), status).await;
    }

    pub async fn set_dead(&self, domain: &str) {
        let existing = self.cache.get(domain).await;
        let failures = existing.map(|s| s.consecutive_failures + 1).unwrap_or(1);

        let status = HealthStatus {
            alive: false,
            last_check: chrono::Local::now().timestamp(),
            consecutive_failures: failures,
            response_time_ms: 0,
        };
        self.cache.insert(domain.to_string(), status).await;
    }

    pub async fn is_alive(&self, domain: &str) -> bool {
        self.cache.get(domain).await.map(|s| s.alive).unwrap_or(true)
    }

    pub fn probe_interval(&self) -> u64 {
        self.probe_interval_secs
    }
}
