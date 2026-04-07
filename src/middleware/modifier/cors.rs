use crate::middleware::modifier::Modifier;
use bytes::Bytes;
use http::{Request, Response};
use http_body_util::Full;

pub struct CorsModifier {
    enabled: bool,
    allow_origins: Vec<String>,
    allow_methods: Vec<String>,
    allow_headers: Vec<String>,
}

impl CorsModifier {
    pub fn new(
        enabled: bool,
        allow_origins: Vec<String>,
        allow_methods: Vec<String>,
        allow_headers: Vec<String>,
    ) -> Self {
        Self {
            enabled,
            allow_origins,
            allow_methods,
            allow_headers,
        }
    }
}

impl Modifier for CorsModifier {
    fn name(&self) -> &str {
        "cors"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn modify(
        &self,
        req: &Request<Full<Bytes>>,
        resp: &mut Response<Full<Bytes>>,
    ) -> Result<(), String> {
        let origin = req
            .headers()
            .get(http::header::ORIGIN)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("*");

        let allow_origin = if self.allow_origins.is_empty() {
            "*".to_string()
        } else {
            self.allow_origins
                .iter()
                .find(|o| o.as_str() == "*" || o.as_str() == origin)
                .map(|s| s.clone())
                .unwrap_or_else(|| "*".to_string())
        };

        let headers = resp.headers_mut();
        let _ = allow_origin.parse::<http::HeaderValue>().map(|v| {
            headers.insert("Access-Control-Allow-Origin", v);
        });

        let methods = if self.allow_methods.is_empty() {
            "GET, POST, PUT, DELETE, OPTIONS, PATCH".to_string()
        } else {
            self.allow_methods.join(", ")
        };
        let _ = methods.parse::<http::HeaderValue>().map(|v| {
            headers.insert("Access-Control-Allow-Methods", v);
        });

        let hdrs = if self.allow_headers.is_empty() {
            "Content-Type, Authorization, X-Requested-With".to_string()
        } else {
            self.allow_headers.join(", ")
        };
        let _ = hdrs.parse::<http::HeaderValue>().map(|v| {
            headers.insert("Access-Control-Allow-Headers", v);
        });

        if req.method() == http::Method::OPTIONS {
            *resp.status_mut() = http::StatusCode::NO_CONTENT;
        }

        Ok(())
    }
}
