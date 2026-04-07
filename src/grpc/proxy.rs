use axum::http::{Request, Response, StatusCode};
use http_body_util::Full;
use http_body_util::BodyExt;
use bytes::Bytes;
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use std::collections::HashSet;
use crate::error::{Result, GrpcError};

pub struct GrpcProxy {
    client: Client<HttpConnector, Full<Bytes>>,
    allowed_addresses: HashSet<String>,
}

impl GrpcProxy {
    pub fn new(allowed_addresses: HashSet<String>) -> Self {
        Self {
            client: Client::builder(TokioExecutor::new()).build_http(),
            allowed_addresses,
        }
    }

    pub async fn proxy(&self, service: &str, method: &str, data: serde_json::Value, address: &str) -> Result<serde_json::Value> {
        if !self.allowed_addresses.is_empty() && !self.allowed_addresses.contains(address) {
            return Err(GrpcError::AddressNotWhitelisted(address.to_string()).into());
        }

        let url = format!("http://{}/{}.{}/invoke", address, service, method);

        let req = Request::builder()
            .method("POST")
            .uri(&url)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Full::new(Bytes::from(data.to_string())))
            .map_err(|e| GrpcError::ClientError(e.to_string()))?;

        let resp = self.client.request(req).await
            .map_err(|e| GrpcError::ClientError(e.to_string()))?;

        let body_bytes = resp.into_body().collect().await
            .map_err(|e| GrpcError::ClientError(e.to_string()))?
            .to_bytes();

        let result: serde_json::Value = serde_json::from_slice(&body_bytes)
            .map_err(|e| GrpcError::ClientError(e.to_string()))?;

        Ok(result)
    }

    pub fn is_allowed(&self, address: &str) -> bool {
        self.allowed_addresses.is_empty() || self.allowed_addresses.contains(address)
    }
}
