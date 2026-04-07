use axum::http::{Request, Response, StatusCode};
use http_body_util::Full;
use bytes::Bytes;

pub struct GrpcWebProxy;

impl GrpcWebProxy {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(&self, _req: Request<Full<Bytes>>) -> Response<Full<Bytes>> {
        Response::builder()
            .status(StatusCode::NOT_IMPLEMENTED)
            .body(Full::new(Bytes::from("gRPC-Web proxy not yet fully implemented")))
            .unwrap()
    }
}

impl Default for GrpcWebProxy {
    fn default() -> Self {
        Self::new()
    }
}
