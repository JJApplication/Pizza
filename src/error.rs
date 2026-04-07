use thiserror::Error;

#[derive(Error, Debug)]
pub enum PizzaError {
    #[error("config error: {0}")]
    Config(#[from] ConfigError),

    #[error("gateway error: {0}")]
    Gateway(#[from] GatewayError),

    #[error("proxy error: {0}")]
    Proxy(#[from] ProxyError),

    #[error("middleware error: {0}")]
    Middleware(#[from] MiddlewareError),

    #[error("flow control error: {0}")]
    FlowControl(#[from] FlowControlError),

    #[error("circuit breaker error: {0}")]
    CircuitBreaker(#[from] CircuitBreakerError),

    #[error("frontend error: {0}")]
    Frontend(#[from] FrontendError),

    #[error("backend error: {0}")]
    Backend(#[from] BackendError),

    #[error("grpc error: {0}")]
    Grpc(#[from] GrpcError),

    #[error("wasm plugin error: {0}")]
    WasmPlugin(#[from] WasmPluginError),

    #[error("stat error: {0}")]
    Stat(#[from] StatError),

    #[error("health probe error: {0}")]
    HealthProbe(#[from] HealthProbeError),

    #[error("latency error: {0}")]
    Latency(#[from] LatencyError),

    #[error("notifier error: {0}")]
    Notifier(#[from] NotifierError),

    #[error("static direct error: {0}")]
    StaticDirect(#[from] StaticDirectError),

    #[error("error page error: {0}")]
    ErrorPage(#[from] ErrorPageError),

    #[error("autocert error: {0}")]
    AutoCert(#[from] AutoCertError),

    #[error("internal flag: {flag} - {message}")]
    InternalFlag { flag: String, message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("application shutdown")]
    Shutdown,

    #[error("unknown error: {0}")]
    Other(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("failed to load config file: {0}")]
    LoadFailed(String),

    #[error("failed to parse config: {0}")]
    ParseFailed(String),

    #[error("failed to merge config: {0}")]
    MergeFailed(String),

    #[error("missing required config: {0}")]
    MissingField(String),

    #[error("invalid config value: {0}")]
    InvalidValue(String),
}

#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("failed to start server: {0}")]
    ServerStartFailed(String),

    #[error("failed to stop server: {0}")]
    ServerStopFailed(String),

    #[error("failed to create transport: {0}")]
    TransportError(String),

    #[error("server not found: {0}")]
    ServerNotFound(String),

    #[error("manager already started")]
    ManagerAlreadyStarted,

    #[error("manager not started")]
    ManagerNotStarted,
}

#[derive(Error, Debug)]
pub enum ProxyError {
    #[error("failed to resolve target: {0}")]
    ResolutionFailed(String),

    #[error("connection refused: {0}")]
    ConnectionRefused(String),

    #[error("upstream error: {0}")]
    UpstreamError(String),

    #[error("request rejected by pre-handler: {0}")]
    PreHandlerRejected(String),

    #[error("request rejected by circuit breaker")]
    CircuitBreakerOpen,

    #[error("request rate limited")]
    RateLimited,
}

#[derive(Error, Debug)]
pub enum MiddlewareError {
    #[error("auth failed: {0}")]
    AuthFailed(String),

    #[error("rate limit exceeded")]
    RateLimitExceeded,

    #[error("domain not allowed: {0}")]
    DomainNotAllowed(String),

    #[error("sanitization error: {0}")]
    SanitizationError(String),

    #[error("image hotlink blocked")]
    ImageHotlinkBlocked,
}

#[derive(Error, Debug)]
pub enum FlowControlError {
    #[error("rate limit exceeded for rule: {0}")]
    RateLimitExceeded(String),

    #[error("invalid rule configuration: {0}")]
    InvalidRule(String),
}

#[derive(Error, Debug)]
pub enum CircuitBreakerError {
    #[error("circuit open for domain: {0}")]
    CircuitOpen(String),

    #[error("invalid circuit configuration: {0}")]
    InvalidConfig(String),
}

#[derive(Error, Debug)]
pub enum FrontendError {
    #[error("file not found: {0}")]
    FileNotFound(String),

    #[error("failed to read file: {0}")]
    FileReadFailed(String),

    #[error("static assets not configured")]
    AssetsNotConfigured,
}

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("backend server not found: {0}")]
    ServerNotFound(String),

    #[error("failed to start backend: {0}")]
    StartFailed(String),

    #[error("TCP proxy error: {0}")]
    TcpProxyError(String),

    #[error("transparent proxy error: {0}")]
    TransparentProxyError(String),
}

#[derive(Error, Debug)]
pub enum GrpcError {
    #[error("gRPC server error: {0}")]
    ServerError(String),

    #[error("gRPC client error: {0}")]
    ClientError(String),

    #[error("connection pool exhausted")]
    PoolExhausted,

    #[error("address not in whitelist: {0}")]
    AddressNotWhitelisted(String),
}

#[derive(Error, Debug)]
pub enum WasmPluginError {
    #[error("failed to load plugin: {0}")]
    LoadFailed(String),

    #[error("plugin execution error: {0}")]
    ExecutionError(String),

    #[error("plugin memory error: {0}")]
    MemoryError(String),

    #[error("plugin rejected request: status={status}, body={body}")]
    RequestRejected { status: u16, body: String },
}

#[derive(Error, Debug)]
pub enum StatError {
    #[error("failed to record stat: {0}")]
    RecordFailed(String),

    #[error("persistence error: {0}")]
    PersistenceError(String),

    #[error("geo lookup failed: {0}")]
    GeoLookupFailed(String),
}

#[derive(Error, Debug)]
pub enum HealthProbeError {
    #[error("probe failed for domain: {0}")]
    ProbeFailed(String),

    #[error("domain marked as dead: {0}")]
    DomainDead(String),
}

#[derive(Error, Debug)]
pub enum LatencyError {
    #[error("measurement failed: {0}")]
    MeasurementFailed(String),

    #[error("backend unreachable: {0}")]
    BackendUnreachable(String),
}

#[derive(Error, Debug)]
pub enum NotifierError {
    #[error("failed to send notification: {0}")]
    SendFailed(String),

    #[error("queue error: {0}")]
    QueueError(String),
}

#[derive(Error, Debug)]
pub enum StaticDirectError {
    #[error("static direct server error: {0}")]
    ServerError(String),

    #[error("host not registered: {0}")]
    HostNotRegistered(String),
}

#[derive(Error, Debug)]
pub enum ErrorPageError {
    #[error("failed to load error page: {0}")]
    LoadFailed(String),

    #[error("invalid error page template: {0}")]
    InvalidTemplate(String),
}

#[derive(Error, Debug)]
pub enum AutoCertError {
    #[error("failed to obtain certificate: {0}")]
    CertObtainFailed(String),

    #[error("ACME challenge failed: {0}")]
    ChallengeFailed(String),

    #[error("certificate expired")]
    CertExpired,
}

pub type Result<T> = std::result::Result<T, PizzaError>;
