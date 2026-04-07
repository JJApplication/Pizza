use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackendConfig {
    #[serde(flatten)]
    pub services: HashMap<String, BackendService>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendService {
    #[serde(default)]
    #[serde(rename = "type")]
    pub server_type: String,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub root: Option<String>,
    #[serde(default)]
    pub protocol: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub health_check: Option<HealthCheckConfig>,
    #[serde(default)]
    pub tls: Option<BackendTlsConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthCheckConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub interval: u64,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackendTlsConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub skip_verify: bool,
}
