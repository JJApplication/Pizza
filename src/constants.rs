pub const APP_NAME: &str = "pizza";
pub const DESCRIPTION: &str = "A next-generation API gateway and static proxy server";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const PROXY_MODE_HTTP: &str = "http";
pub const PROXY_MODE_FASTHTTP: &str = "fasthttp";

pub const NET_IO_HTTP: &str = "http";
pub const NET_IO_NBIO: &str = "nbio";

pub const SCHEME_HTTP: &str = "http";
pub const SCHEME_HTTPS: &str = "https";
pub const SCHEME_GRPC: &str = "grpc";
pub const SCHEME_GRPC_WEB: &str = "grpc-web";
pub const SCHEME_INTERNAL: &str = "sandwich";

pub const HEADER_TRACE_ID: &str = "X-Gateway-Trace-Id";
pub const HEADER_INTERNAL_FLAG: &str = "X-Proxy-Internal-Flag";
pub const HEADER_INTERNAL_HOST: &str = "X-Proxy-Internal-Host";
pub const HEADER_DIRECT_ACCESS: &str = "X-Direct-Access";

pub const INTERNAL_FLAG_GRPC: &str = "grpc";
pub const INTERNAL_FLAG_GRPC_WEB: &str = "grpc-web";
pub const INTERNAL_FLAG_STATIC_DIRECT: &str = "static-direct";
pub const INTERNAL_FLAG_SERVICE_STOPPED: &str = "service-stopped";

pub const ERR_BUCKET_LIMIT: &str = "SandwichBucketLimit";
pub const ERR_REQ_LIMIT: &str = "SandwichReqLimit";
pub const ERR_DOMAIN_NOT_ALLOW: &str = "SandwichDomainNotAllow";
pub const ERR_PRE_AUTH_FAILED: &str = "SandwichPreAuthFailed";
pub const ERR_SERVICE_STOPPED: &str = "SandwichServiceStopped";
pub const ERR_BACKEND_ERROR: &str = "SandwichBackendError";
pub const ERR_PROXY_CONNECT: &str = "SandwichProxyConnectError";

pub const DEFAULT_GRPC_PORT: u16 = 50051;
pub const DEFAULT_API_PORT: u16 = 8080;
pub const DEFAULT_STAT_PORT: u16 = 9090;
pub const DEFAULT_LATENCY_PORT: u16 = 8081;

pub const SHUTDOWN_TIMEOUT_SECS: u64 = 30;

pub const ENV_CONFIG_PATH: &str = "PIZZA_CONFIG";
pub const ENV_ENVIRONMENT: &str = "PIZZA_ENV";
