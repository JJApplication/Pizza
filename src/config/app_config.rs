use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub pxy_backend_file: Option<String>,
    #[serde(default)]
    pub pxy_frontend_file: Option<String>,
    #[serde(default)]
    pub domain_map: Option<String>,
    #[serde(default)]
    pub plugin: PluginConfig,
    #[serde(default)]
    pub proxy: ProxyConfig,
    #[serde(default)]
    pub error_config: ErrorConfig,
    #[serde(default)]
    pub servers: Vec<ServerConfig>,
    #[serde(default)]
    pub middleware: MiddlewareConfig,
    #[serde(default)]
    pub features: FeatureConfig,
    #[serde(default)]
    pub database: DatabaseConfig,
    #[serde(default)]
    pub security: SecurityConfig,
    #[serde(default)]
    pub proxy_header: ProxyHeaderConfig,
    #[serde(default)]
    pub log: LogConfig,
    #[serde(default)]
    pub module: Option<serde_json::Value>,
    #[serde(default)]
    pub api_server_config: ApiServerConfig,
    #[serde(default)]
    pub stat: StatConfig,
    #[serde(default)]
    pub custom_header: HashMap<String, String>,
    #[serde(default)]
    pub syncer: SyncerConfig,
    #[serde(default)]
    pub debug: bool,
    #[serde(default)]
    pub pprof: PprofConfig,
    #[serde(default)]
    pub max_cores: u32,
    #[serde(default)]
    pub pxy_backend: serde_json::Value,
    #[serde(default)]
    pub pxy_frontend: serde_json::Value,
    #[serde(default)]
    pub exp_config: serde_json::Value,
    #[serde(default)]
    pub pxy_custom_service: PxyCustomServiceConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            pxy_backend_file: None,
            pxy_frontend_file: None,
            domain_map: None,
            plugin: PluginConfig::default(),
            proxy: ProxyConfig::default(),
            error_config: ErrorConfig::default(),
            servers: vec![],
            middleware: MiddlewareConfig::default(),
            features: FeatureConfig::default(),
            database: DatabaseConfig::default(),
            security: SecurityConfig::default(),
            proxy_header: ProxyHeaderConfig::default(),
            log: LogConfig::default(),
            module: None,
            api_server_config: ApiServerConfig::default(),
            stat: StatConfig::default(),
            custom_header: HashMap::new(),
            syncer: SyncerConfig::default(),
            debug: false,
            pprof: PprofConfig::default(),
            max_cores: 0,
            pxy_backend: serde_json::Value::Null,
            pxy_frontend: serde_json::Value::Null,
            exp_config: serde_json::Value::Null,
            pxy_custom_service: PxyCustomServiceConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProxyConfig {
    #[serde(default)]
    pub flush_interval: i64,
    #[serde(default = "default_buf_size")]
    pub buf_size: usize,
    #[serde(default)]
    pub transport: String,
    #[serde(default)]
    pub proxy_mode: String,
    #[serde(default)]
    pub net_io: String,
    #[serde(default = "default_max_conns")]
    pub max_conns_per_host: usize,
    #[serde(default = "default_idle_timeout")]
    pub idle_conn_timeout: u64,
}

