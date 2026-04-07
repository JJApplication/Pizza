use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use parking_lot::RwLock;

pub struct LatencyMeasurement {
    latencies: RwLock<HashMap<String, Vec<u64>>>,
    averages: RwLock<HashMap<String, u64>>,
}

impl LatencyMeasurement {
    pub fn new() -> Self {
        Self {
            latencies: RwLock::new(HashMap::new()),
            averages: RwLock::new(HashMap::new()),
        }
    }

    pub fn record(&self, backend: &str, latency_ms: u64) {
        let mut latencies = self.latencies.write();
        let entries = latencies.entry(backend.to_string()).or_insert_with(Vec::new);
        entries.push(latency_ms);

        if entries.len() > 100 {
            entries.drain(..entries.len() - 100);
        }

        let avg = entries.iter().sum::<u64>() / entries.len() as u64;
        self.averages.write().insert(backend.to_string(), avg);
    }

    pub async fn measure<F, Fut>(&self, backend: &str, f: F) -> Result<(), String>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<(), String>>,
    {
        let start = Instant::now();
        match f().await {
            Ok(()) => {
                let elapsed = start.elapsed().as_millis() as u64;
                self.record(backend, elapsed);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_average(&self, backend: &str) -> Option<u64> {
        self.averages.read().get(backend).copied()
    }

    pub fn get_all_averages(&self) -> HashMap<String, u64> {
        self.averages.read().clone()
    }

    pub fn clear(&self) {
        self.latencies.write().clear();
        self.averages.write().clear();
    }
}

impl Default for LatencyMeasurement {
    fn default() -> Self {
        Self::new()
    }
}
