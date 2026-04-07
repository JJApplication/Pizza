use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;
use std::path::PathBuf;
use std::collections::HashMap;
use crate::config::backend_config::{RouteConfig, BasicAuthConfig};

pub struct HttpBackendServer {
    name: String,
    root_path: Option<PathBuf>,
    routes: Vec<RouteConfig>,
    basic_auth: Option<BasicAuthConfig>,
}

impl HttpBackendServer {
    pub fn new(name: String, root_path: Option<PathBuf>, routes: Vec<RouteConfig>, basic_auth: Option<BasicAuthConfig>) -> Self {
        Self { name, root_path, routes, basic_auth }
    }

    pub fn root_path(&self) -> Option<&PathBuf> {
        self.root_path.as_ref()
    }

    pub fn into_router(self) -> Router {
        let mut router = Router::new();

        for route in &self.routes {
            let status = route.status_code.unwrap_or(200);
            let body = route.body.clone().unwrap_or_default();
            let path = route.path.clone();
            router = router.route(&path, get(move || {
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
