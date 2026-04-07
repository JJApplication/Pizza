use crate::constants::HEADER_TRACE_ID;
use crate::middleware::modifier::Modifier;
use crate::utils::header::{get_trace_id, set_trace_id};
use crate::utils::trace_id::generate_trace_id;
use bytes::Bytes;
use http::{Request, Response};
use http_body_util::Full;

pub struct TraceIdModifier {
    enabled: bool,
}

impl TraceIdModifier {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Modifier for TraceIdModifier {
    fn name(&self) -> &str {
        "trace_id"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn modify(
        &self,
        req: &Request<Full<Bytes>>,
        resp: &mut Response<Full<Bytes>>,
    ) -> Result<(), String> {
        let trace_id = get_trace_id(req.headers()).unwrap_or_else(|| generate_trace_id());

        if let Ok(val) = trace_id.parse() {
            resp.headers_mut().insert(HEADER_TRACE_ID, val);
        }

        Ok(())
    }
}
