use crate::flow_control::{FixedWindowLimiter, RateLimiter};
use crate::middleware::pre_handler::{PreHandler, PreHandlerResult};
use bytes::Bytes;
use http::Request;
use http::StatusCode;
use http_body_util::Full;
use parking_lot::RwLock;
use std::collections::HashMap;

pub struct RateLimiterPreHandler {
    enabled: bool,
    global_limiter: Box<dyn RateLimiter>,
    per_ip_limiters: RwLock<HashMap<String, Box<dyn RateLimiter>>>,
}

impl RateLimiterPreHandler {
    pub fn new(enabled: bool, requests_per_second: u64) -> Self {
        Self {
            enabled,
            global_limiter: Box::new(FixedWindowLimiter::new(requests_per_second, 1)),
            per_ip_limiters: RwLock::new(HashMap::new()),
        }
    }

    fn check_ip_limit(&self, ip: &str, rps: u64) -> bool {
        let mut limiters = self.per_ip_limiters.write();
        let limiter = limiters
            .entry(ip.to_string())
            .or_insert_with(|| Box::new(FixedWindowLimiter::new(rps, 1)));
        limiter.allow()
    }
}

impl PreHandler for RateLimiterPreHandler {
    fn name(&self) -> &str {
        "rate_limiter"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn handle(&self, _req: &mut Request<Full<Bytes>>) -> Result<(), PreHandlerResult> {
        if !self.global_limiter.allow() {
            return Err(PreHandlerResult::reject(
                StatusCode::TOO_MANY_REQUESTS,
                "Rate limit exceeded",
            ));
        }
        Ok(())
    }
}
