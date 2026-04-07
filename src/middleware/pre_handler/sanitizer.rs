use crate::middleware::pre_handler::{PreHandler, PreHandlerResult};
use bytes::Bytes;
use http::Request;
use http_body_util::Full;
use std::collections::HashSet;

pub struct HeaderSanitizerPreHandler {
    enabled: bool,
    remove_headers: HashSet<String>,
}

impl HeaderSanitizerPreHandler {
    pub fn new(enabled: bool, remove_headers: Vec<String>) -> Self {
        Self {
            enabled,
            remove_headers: remove_headers
                .into_iter()
                .map(|h| h.to_lowercase())
                .collect(),
        }
    }
}

impl PreHandler for HeaderSanitizerPreHandler {
    fn name(&self) -> &str {
        "header_sanitizer"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn handle(&self, req: &mut Request<Full<Bytes>>) -> Result<(), PreHandlerResult> {
        let headers_to_remove: Vec<_> = req
            .headers()
            .keys()
            .filter(|k| self.remove_headers.contains(&k.as_str().to_lowercase()))
            .cloned()
            .collect();

        for key in headers_to_remove {
            req.headers_mut().remove(key);
        }

        Ok(())
    }
}
