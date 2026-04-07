use crate::middleware::pre_handler::{PreHandler, PreHandlerResult};
use bytes::Bytes;
use http::Request;
use http::StatusCode;
use http_body_util::Full;
use std::collections::HashSet;

pub struct DomainControlPreHandler {
    enabled: bool,
    allowed_domains: HashSet<String>,
    denied_domains: HashSet<String>,
}

impl DomainControlPreHandler {
    pub fn new(enabled: bool, allowed_domains: Vec<String>, denied_domains: Vec<String>) -> Self {
        Self {
            enabled,
            allowed_domains: allowed_domains
                .into_iter()
                .map(|d| d.to_lowercase())
                .collect(),
            denied_domains: denied_domains
                .into_iter()
                .map(|d| d.to_lowercase())
                .collect(),
        }
    }
}

impl PreHandler for DomainControlPreHandler {
    fn name(&self) -> &str {
        "domain_control"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn handle(&self, req: &mut Request<Full<Bytes>>) -> Result<(), PreHandlerResult> {
        let host = req
            .uri()
            .host()
            .map(|h| h.to_lowercase())
            .or_else(|| {
                req.headers()
                    .get(http::header::HOST)
                    .and_then(|v| v.to_str().ok())
                    .map(String::from)
            })
            .unwrap_or_default();

        if !self.denied_domains.is_empty() && self.denied_domains.contains(&host) {
            return Err(PreHandlerResult::reject(
                StatusCode::FORBIDDEN,
                "Domain not allowed",
            ));
        }

        if !self.allowed_domains.is_empty() && !self.allowed_domains.contains(&host) {
            return Err(PreHandlerResult::reject(
                StatusCode::FORBIDDEN,
                "Domain not allowed",
            ));
        }

        Ok(())
    }
}
