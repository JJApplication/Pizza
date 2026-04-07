use moka::future::Cache;
use crate::gateway::resolver::ResolvedTarget;
use std::time::Duration;

pub struct ProxyCache {
    cache: Cache<String, ResolvedTarget>,
}

impl ProxyCache {
    pub fn new(ttl_secs: u64, max_capacity: u64) -> Self {
        Self {
            cache: Cache::builder()
                .time_to_live(Duration::from_secs(ttl_secs))
                .max_capacity(max_capacity)
                .build(),
        }
    }

    pub async fn get(&self, key: &str) -> Option<ResolvedTarget> {
        self.cache.get(key).await
    }

    pub async fn insert(&self, key: String, target: ResolvedTarget) {
        self.cache.insert(key, target).await;
    }

    pub async fn invalidate(&self, key: &str) {
        self.cache.invalidate(key).await;
    }

    pub fn invalidate_all(&self) {
        self.cache.invalidate_all();
    }

    pub fn entry_count(&self) -> u64 {
        self.cache.entry_count()
    }
}
