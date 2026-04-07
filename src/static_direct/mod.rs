use crate::error::{Result, StaticDirectError};
use axum::routing::get;
use axum::Router;
use dashmap::DashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;

pub struct StaticDirectServer {
    hosts: Arc<DashMap<String, PathBuf>>,
}

impl StaticDirectServer {
    pub fn new() -> Self {
        Self {
            hosts: Arc::new(DashMap::new()),
        }
    }

    pub fn register_host(&self, host: String, root_path: PathBuf) {
        self.hosts.insert(host, root_path);
    }

    pub fn unregister_host(&self, host: &str) {
        self.hosts.remove(host);
    }

    pub fn get_router_for_host(&self, host: &str) -> Result<Router> {
        if let Some(entry) = self.hosts.get(host) {
            let path = entry.value().clone();
            Ok(Router::new().nest_service("/", ServeDir::new(path)))
        } else {
            Err(StaticDirectError::HostNotRegistered(host.to_string()).into())
        }
    }

    pub fn registered_hosts(&self) -> Vec<String> {
        self.hosts.iter().map(|e| e.key().clone()).collect()
    }

    pub fn host_count(&self) -> usize {
        self.hosts.len()
    }
}

impl Default for StaticDirectServer {
    fn default() -> Self {
        Self::new()
    }
}