fn default_buf_size() -> usize {
    32768
}
fn default_max_conns() -> usize {
    100
}
fn default_idle_timeout() -> u64 {
    60
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub use_http2: bool,
    #[serde(default)]
    pub protocol: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub max_request_body: usize,
    #[serde(default)]
    pub domains: Vec<ServerDomainConfig>,
    #[serde(default)]
    pub tls: Option<TlsConfig>,
    #[serde(default)]
    pub http2: Option<Http2Config>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerDomainConfig {
    #[serde(default)]
    pub domains: Vec<String>,
    #[serde(default)]
    pub auto_redirect: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    #[serde(default)]
    pub min_version: String,
    #[serde(default)]
    pub cert_map: HashMap<String, CertEntry>,
    #[serde(default)]
    pub auto_tls: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertEntry {
    #[serde(default)]
    pub domains: Vec<String>,
    #[serde(default)]
    pub cert_file: String,
    #[serde(default)]
    pub key_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Http2Config {
    #[serde(default)]
    pub max_concurrent_streams: u32,
    #[serde(default)]
    pub max_handlers: u32,
    #[serde(default)]
    pub idle_timeout: u64,
    #[serde(default)]
    pub read_idle_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MiddlewareConfig {
    #[serde(default)]
    pub cors: CorsConfig,
    #[serde(default)]
    pub trace: TraceConfig,
    #[serde(default)]
    pub secure_header: bool,
    #[serde(default)]
    pub gzip: GzipConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CorsConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub header: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TraceConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub trace_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GzipConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_gzip_level")]
    pub level: u32,
    #[serde(default)]
    pub types: Vec<String>,
    #[serde(default = "default_gzip_threshold")]
    pub threshold: usize,
}

fn default_gzip_level() -> u32 {
    6
}
fn default_gzip_threshold() -> usize {
    1024
}

impl Default for GzipConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            level: default_gzip_level(),
            types: vec![],
            threshold: default_gzip_threshold(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureConfig {
    #[serde(default)]
    pub flow_control: FlowControlConfig,
    #[serde(default)]
    pub websocket: WebsocketConfig,
    #[serde(default)]
    pub proxy_cache: ProxyCacheConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FlowControlConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub global_limit: Option<FlowLimit>,
    #[serde(default)]
    pub rules: Vec<FlowRule>,
    #[serde(default)]
    pub recording: FlowRecordingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowLimit {
    #[serde(default)]
    pub requests: u64,
    #[serde(default)]
    pub window: String,
    #[serde(default)]
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowRule {
    pub name: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub priority: u32,
    #[serde(default)]
    pub match_type: String,
    #[serde(default)]
    pub match_value: String,
    #[serde(default)]
    pub header_key: String,
    #[serde(default)]
    pub limits: Vec<FlowLimit>,
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FlowRecordingConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub record_blocked: bool,
    #[serde(default)]
    pub record_allowed: bool,
    #[serde(default)]
    pub storage_type: String,
    #[serde(default)]
    pub retention_period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebsocketConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProxyCacheConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub cache_size: u64,
    #[serde(default)]
    pub cache_ttl: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseConfig {
    #[serde(default)]
    pub mongo: MongoConfig,
    #[serde(default)]
    pub influx: InfluxConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MongoConfig {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub database: String,
    #[serde(default)]
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfluxConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub org: String,
    #[serde(default)]
    pub bucket: String,
    #[serde(default)]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityConfig {
    #[serde(default)]
    pub strict_mode: bool,
    #[serde(default)]
    pub allow_ips: Vec<String>,
    #[serde(default)]
    pub deny_ips: Vec<String>,
    #[serde(default)]
    pub rate_limit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProxyHeaderConfig {
    #[serde(default)]
    pub trace_id: String,
    #[serde(default)]
    pub frontend_host_header: String,
    #[serde(default)]
    pub backend_header: String,
    #[serde(default)]
    pub proxy_app: String,
    #[serde(default)]
    pub forward_host_header: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LogConfig {
    #[serde(default)]
    pub log_level: String,
    #[serde(default)]
    pub log_file: String,
    #[serde(default)]
    pub color: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiServerConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub http2: Option<ApiHttp2Config>,
    #[serde(default)]
    pub bblot: Option<BblotConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiHttp2Config {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub insecure: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BblotConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatConfig {
    #[serde(default)]
    pub db_file: String,
    #[serde(default)]
    pub use_db: bool,
    #[serde(default)]
    pub compatible: bool,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub enable_stat: bool,
    #[serde(default)]
    pub sync_duration: u64,
    #[serde(default)]
    pub save_duration: u64,
    #[serde(default)]
    pub save_file: String,
    #[serde(default)]
    pub geo_file: String,
    #[serde(default)]
    pub domain_file: String,
    #[serde(default)]
    pub geo_db: String,
    #[serde(default)]
    pub sequence: StatSequenceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatSequenceConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncerConfig {
    #[serde(default)]
    pub job_sync_domains: u64,
    #[serde(default)]
    pub job_sync_domain_ports: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PprofConfig {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub root: String,
    #[serde(default)]
    pub plugins: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ErrorConfig {
    #[serde(default)]
    pub error_mode: String,
    #[serde(default)]
    pub enable_page_cache: bool,
    #[serde(default)]
    pub error_page: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PxyCustomServiceConfig {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub custom_service: Vec<CustomServiceConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomServiceConfig {
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub upstream: Vec<Upstream>,
    #[serde(default)]
    pub apis: Vec<APIBackendConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Upstream {
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct APIBackendConfig {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub api: String,
    #[serde(default)]
    pub target_service: String,
    #[serde(default)]
    pub target: String,
    #[serde(default)]
    pub target_host: String,
    #[serde(default)]
    pub target_port: i32,
    #[serde(default)]
    pub use_rewrite: bool,
    #[serde(default)]
    pub rewrite: String,
}
