use axum::http::{Request, Response, StatusCode, Uri};
use http_body_util::Full;
use http_body_util::BodyExt;
use bytes::Bytes;
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use std::sync::Arc;
use crate::gateway::resolver::Resolver;
use crate::gateway::proxy_cache::ProxyCache;
use crate::breaker::CircuitBreaker;
use crate::middleware::pre_handler::PreHandlerManager;
use crate::middleware::modifier::ModifierManager;
use crate::utils::header::{is_grpc_request, is_grpc_web_request, get_trace_id, set_trace_id, remove_hop_by_hop_headers};
use crate::utils::trace_id::generate_trace_id;
use tracing;

pub struct ReverseProxy {
    client: Client<HttpConnector, Full<Bytes>>,
    resolver: Arc<Resolver>,
    proxy_cache: Arc<ProxyCache>,
    circuit_breaker: Arc<CircuitBreaker>,
    pre_handler_manager: Arc<PreHandlerManager>,
    modifier_manager: Arc<ModifierManager>,
    blacklist: Vec<String>,
    static_direct_hosts: Vec<String>,
}

impl ReverseProxy {
    pub fn new(
        resolver: Arc<Resolver>,
        proxy_cache: Arc<ProxyCache>,
        circuit_breaker: Arc<CircuitBreaker>,
        pre_handler_manager: Arc<PreHandlerManager>,
        modifier_manager: Arc<ModifierManager>,
        blacklist: Vec<String>,
        static_direct_hosts: Vec<String>,
    ) -> Self {
        let client = Client::builder(TokioExecutor::new())
            .pool_idle_timeout(std::time::Duration::from_secs(90))
            .build_http();

        Self {
            client,
            resolver,
            proxy_cache,
            circuit_breaker,
            pre_handler_manager,
            modifier_manager,
            blacklist,
            static_direct_hosts,
        }
    }

    pub async fn handle(&self, mut req: Request<Full<Bytes>>) -> Response<Full<Bytes>> {
        let trace_id = get_trace_id(req.headers())
            .unwrap_or_else(|| generate_trace_id());
        set_trace_id(req.headers_mut(), &trace_id);

        let span = tracing::info_span!("proxy_request", trace_id = %trace_id);
        let _enter = span.enter();

        if is_grpc_request(req.headers()) {
            tracing::debug!("gRPC request detected, routing to gRPC proxy");
            return self.make_error_response(StatusCode::NOT_IMPLEMENTED, "gRPC proxy not yet implemented");
        }

        if is_grpc_web_request(req.headers()) {
            tracing::debug!("gRPC-Web request detected, routing to gRPC-Web proxy");
            return self.make_error_response(StatusCode::NOT_IMPLEMENTED, "gRPC-Web proxy not yet implemented");
        }

        let host = req.uri()
            .host()
            .unwrap_or("")
            .to_string();

        for bl in &self.blacklist {
            if host.contains(bl) {
                tracing::warn!(host = %host, "Request blocked by blacklist");
                return self.make_error_response(StatusCode::FORBIDDEN, "Host not allowed");
            }
        }

        for sd_host in &self.static_direct_hosts {
            if host.as_str() == sd_host {
                tracing::debug!(host = %host, "Static direct host detected");
                return self.make_error_response(StatusCode::NOT_IMPLEMENTED, "Static direct not yet implemented");
            }
        }

        if let Err(result) = self.pre_handler_manager.execute(&mut req) {
            tracing::warn!(trace_id = %trace_id, "Pre-handler rejected request");
            return result.to_response();
        }

        let path = req.uri().path().to_string();
        let cache_key = format!("{}:{}", host, path);

        let target = if let Some(cached) = self.proxy_cache.get(&cache_key).await {
            cached
        } else if let Some(resolved) = self.resolver.resolve(&host, &path) {
            self.proxy_cache.insert(cache_key.clone(), resolved.clone()).await;
            resolved
        } else if let Some(custom) = self.resolver.resolve_custom_service(&host, &path) {
            tracing::debug!(host = %host, path = %path, "Resolved via custom service config");
            custom
        } else {
            tracing::warn!(host = %host, path = %path, "No route found");
            return self.make_error_response(StatusCode::NOT_FOUND, "No route found");
        };

        if target.is_custom_service && target.target_service.is_some() {
            let service_name = target.target_service.as_ref().unwrap();
            tracing::debug!(service = %service_name, "Routing to target service");
        }

        if target.port == 0 && target.target_service.is_some() {
            return self.make_error_response(StatusCode::BAD_GATEWAY, "Target service port not configured");
        }

        if self.circuit_breaker.is_open(&target.host) {
            tracing::warn!(host = %target.host, "Circuit breaker open");
            return self.make_error_response(StatusCode::GATEWAY_TIMEOUT, "Service temporarily unavailable");
        }

        let upstream_url = format!(
            "{}://{}:{}{}",
            target.scheme,
            target.host,
            target.port,
            self.rewrite_path(req.uri().path(), &target)
        );

        let upstream_uri: Uri = match upstream_url.parse() {
            Ok(uri) => uri,
            Err(e) => {
                tracing::error!(error = %e, "Failed to parse upstream URI");
                return self.make_error_response(StatusCode::BAD_GATEWAY, "Invalid upstream URL");
            }
        };

        *req.uri_mut() = upstream_uri;
        remove_hop_by_hop_headers(req.headers_mut());

        match self.client.request(req).await {
            Ok(resp) => {
                let (parts, body) = resp.into_parts();
                let body_bytes = match body.collect().await {
                    Ok(collected) => collected.to_bytes(),
                    Err(e) => {
                        tracing::error!(error = %e, "Failed to read response body");
                        return self.make_error_response(StatusCode::BAD_GATEWAY, "Failed to read upstream response");
                    }
                };

                let mut response = Response::from_parts(parts, Full::new(body_bytes));
                self.circuit_breaker.record_success(&target.host);

                if let Err(e) = self.modifier_manager.execute(&Request::default(), &mut response) {
                    tracing::error!(error = %e, "Modifier error");
                }

                response
            }
            Err(e) => {
                tracing::error!(error = %e, host = %target.host, "Upstream request failed");
                self.circuit_breaker.record_error(&target.host);
                self.make_error_response(StatusCode::BAD_GATEWAY, "Upstream request failed")
            }
        }
    }

    fn rewrite_path<'a>(&self, path: &'a str, target: &crate::gateway::resolver::ResolvedTarget) -> String {
        if target.is_custom_service {
            if let Some(ref rewrite_path) = target.rewrite {
                return rewrite_path.clone();
            }
        } else if target.strip_prefix {
            if let Some(ref rewrite) = target.rewrite {
                let prefix = rewrite.as_str();
                if let Some(stripped) = path.strip_prefix(prefix) {
                    return format!("{}{}", rewrite, stripped);
                }
            }
        }
        path.to_string()
    }

    fn make_error_response(&self, status: StatusCode, message: &str) -> Response<Full<Bytes>> {
        Response::builder()
            .status(status)
            .body(Full::new(Bytes::from(message.to_string())))
            .unwrap()
    }
}
