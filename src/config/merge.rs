use crate::config::app_config::AppConfig;
use crate::config::backend_config::BackendConfig;
use crate::config::frontend_config::FrontendConfig;
use crate::config::server_config::DomainMapConfig;
use crate::error::{ConfigError, Result};
use std::fs;

#[derive(Debug, Clone)]
pub struct MergedConfig {
    pub app: AppConfig,
    pub backend: BackendConfig,
    pub frontend: FrontendConfig,
    pub domain_map: DomainMapConfig,
}

pub fn merge_configs(
    app_config: AppConfig,
    backend_file: Option<&str>,
    frontend_file: Option<&str>,
    domain_map_file: Option<&str>,
) -> Result<MergedConfig> {
    let backend = if let Some(path) = backend_file {
        load_backend_config(path)?
    } else {
        BackendConfig::default()
    };

    let frontend = if let Some(path) = frontend_file {
        load_frontend_config(path)?
    } else {
        FrontendConfig::default()
    };

    let domain_map = if let Some(path) = domain_map_file {
        load_domain_map(path)?
    } else {
        DomainMapConfig::default()
    };

    Ok(MergedConfig {
        app: app_config,
        backend,
        frontend,
        domain_map,
    })
}

fn load_backend_config(path: &str) -> Result<BackendConfig> {
    let content = fs::read_to_string(path)
        .map_err(|e| ConfigError::LoadFailed(format!("backend config {}: {}", path, e)))?;

    let ext = path.rsplit('.').next().unwrap_or("json");
    match ext {
        "json" => serde_json::from_str(&content)
            .map_err(|e| ConfigError::ParseFailed(format!("backend config: {}", e)))
            .map_err(Into::into),
        _ => serde_json::from_str(&content)
            .map_err(|e| ConfigError::ParseFailed(format!("backend config: {}", e)))
            .map_err(Into::into),
    }
}

fn load_frontend_config(path: &str) -> Result<FrontendConfig> {
    let content = fs::read_to_string(path)
        .map_err(|e| ConfigError::LoadFailed(format!("frontend config {}: {}", path, e)))?;

    serde_json::from_str(&content)
        .map_err(|e| ConfigError::ParseFailed(format!("frontend config: {}", e)))
        .map_err(Into::into)
}

fn load_domain_map(path: &str) -> Result<DomainMapConfig> {
    let content = fs::read_to_string(path)
        .map_err(|e| ConfigError::LoadFailed(format!("domain map {}: {}", path, e)))?;

    serde_json::from_str(&content)
        .map_err(|e| ConfigError::ParseFailed(format!("domain map: {}", e)))
        .map_err(Into::into)
}
