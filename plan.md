# Pizza - Rust Refactoring of Hamburger API Gateway

## Scope

**Included**: Core gateway, reverse proxy, static file serving, backend proxy (HTTP/TCP/Transparent), middleware chain, flow control, circuit breaker, gRPC management plane, WASM plugins, statistics, health probes, latency measurement, ACME cert management

**Excluded**: VPN/Trojan/DNS/AnyTLS/WebDAV experimental servers

## Tech Stack

| Layer | Choice | Notes |
|---|---|---|
| HTTP | axum + hyper | Async, high-performance |
| HTTP/3 | quinn | QUIC/HTTP3 |
| Async | tokio | Standard runtime |
| Config | serde + serde_json + toml | Multi-format |
| Logging | tracing + tracing-subscriber | Replaces zerolog |
| Cache | moka | Replaces bigcache |
| DB | sqlx (SQLite) + mongodb | Replaces gorm + mongo-driver |
| gRPC | tonic | Replaces grpc-go |
| WASM | wasmtime | Replaces wazero |
| CLI | clap | Replaces cobra |
| TLS | rustls + acme2-eab | Replaces utls |
| Errors | thiserror + anyhow | Standardized |

## Project Structure

```
pizza/
├── Cargo.toml
├── build.rs
├── plan.md
├── proto/
│   └── service.proto
└── src/
    ├── main.rs
    ├── lib.rs
    ├── error.rs
    ├── constants.rs
    ├── config/
    ├── app/
    ├── gateway/
    ├── middleware/
    ├── flow_control/
    ├── breaker/
    ├── frontend/
    ├── backend/
    ├── static_direct/
    ├── grpc/
    ├── wasm_plugin/
    ├── stat/
    ├── health_probe/
    ├── latency/
    ├── notifier/
    ├── error_page/
    ├── initialize/
    └── utils/
```

## Implementation Phases

### Phase 1: Infrastructure
- Cargo.toml, build.rs, proto
- constants.rs, error.rs
- utils/ (time, trace_id, header, defaults)
- config/ (loader, merge, all config structs)
- initialize/ (priority registry)

### Phase 2: Gateway Core
- gateway/proxy.rs (reverse proxy with axum+hyper)
- gateway/director.rs (request routing brain)
- gateway/transport.rs (multi-protocol: HTTP/H2C/H3)
- gateway/resolver.rs + balancer.rs + proxy_cache.rs
- gateway/manager.rs (multi-server lifecycle)
- gateway/server/ (http_server, h3_server)

### Phase 3: Middleware + Flow Control
- middleware/pre_handler/ (auth, sanitizer, domain_ctrl, rate_limiter, image_protect)
- middleware/modifier/ (trace_id, secure_header, cors, gzip, custom_header, fail_response)
- flow_control/ (fixed_window, leaky_bucket, token_bucket, sliding_window)
- breaker/ (per-domain circuit breaker)

### Phase 4: Frontend + Backend Proxy
- frontend/ (static file server, file cache, SPA fallback)
- backend/ (HTTP server, TCP proxy, transparent proxy)
- static_direct/ (static bypass server)

### Phase 5: gRPC + Advanced Features
- grpc/ (server, service impl, HTTP→gRPC bridge, gRPC-Web proxy)
- wasm_plugin/ (WASM plugin system with wasmtime)
- stat/ (metrics collector, GeoIP)
- health_probe/ (per-domain health checks)
- latency/ (latency measurement)
- notifier/ (email notifications)
- error_page/ (custom error pages)

### Phase 6: App Assembly + CLI
- app/application.rs (PizzaApp main struct)
- app/lifecycle.rs (signal handling, graceful shutdown)
- main.rs (clap CLI: run/generate/test/reload)

## Key Design Mappings

| Go Pattern | Rust Replacement |
|---|---|
| sync.RWMutex | parking_lot::RwLock / tokio::sync::RwLock |
| goroutine | tokio::spawn |
| sync.WaitGroup | tokio::task::JoinSet |
| sync.Once | std::sync::OnceLock |
| context.Context | CancellationToken / AbortHandle |
| httputil.ReverseProxy | axum::Router + hyper::client::Client |
| gin middleware | tower::Layer / axum::middleware |
| bigcache | moka::future::Cache |
| zerolog | tracing |
| cobra | clap |
| wazero | wasmtime |
