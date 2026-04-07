use std::sync::atomic::{AtomicUsize, Ordering};

pub struct RoundRobinBalancer {
    counter: AtomicUsize,
    size: usize,
}

impl RoundRobinBalancer {
    pub fn new(size: usize) -> Self {
        Self {
            counter: AtomicUsize::new(0),
            size,
        }
    }

    pub fn next(&self) -> usize {
        if self.size == 0 {
            return 0;
        }
        self.counter.fetch_add(1, Ordering::Relaxed) % self.size
    }
}

pub struct RandomBalancer {
    size: usize,
}

impl RandomBalancer {
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    pub fn next(&self) -> usize {
        if self.size == 0 {
            return 0;
        }
        rand::random::<usize>() % self.size
    }
}

pub enum Balancer {
    RoundRobin(RoundRobinBalancer),
    Random(RandomBalancer),
}

impl Balancer {
    pub fn round_robin(size: usize) -> Self {
        Self::RoundRobin(RoundRobinBalancer::new(size))
    }

    pub fn random(size: usize) -> Self {
        Self::Random(RandomBalancer::new(size))
    }

    pub fn next(&self) -> usize {
        match self {
            Self::RoundRobin(b) => b.next(),
            Self::Random(b) => b.next(),
        }
    }
}
