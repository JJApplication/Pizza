use dashmap::DashMap;
use parking_lot::RwLock;
use std::time::{Duration, Instant};

pub struct DomainBreaker {
    errors: RwLock<u32>,
    last_reset: RwLock<Instant>,
    bucket_size: u32,
    reset_duration: Duration,
}

impl DomainBreaker {
    pub fn new(bucket_size: u32, reset_secs: u64) -> Self {
        Self {
            errors: RwLock::new(0),
            last_reset: RwLock::new(Instant::now()),
            bucket_size,
            reset_duration: Duration::from_secs(reset_secs),
        }
    }

    pub fn record_error(&self) {
        let mut errors = self.errors.write();
        *errors += 1;
    }

    pub fn record_success(&self) {
        let mut errors = self.errors.write();
        *errors = 0;
    }

    pub fn is_open(&self) -> bool {
        let now = Instant::now();
        let last_reset = self.last_reset.read();

        if now.duration_since(*last_reset) > self.reset_duration {
            drop(last_reset);
            let mut errors = self.errors.write();
            let mut last_reset = self.last_reset.write();
            *errors = 0;
            *last_reset = Instant::now();
            return false;
        }

        *self.errors.read() >= self.bucket_size
    }

    pub fn reset(&self) {
        *self.errors.write() = 0;
        *self.last_reset.write() = Instant::now();
    }
}

pub struct CircuitBreaker {
    domains: DashMap<String, DomainBreaker>,
    bucket_size: u32,
    reset_secs: u64,
}

impl CircuitBreaker {
    pub fn new(bucket_size: u32, reset_secs: u64) -> Self {
        Self {
            domains: DashMap::new(),
            bucket_size,
            reset_secs,
        }
    }

    pub fn record_error(&self, domain: &str) {
        self.domains
            .entry(domain.to_string())
            .or_insert_with(|| DomainBreaker::new(self.bucket_size, self.reset_secs))
            .value()
            .record_error();
    }

    pub fn record_success(&self, domain: &str) {
        self.domains
            .entry(domain.to_string())
            .or_insert_with(|| DomainBreaker::new(self.bucket_size, self.reset_secs))
            .value()
            .record_success();
    }

    pub fn is_open(&self, domain: &str) -> bool {
        if let Some(entry) = self.domains.get(domain) {
            entry.value().is_open()
        } else {
            false
        }
    }

    pub fn reset_domain(&self, domain: &str) {
        if let Some(entry) = self.domains.get(domain) {
            entry.value().reset();
        }
    }

    pub fn reset_all(&self) {
        for entry in self.domains.iter() {
            entry.value().reset();
        }
    }
}
