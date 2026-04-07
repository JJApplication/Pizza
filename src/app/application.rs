use std::sync::Arc;
use tokio::sync::RwLock;
use crate::config::merge::MergedConfig;
use crate::config::server_config::DomainEntry;
use crate::error::Result;
use crate::gateway::GatewayManager;
use crate::gateway::ReverseProxy;
use crate::gateway::resolver::{Resolver, ServiceMapping};
use crate::gateway::proxy_cache::ProxyCache;
use crate::breaker::CircuitBreaker;
use crate::middleware::pre_handler::PreHandlerManager;
use crate::middleware::modifier::ModifierManager;
use crate::frontend::FrontendServer;
use crate::backend::BackendProxy;
use crate::stat::StatCollector;
use crate::health_probe::HealthProbe;
use crate::latency::LatencyMeasurement;
use crate::error_page::ErrorPageManager;
use crate::notifier::NotificationQueue;
use crate::wasm_plugin::PluginManager;
use crate::grpc::service::PizzaAppService;
use std::path::PathBuf;
use std::collections::HashMap;

pub struct PizzaApp {
    pub config: Arc<RwLock<MergedConfig>>,
    pub gateway_manager: Arc<RwLock<Option<GatewayManager>>>,
    pub reverse_proxy: Arc<ReverseProxy>,
    pub frontend_server: Arc<RwLock<Option<FrontendServer>>>,
    pub backend_proxy: Arc<BackendProxy>,
    pub stat_collector: Arc<StatCollector>,
    pub health_probe: Arc<HealthProbe>,
    pub latency_measurement: Arc<LatencyMeasurement>,
    pub error_page_manager: Arc<ErrorPageManager>,
    pub notification_queue: Arc<NotificationQueue>,
    pub plugin_manager: Arc<PluginManager>,
    pub grpc_service: Arc<PizzaAppService>,
}

impl PizzaApp {
    pub async fn new(config: MergedConfig) -> Result<Self> {
        let config = Arc::new(RwLock::new(config));

        let config_guard = config.read().await;

        let mut service_map: HashMap<String, Vec<crate::gateway::resolver::ServiceMapping>> = HashMap::new();
        for (domain, entry) in &config_guard.domain_map.domains {
            let mut mappings = Vec::new();
            if !entry.backend.is_empty() {
                mappings.push(ServiceMapping {
                    path_prefix: "/".to_string(),
                    backend: entry.backend.clone(),
                    port: None,
                    scheme: None,
                    rewrite: None,
                    strip_prefix: false,
                });
            }
            if !mappings.is_empty() {
                service_map.insert(domain.clone(), mappings);
            }
        }

        let custom_services = config_guard.app.pxy_custom_service.custom_service.clone();
        let custom_service_enabled = config_guard.app.pxy_custom_service.enable;
        let cache_config = config_guard.app.features.proxy_cache.clone();
        let plugin_root = if config_guard.app.plugin.enabled && !config_guard.app.plugin.root.is_empty() {
            Some(PathBuf::from(&config_guard.app.plugin.root))
        } else {
            None
        };
        drop(config_guard);

        let resolver = Arc::new(Resolver::new(service_map, None, custom_services, custom_service_enabled));

        let cache_ttl = if cache_config.cache_ttl > 0 { cache_config.cache_ttl } else { 300 };
        let cache_size = if cache_config.cache_size > 0 { cache_config.cache_size } else { 10000 };
        let proxy_cache = Arc::new(ProxyCache::new(cache_ttl, cache_size));

        let circuit_breaker = Arc::new(CircuitBreaker::new(5, 60));

        let pre_handler_manager = Arc::new(PreHandlerManager::new());
        let modifier_manager = Arc::new(ModifierManager::new());

        let reverse_proxy = Arc::new(ReverseProxy::new(
            resolver,
            proxy_cache,
            circuit_breaker,
            pre_handler_manager,
            modifier_manager,
            vec![],
            vec![],
        ));

        let plugin_manager = Arc::new(PluginManager::new(plugin_root)?);

        let grpc_service = Arc::new(PizzaAppService::new("config/config.json".to_string()));

        Ok(Self {
            config,
            gateway_manager: Arc::new(RwLock::new(None)),
            reverse_proxy,
            frontend_server: Arc::new(RwLock::new(None)),
            backend_proxy: Arc::new(BackendProxy::new()),
            stat_collector: Arc::new(StatCollector::new()),
            health_probe: Arc::new(HealthProbe::new(30, 120)),
            latency_measurement: Arc::new(LatencyMeasurement::new()),
            error_page_manager: Arc::new(ErrorPageManager::new()),
            notification_queue: Arc::new(NotificationQueue::new(None)),
            plugin_manager,
            grpc_service,
        })
    }

    pub async fn initialize(&self) -> Result<()> {
        tracing::info!("Initializing Pizza application");

        self.plugin_manager.load_plugins()?;

        self.backend_proxy.start_all().await?;

        self.error_page_manager.minify_all();

        tracing::info!("Pizza application initialized");
        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        tracing::info!("Starting Pizza gateway");

        let config = self.config.read().await;
        let server_configs: Vec<_> = config.app.servers.iter()
            .filter(|s| s.enabled)
            .map(|s| {
                let addr = if !s.host.is_empty() {
                    format!("{}:{}", s.host, s.port)
                } else {
                    format!("0.0.0.0:{}", s.port)
                };
                (s.name.clone(), addr, s.protocol == "https")
            })
            .collect();
        drop(config);

        let mut gateway_manager = self.gateway_manager.write().await;
        if gateway_manager.is_none() {
            *gateway_manager = Some(GatewayManager::new());
        }

        if let Some(ref mut mgr) = *gateway_manager {
            for (name, addr_str, tls) in server_configs {
                let addr: std::net::SocketAddr = addr_str.parse()
                    .map_err(|e: std::net::AddrParseError| crate::error::PizzaError::Gateway(crate::error::GatewayError::ServerStartFailed(e.to_string())))?;

                let server = crate::gateway::server::http_server::GatewayServer::new(
                    addr,
                    self.reverse_proxy.clone(),
                );

                mgr.add_server(server, crate::gateway::manager::ServerInfo {
                    name,
                    addr,
                    tls,
                    started: false,
                });
            }

            mgr.start_all().await?;
        }

        tracing::info!("Pizza gateway is running");
        Ok(())
    }

    pub fn print_status(&self) {
        println!("================================================");
        println!("          Pizza Gateway Status");
        println!("================================================");

        let (total, api, static_req, failures) = self.stat_collector.get_totals();
        println!("  Total Requests:   {}", total);
        println!("  API Requests:     {}", api);
        println!("  Static Requests:  {}", static_req);
        println!("  Failures:         {}", failures);
        println!("  WASM Plugins:     {}", self.plugin_manager.plugin_count());
        println!("================================================");
    }
}
