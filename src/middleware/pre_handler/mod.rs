use bytes::Bytes;
use http::{Request, Response, StatusCode};
use http_body_util::Full;
use std::collections::HashMap;

pub trait PreHandler: Send + Sync {
    fn name(&self) -> &str;
    fn enabled(&self) -> bool;
    fn handle(&self, req: &mut Request<Full<Bytes>>) -> Result<(), PreHandlerResult>;
}

pub struct PreHandlerResult {
    pub status: StatusCode,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl PreHandlerResult {
    pub fn reject(status: StatusCode, message: &str) -> Self {
        Self {
            status,
            body: message.to_string(),
            headers: HashMap::new(),
        }
    }

    pub fn reject_with_headers(
        status: StatusCode,
        message: &str,
        headers: HashMap<String, String>,
    ) -> Self {
        Self {
            status,
            body: message.to_string(),
            headers,
        }
    }

    pub fn to_response(&self) -> Response<Full<Bytes>> {
        let mut resp = Response::builder()
            .status(self.status)
            .body(Full::new(Bytes::from(self.body.clone())))
            .unwrap();
        for (k, v) in &self.headers {
            if let Ok(name) = k.parse::<http::header::HeaderName>() {
                if let Ok(val) = v.parse::<http::header::HeaderValue>() {
                    resp.headers_mut().insert(name, val);
                }
            }
        }
        resp
    }
}

pub struct PreHandlerManager {
    handlers: Vec<Box<dyn PreHandler>>,
}

impl PreHandlerManager {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn register(&mut self, handler: Box<dyn PreHandler>) {
        self.handlers.push(handler);
    }

    pub fn execute(&self, req: &mut Request<Full<Bytes>>) -> Result<(), PreHandlerResult> {
        for handler in &self.handlers {
            if handler.enabled() {
                handler.handle(req)?;
            }
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.handlers.len()
    }
}

impl Default for PreHandlerManager {
    fn default() -> Self {
        Self::new()
    }
}
