use axum::Router;
use axum::routing::get;
use axum::http::{Request, StatusCode};
use axum::response::Response;
use axum::body::Body;
use http_body_util::Full;
use http_body_util::BodyExt;
use bytes::Bytes;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use crate::error::Result;
use crate::gateway::proxy::ReverseProxy;

pub struct GatewayServer {
    addr: SocketAddr,
    proxy: Arc<ReverseProxy>,
    tls_cert: Option<String>,
    tls_key: Option<String>,
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl GatewayServer {
    pub fn new(addr: SocketAddr, proxy: Arc<ReverseProxy>) -> Self {
        Self {
            addr,
            proxy,
            tls_cert: None,
            tls_key: None,
            shutdown_tx: None,
        }
    }

    pub fn with_tls(mut self, cert: String, key: String) -> Self {
        self.tls_cert = Some(cert);
        self.tls_key = Some(key);
        self
    }

    pub async fn start(&mut self) -> Result<()> {
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
        self.shutdown_tx = Some(shutdown_tx);

        let proxy = self.proxy.clone();
        let app = Router::new()
            .route("/", get(root_handler))
            .fallback(move |req| proxy_handle(req, proxy.clone()));

        let listener = TcpListener::bind(self.addr).await?;
        tracing::info!(addr = %self.addr, "Gateway server listening");

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal(shutdown_rx))
            .await?;

        Ok(())
    }

    pub fn shutdown(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}

async fn root_handler() -> &'static str {
    "Pizza Gateway"
}

async fn proxy_handle(
    req: Request<axum::body::Body>,
    proxy: Arc<ReverseProxy>,
) -> Response<axum::body::Body> {
    let (parts, body) = req.into_parts();
    let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(b) => b,
        Err(_) => {
            return Response::builder()
                .status(axum::http::StatusCode::BAD_REQUEST)
                .body(axum::body::Body::from("Bad request"))
                .unwrap();
        }
    };

    let full_body = Full::new(body_bytes);
    let req = Request::from_parts(parts, full_body);
    let resp = proxy.handle(req).await;

    let (parts, body) = resp.into_parts();
    let body_bytes = match body.collect().await {
        Ok(c) => c.to_bytes(),
        Err(_) => Bytes::new(),
    };
    Response::from_parts(parts, axum::body::Body::from(body_bytes))
}

async fn shutdown_signal(rx: tokio::sync::oneshot::Receiver<()>) {
    let _ = rx.await;
}
