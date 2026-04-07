use crate::middleware::modifier::Modifier;
use crate::utils::header::get_internal_flag;
use bytes::Bytes;
use http::{Request, Response, StatusCode};
use http_body_util::Full;

pub struct FailResponseModifier {
    enabled: bool,
}

impl FailResponseModifier {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Modifier for FailResponseModifier {
    fn name(&self) -> &str {
        "fail_response"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn modify(
        &self,
        _req: &Request<Full<Bytes>>,
        resp: &mut Response<Full<Bytes>>,
    ) -> Result<(), String> {
        let status = resp.status();

        if status.is_server_error() || status.is_client_error() {
            if let Some(flag) = get_internal_flag(resp.headers()) {
                let body = match flag.as_str() {
                    "SandwichBucketLimit" => format!(
                        "{{\"error\":\"circuit breaker open\",\"status\":{}}}",
                        StatusCode::GATEWAY_TIMEOUT.as_u16()
                    ),
                    "SandwichReqLimit" => format!(
                        "{{\"error\":\"rate limit exceeded\",\"status\":{}}}",
                        StatusCode::TOO_MANY_REQUESTS.as_u16()
                    ),
                    "SandwichDomainNotAllow" => format!(
                        "{{\"error\":\"domain not allowed\",\"status\":{}}}",
                        StatusCode::FORBIDDEN.as_u16()
                    ),
                    "SandwichPreAuthFailed" => format!(
                        "{{\"error\":\"authentication failed\",\"status\":{}}}",
                        StatusCode::UNAUTHORIZED.as_u16()
                    ),
                    "SandwichBackendError" => format!(
                        "{{\"error\":\"backend error\",\"status\":{}}}",
                        StatusCode::INTERNAL_SERVER_ERROR.as_u16()
                    ),
                    "SandwichServiceStopped" => format!(
                        "{{\"error\":\"service stopped\",\"status\":{}}}",
                        StatusCode::SERVICE_UNAVAILABLE.as_u16()
                    ),
                    "SandwichProxyConnectError" => format!(
                        "{{\"error\":\"proxy connect error\",\"status\":{}}}",
                        StatusCode::BAD_GATEWAY.as_u16()
                    ),
                    _ => format!(
                        "{{\"error\":\"unknown error\",\"status\":{}}}",
                        status.as_u16()
                    ),
                };

                *resp.body_mut() = Full::new(Bytes::from(body));
                resp.headers_mut().insert(
                    http::header::CONTENT_TYPE,
                    "application/json".parse().unwrap(),
                );
            }
        }

        Ok(())
    }
}
