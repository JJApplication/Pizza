use crate::utils::time::daily_key;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct DailyStats {
    pub total_requests: AtomicU64,
    pub api_requests: AtomicU64,
    pub static_requests: AtomicU64,
    pub failures: AtomicU64,
}

pub struct StatCollector {
    total_requests: AtomicU64,
    api_requests: AtomicU64,
    static_requests: AtomicU64,
    failures: AtomicU64,
    daily_stats: RwLock<HashMap<String, DailyStats>>,
}

impl StatCollector {
    pub fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            api_requests: AtomicU64::new(0),
            static_requests: AtomicU64::new(0),
            failures: AtomicU64::new(0),
            daily_stats: RwLock::new(HashMap::new()),
        }
    }

    pub fn record_request(&self, is_api: bool) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        if is_api {
            self.api_requests.fetch_add(1, Ordering::Relaxed);
        } else {
            self.static_requests.fetch_add(1, Ordering::Relaxed);
        }

        let key = daily_key();
        let mut stats = self.daily_stats.write();
        let daily = stats.entry(key).or_insert_with(|| DailyStats::default());
        daily.total_requests.fetch_add(1, Ordering::Relaxed);
        if is_api {
            daily.api_requests.fetch_add(1, Ordering::Relaxed);
        } else {
            daily.static_requests.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_failure(&self) {
        self.failures.fetch_add(1, Ordering::Relaxed);

        let key = daily_key();
        let mut stats = self.daily_stats.write();
        let daily = stats.entry(key).or_insert_with(|| DailyStats::default());
        daily.failures.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_totals(&self) -> (u64, u64, u64, u64) {
        (
            self.total_requests.load(Ordering::Relaxed),
            self.api_requests.load(Ordering::Relaxed),
            self.static_requests.load(Ordering::Relaxed),
            self.failures.load(Ordering::Relaxed),
        )
    }

    pub fn get_daily(&self, date: &str) -> Option<DailyStats> {
        let stats = self.daily_stats.read();
        stats.get(date).map(|d| DailyStats {
            total_requests: AtomicU64::new(d.total_requests.load(Ordering::Relaxed)),
            api_requests: AtomicU64::new(d.api_requests.load(Ordering::Relaxed)),
            static_requests: AtomicU64::new(d.static_requests.load(Ordering::Relaxed)),
            failures: AtomicU64::new(d.failures.load(Ordering::Relaxed)),
        })
    }

    pub fn reset_daily(&self) {
        self.daily_stats.write().clear();
    }
}

impl Default for StatCollector {
    fn default() -> Self {
        Self::new()
    }
}
