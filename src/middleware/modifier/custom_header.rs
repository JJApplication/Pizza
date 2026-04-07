use crate::middleware::modifier::Modifier;
use bytes::Bytes;
use http::{Request, Response};
use http_body_util::Full;

pub struct CustomHeaderEntry {
    pub key: String,
    pub value: String,
}

pub struct CustomHeaderModifier {
    enabled: bool,
    headers: Vec<(String, String)>,
}

impl CustomHeaderModifier {
    pub fn new(enabled: bool, custom_headers: Vec<CustomHeaderEntry>) -> Self {
        Self {
            enabled,
            headers: custom_headers
                .into_iter()
                .map(|h| (h.key, h.value))
                .collect(),
        }
    }
}

impl Modifier for CustomHeaderModifier {
    fn name(&self) -> &str {
        "custom_headers"
    }
    fn enabled(&self) -> bool {
        self.enabled && !self.headers.is_empty()
    }

    fn modify(
        &self,
        _req: &Request<Full<Bytes>>,
        resp: &mut Response<Full<Bytes>>,
    ) -> Result<(), String> {
        for (key, value) in &self.headers {
            if let Ok(name) = key.parse::<http::header::HeaderName>() {
                if let Ok(val) = value.parse::<http::header::HeaderValue>() {
                    resp.headers_mut().insert(name, val);
                }
            }
        }
        Ok(())
    }
}
