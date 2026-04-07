use axum::Router;
use tower_http::services::ServeDir;
use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct RouteConfig {
    pub path: String,
    pub status_code: Option<u16>,
    pub body: Option<String>,
    pub headers: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct BasicAuthConfig {
    pub username: String,
    pub password: String,
}

pub struct HttpBackendServer {
    pub name: String,
    pub root_path: Option<PathBuf>,
    pub routes: Vec<RouteConfig>,
    pub basic_auth: Option<BasicAuthConfig>,
}

impl HttpBackendServer {
    pub fn new(name: String, root_path: Option<PathBuf>, routes: Vec<RouteConfig>, basic_auth: Option<BasicAuthConfig>) -> Self {
        Self { name, root_path, routes, basic_auth }
    }

    pub fn into_router(self) -> Router {
        let mut router = Router::new();

        for route in &self.routes {
            let status = route.status_code.unwrap_or(200);
            let body = route.body.clone().unwrap_or_default();
            let path = route.path.clone();
            router = router.route(&path, axum::routing::get(move || {
                let body = body.clone();
                async move {
                    (http::StatusCode::from_u16(status).unwrap_or(http::StatusCode::OK), body)
                }
            }));
        }

        if let Some(ref root) = self.root_path {
            router = router.nest_service("/", ServeDir::new(root));
        }

        router
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct BackendProxy {
    http_servers: Arc<RwLock<HashMap<String, HttpBackendServer>>>,
    tcp_proxy_addrs: Vec<String>,
    transparent_addrs: Vec<String>,
}

impl BackendProxy {
    pub fn new() -> Self {
        Self {
            http_servers: Arc::new(RwLock::new(HashMap::new())),
            tcp_proxy_addrs: Vec::new(),
            transparent_addrs: Vec::new(),
        }
    }

    pub async fn register_http_server(&self, name: String, server: HttpBackendServer) {
        self.http_servers.write().await.insert(name, server);
    }

    pub async fn remove_http_server(&self, name: &str) {
        self.http_servers.write().await.remove(name);
    }

    pub async fn get_http_router(&self) -> Router {
        let mut router = Router::new();
        let servers = self.http_servers.read().await;
        let server_names: Vec<String> = servers.keys().cloned().collect();

        for name in server_names {
            if let Some(server) = servers.get(&name) {
                if server.root_path.is_some() || !server.routes.is_empty() {
                    let root = server.root_path.clone();
                    let routes = server.routes.clone();
                    let server_name = name.clone();

                    let mut sub_router = Router::new();
                    for route in &routes {
                        let status = route.status_code.unwrap_or(200);
                        let body = route.body.clone().unwrap_or_default();
                        let path = route.path.clone();
                        sub_router = sub_router.route(&path, axum::routing::get(move || {
                            let body = body.clone();
                            async move {
                                (http::StatusCode::from_u16(status).unwrap_or(http::StatusCode::OK), body)
                            }
                        }));
                    }

                    if let Some(r) = root {
                        sub_router = sub_router.nest_service("/", ServeDir::new(r));
                    }

                    router = router.nest(&format!("/{}", server_name), sub_router);
                }
            }
        }
        router
    }

    pub fn add_tcp_proxy(&mut self, addr: String) {
        self.tcp_proxy_addrs.push(addr);
    }

    pub fn add_transparency(&mut self, addr: String) {
        self.transparent_addrs.push(addr);
    }

    pub async fn start_all(&self) -> Result<()> {
        let servers = self.http_servers.read().await;
        for name in servers.keys() {
            tracing::info!(name = %name, "Starting backend HTTP server");
        }
        Ok(())
    }

    pub async fn stop_all(&self) -> Result<()> {
        let servers = self.http_servers.read().await;
        for name in servers.keys() {
            tracing::info!(name = %name, "Stopping backend HTTP server");
        }
        Ok(())
    }

    pub fn tcp_proxy_count(&self) -> usize {
        self.tcp_proxy_addrs.len()
    }

    pub fn transparent_count(&self) -> usize {
        self.transparent_addrs.len()
    }
}

impl Default for BackendProxy {
    fn default() -> Self {
        Self::new()
    }
}
