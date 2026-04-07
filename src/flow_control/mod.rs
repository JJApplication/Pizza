use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

pub trait RateLimiter: Send + Sync {
    fn allow(&self) -> bool;
    fn allow_n(&self, n: u32) -> bool;
}

pub struct FixedWindowLimiter {
    max_requests: u64,
    window: Duration,
    current_window: parking_lot::Mutex<Window>,
}

struct Window {
    start: Instant,
    count: u64,
}

impl FixedWindowLimiter {
    pub fn new(max_requests: u64, window_secs: u64) -> Self {
        Self {
            max_requests,
            window: Duration::from_secs(window_secs),
            current_window: parking_lot::Mutex::new(Window {
                start: Instant::now(),
                count: 0,
            }),
        }
    }
}

impl RateLimiter for FixedWindowLimiter {
    fn allow(&self) -> bool {
        self.allow_n(1)
    }

    fn allow_n(&self, n: u32) -> bool {
        let mut window = self.current_window.lock();
        let now = Instant::now();

        if now.duration_since(window.start) > self.window {
            window.start = now;
            window.count = 0;
        }

        if window.count + n as u64 <= self.max_requests {
            window.count += n as u64;
            true
        } else {
            false
        }
    }
}

pub struct TokenBucketLimiter {
    capacity: u64,
    tokens: AtomicU64,
    refill_rate: f64,
    last_refill: parking_lot::Mutex<Instant>,
}

impl TokenBucketLimiter {
    pub fn new(capacity: u64, refill_per_sec: f64) -> Self {
        Self {
            capacity,
            tokens: AtomicU64::new(capacity),
            refill_rate: refill_per_sec,
            last_refill: parking_lot::Mutex::new(Instant::now()),
        }
    }

    fn refill(&self) {
        let mut last = self.last_refill.lock();
        let now = Instant::now();
        let elapsed = now.duration_since(*last).as_secs_f64();
        let new_tokens = (elapsed * self.refill_rate) as u64;
        if new_tokens > 0 {
            let current = self.tokens.load(Ordering::Relaxed);
            let updated = (current + new_tokens).min(self.capacity);
            self.tokens.store(updated, Ordering::Relaxed);
            *last = now;
        }
    }
}

impl RateLimiter for TokenBucketLimiter {
    fn allow(&self) -> bool {
        self.refill();
        let current = self.tokens.load(Ordering::Relaxed);
        if current > 0 {
            self.tokens.fetch_sub(1, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    fn allow_n(&self, n: u32) -> bool {
        self.refill();
        let current = self.tokens.load(Ordering::Relaxed);
        if current >= n as u64 {
            self.tokens.fetch_sub(n as u64, Ordering::Relaxed);
            true
        } else {
            false
        }
    }
}

pub struct LeakyBucketLimiter {
    capacity: u64,
    leak_rate: f64,
    water: AtomicU64,
    last_leak: parking_lot::Mutex<Instant>,
}

impl LeakyBucketLimiter {
    pub fn new(capacity: u64, leak_per_sec: f64) -> Self {
        Self {
            capacity,
            leak_rate: leak_per_sec,
            water: AtomicU64::new(0),
            last_leak: parking_lot::Mutex::new(Instant::now()),
        }
    }

    fn leak(&self) {
        let mut last = self.last_leak.lock();
        let now = Instant::now();
        let elapsed = now.duration_since(*last).as_secs_f64();
        let leaked = (elapsed * self.leak_rate) as u64;
        if leaked > 0 {
            let current = self.water.load(Ordering::Relaxed);
            let updated = current.saturating_sub(leaked);
            self.water.store(updated, Ordering::Relaxed);
            *last = now;
        }
    }
}

impl RateLimiter for LeakyBucketLimiter {
    fn allow(&self) -> bool {
        self.allow_n(1)
    }

    fn allow_n(&self, n: u32) -> bool {
        self.leak();
        let current = self.water.load(Ordering::Relaxed);
        if current + n as u64 <= self.capacity {
            self.water.fetch_add(n as u64, Ordering::Relaxed);
            true
        } else {
            false
        }
    }
}

pub struct SlidingWindowLimiter {
    max_requests: u64,
    window: Duration,
    requests: parking_lot::Mutex<Vec<Instant>>,
}

impl SlidingWindowLimiter {
    pub fn new(max_requests: u64, window_secs: u64) -> Self {
        Self {
            max_requests,
            window: Duration::from_secs(window_secs),
            requests: parking_lot::Mutex::new(Vec::new()),
        }
    }
}

impl RateLimiter for SlidingWindowLimiter {
    fn allow(&self) -> bool {
        self.allow_n(1)
    }

    fn allow_n(&self, n: u32) -> bool {
        let mut requests = self.requests.lock();
        let now = Instant::now();
        let cutoff = now - self.window;

        requests.retain(|t| *t > cutoff);

        if requests.len() + n as usize <= self.max_requests as usize {
            for _ in 0..n {
                requests.push(now);
            }
            true
        } else {
            false
        }
    }
}

pub enum LimiterAlgorithm {
    FixedWindow,
    TokenBucket,
    LeakyBucket,
    SlidingWindow,
}

pub fn create_limiter(
    algorithm: LimiterAlgorithm,
    max_requests: u64,
    window_secs: u64,
) -> Box<dyn RateLimiter> {
    match algorithm {
        LimiterAlgorithm::FixedWindow => {
            Box::new(FixedWindowLimiter::new(max_requests, window_secs))
        }
        LimiterAlgorithm::TokenBucket => {
            let rate = max_requests as f64 / window_secs as f64;
            Box::new(TokenBucketLimiter::new(max_requests, rate))
        }
        LimiterAlgorithm::LeakyBucket => {
            let rate = max_requests as f64 / window_secs as f64;
            Box::new(LeakyBucketLimiter::new(max_requests, rate))
        }
        LimiterAlgorithm::SlidingWindow => {
            Box::new(SlidingWindowLimiter::new(max_requests, window_secs))
        }
    }
}
