use moka::future::Cache;
use std::path::PathBuf;
use std::time::Duration;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct FileCache {
    cache: Option<Cache<PathBuf, Vec<u8>>>,
    hits: AtomicU64,
    misses: AtomicU64,
}

impl FileCache {
    pub fn new(enabled: bool, ttl_secs: u64) -> Self {
        Self {
            cache: if enabled {
                Some(Cache::builder()
                    .time_to_live(Duration::from_secs(ttl_secs))
                    .max_capacity(10000)
                    .build())
            } else {
                None
            },
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        }
    }

    pub async fn get(&self, path: &PathBuf) -> Option<Vec<u8>> {
        if let Some(cache) = &self.cache {
            if let Some(content) = cache.get(path).await {
                self.hits.fetch_add(1, Ordering::Relaxed);
                return Some(content);
            }
        }
        self.misses.fetch_add(1, Ordering::Relaxed);
        None
    }

    pub async fn insert(&self, path: PathBuf, content: Vec<u8>) {
        if let Some(cache) = &self.cache {
            cache.insert(path, content).await;
        }
    }

    pub fn clear(&self) {
        if let Some(cache) = &self.cache {
            cache.invalidate_all();
        }
    }

    pub fn stats(&self) -> (u64, u64) {
        (self.hits.load(Ordering::Relaxed), self.misses.load(Ordering::Relaxed))
    }

    pub fn is_enabled(&self) -> bool {
        self.cache.is_some()
    }
}
