use crate::middleware::modifier::Modifier;
use bytes::Bytes;
use http::{Request, Response};
use http_body_util::Full;

pub struct SecureHeaderModifier {
    enabled: bool,
}

impl SecureHeaderModifier {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Modifier for SecureHeaderModifier {
    fn name(&self) -> &str {
        "secure_headers"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn modify(
        &self,
        _req: &Request<Full<Bytes>>,
        resp: &mut Response<Full<Bytes>>,
    ) -> Result<(), String> {
        let headers = resp.headers_mut();
        let _ = "max-age=63072000; includeSubDomains; preload"
            .parse::<http::HeaderValue>()
            .map(|v| {
                headers.insert("Strict-Transport-Security", v);
            });
        let _ = "DENY".parse::<http::HeaderValue>().map(|v| {
            headers.insert("X-Frame-Options", v);
        });
        let _ = "nosniff".parse::<http::HeaderValue>().map(|v| {
            headers.insert("X-Content-Type-Options", v);
        });
        let _ = "no-referrer".parse::<http::HeaderValue>().map(|v| {
            headers.insert("Referrer-Policy", v);
        });
        let _ = "default-src 'self'".parse::<http::HeaderValue>().map(|v| {
            headers.insert("Content-Security-Policy", v);
        });
        Ok(())
    }
}
