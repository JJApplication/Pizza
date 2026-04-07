use crate::middleware::pre_handler::{PreHandler, PreHandlerResult};
use bytes::Bytes;
use http::Request;
use http::StatusCode;
use http_body_util::Full;

pub struct ImageProtectPreHandler {
    enabled: bool,
    allowed_referers: Vec<String>,
}

impl ImageProtectPreHandler {
    pub fn new(enabled: bool, allowed_referers: Vec<String>) -> Self {
        Self {
            enabled,
            allowed_referers,
        }
    }

    fn is_image_request(req: &Request<Full<Bytes>>) -> bool {
        req.uri().path().ends_with(".jpg")
            || req.uri().path().ends_with(".jpeg")
            || req.uri().path().ends_with(".png")
            || req.uri().path().ends_with(".gif")
            || req.uri().path().ends_with(".webp")
            || req.uri().path().ends_with(".svg")
    }
}

impl PreHandler for ImageProtectPreHandler {
    fn name(&self) -> &str {
        "image_protect"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn handle(&self, req: &mut Request<Full<Bytes>>) -> Result<(), PreHandlerResult> {
        if !Self::is_image_request(req) {
            return Ok(());
        }

        let referer = req
            .headers()
            .get(http::header::REFERER)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if referer.is_empty() {
            return Err(PreHandlerResult::reject(
                StatusCode::FORBIDDEN,
                "Hotlinking not allowed",
            ));
        }

        let allowed = self
            .allowed_referers
            .iter()
            .any(|allowed| referer.starts_with(allowed));
        if !allowed {
            return Err(PreHandlerResult::reject(
                StatusCode::FORBIDDEN,
                "Hotlinking not allowed",
            ));
        }

        Ok(())
    }
}
