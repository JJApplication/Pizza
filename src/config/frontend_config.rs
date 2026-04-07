use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FrontendConfig {
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub balancer: String,
    #[serde(default)]
    pub exp_fast_connect: ExpFastConnectConfig,
    #[serde(default)]
    pub cache: FrontendCacheConfig,
    #[serde(default)]
    pub internal_flag: String,
    #[serde(default)]
    pub internal_local_flag: String,
    #[serde(default)]
    pub internal_backend_flag: String,
    #[serde(default)]
    pub cache_header: String,
    #[serde(default)]
    pub error: FrontendErrorConfig,
    #[serde(default)]
    pub custom_headers: Vec<CustomHeaderEntry>,
    #[serde(default)]
    pub servers: Vec<FrontendServerEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExpFastConnectConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub http2: Http2ClientConfig,
    #[serde(default)]
    pub http3: Http3ClientConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Http2ClientConfig {
    #[serde(default)]
    pub read_timeout: u64,
    #[serde(default)]
    pub write_timeout: u64,
    #[serde(default)]
    pub idle_timeout: u64,
    #[serde(default)]
    pub read_header_timeout: u64,
    #[serde(default)]
    pub max_header_bytes: usize,
    #[serde(default)]
    pub keep_alive: u64,
    #[serde(default)]
    pub max_handlers: u32,
    #[serde(default)]
    pub max_concurrent_streams: u32,
    #[serde(default)]
    pub max_upload_buffer_per_connection: usize,
    #[serde(default)]
    pub max_upload_buffer_per_stream: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Http3ClientConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub cert_file: String,
    #[serde(default)]
    pub key_file: String,
    #[serde(default)]
    pub max_connections: usize,
    #[serde(default)]
    pub idle_timeout: u64,
    #[serde(default)]
    pub keep_alive: u64,
    #[serde(default)]
    pub insecure_skip_verify: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FrontendCacheConfig {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub dir: String,
    #[serde(default)]
    pub expire: u64,
    #[serde(default)]
    pub matcher: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FrontendErrorConfig {
    #[serde(default)]
    pub not_found: String,
    #[serde(default)]
    pub internal_server_error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomHeaderEntry {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendServerEntry {
    #[serde(default)]
    #[serde(rename = "type")]
    pub server_type: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub root: String,
    #[serde(default)]
    pub index: String,
    #[serde(default)]
    pub try_file: String,
    #[serde(default)]
    pub access: bool,
    #[serde(default)]
    pub compress: bool,
    #[serde(default)]
    pub alias: HashMap<String, String>,
    #[serde(default)]
    pub backends: Vec<BackendRoute>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackendRoute {
    #[serde(default)]
    pub api: String,
    #[serde(default)]
    pub service: String,
    #[serde(default)]
    pub use_rewrite: bool,
}
