use crate::middleware::modifier::Modifier;
use bytes::Bytes;
use http::{Request, Response};
use http_body_util::Full;

pub struct GzipModifier {
    enabled: bool,
    min_length: usize,
}

impl GzipModifier {
    pub fn new(enabled: bool, min_length: usize) -> Self {
        Self {
            enabled,
            min_length,
        }
    }
}

impl Modifier for GzipModifier {
    fn name(&self) -> &str {
        "gzip"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn modify(
        &self,
        req: &Request<Full<Bytes>>,
        resp: &mut Response<Full<Bytes>>,
    ) -> Result<(), String> {
        let accepts_gzip = req
            .headers()
            .get(http::header::ACCEPT_ENCODING)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.contains("gzip"))
            .unwrap_or(false);

        if !accepts_gzip {
            return Ok(());
        }

        let body_len = resp.body().len();
        if body_len < self.min_length {
            return Ok(());
        }

        let content_type = resp
            .headers()
            .get(http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if !should_compress(content_type) {
            return Ok(());
        }

        let body_bytes = resp.body().clone().aggregate().to_bytes();
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        use std::io::Write;
        encoder.write_all(&body_bytes).map_err(|e| e.to_string())?;
        let compressed = encoder.finish().map_err(|e| e.to_string())?;

        if compressed.len() < body_len {
            *resp.body_mut() = Full::new(Bytes::from(compressed));
            resp.headers_mut()
                .insert(http::header::CONTENT_ENCODING, "gzip".parse().unwrap());
        }

        Ok(())
    }
}

fn should_compress(content_type: &str) -> bool {
    content_type.starts_with("text/")
        || content_type.starts_with("application/json")
        || content_type.starts_with("application/javascript")
        || content_type.starts_with("application/xml")
        || content_type.contains("font/")
}
