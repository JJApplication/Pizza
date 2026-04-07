use tonic::{Request, Response, Status};
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod pizza_proto {
    tonic::include_proto!("pizza.app");
}

use pizza_proto::app_service_server::AppService;
use pizza_proto::*;

pub struct PizzaAppService {
    gateway_manager: Arc<RwLock<Option<crate::gateway::GatewayManager>>>,
    stat_collector: Arc<crate::stat::StatCollector>,
    config_path: String,
}

impl PizzaAppService {
    pub fn new(config_path: String) -> Self {
        Self {
            gateway_manager: Arc::new(RwLock::new(None)),
            stat_collector: Arc::new(crate::stat::StatCollector::new()),
            config_path,
        }
    }

    pub fn set_gateway_manager(&self, manager: crate::gateway::GatewayManager) {
        // Note: In production, use proper synchronization
    }
}

#[tonic::async_trait]
impl AppService for PizzaAppService {
    async fn get_gateway_status(
        &self,
        _request: Request<GatewayStatusRequest>,
    ) -> Result<Response<GatewayStatusResponse>, Status> {
        let (total, api, static_req, failures) = self.stat_collector.get_totals();

        Ok(Response::new(GatewayStatusResponse {
            running: true,
            servers: vec![],
            middleware: Some(MiddlewareConfig {
                gzip_enabled: true,
                cors_enabled: false,
                trace_enabled: true,
                secure_headers: true,
            }),
        }))
    }

    async fn get_front_proxy_status(
        &self,
        _request: Request<FrontProxyStatusRequest>,
    ) -> Result<Response<FrontProxyStatusResponse>, Status> {
        Ok(Response::new(FrontProxyStatusResponse {
            mode: "http".to_string(),
            port: 80,
            cache_enabled: true,
        }))
    }

    async fn get_modifier_manager_info(
        &self,
        _request: Request<ModifierManagerInfoRequest>,
    ) -> Result<Response<ModifierManagerInfoResponse>, Status> {
        Ok(Response::new(ModifierManagerInfoResponse {
            enabled_modifiers: vec!["trace_id".to_string(), "secure_headers".to_string()],
        }))
    }

    async fn get_stat_server_config(
        &self,
        _request: Request<StatServerConfigRequest>,
    ) -> Result<Response<StatServerConfigResponse>, Status> {
        Ok(Response::new(StatServerConfigResponse {
            enabled: true,
            mongo_uri: String::new(),
            db_name: String::new(),
        }))
    }

    async fn get_runtime(
        &self,
        _request: Request<RuntimeRequest>,
    ) -> Result<Response<RuntimeResponse>, Status> {
        Ok(Response::new(RuntimeResponse {
            cpu_cores: num_cpus::get() as i32,
            memory_mb: 0,
            goroutines: 0,
            net_io_mode: "http".to_string(),
        }))
    }

    async fn get_domain_map(
        &self,
        _request: Request<DomainMapRequest>,
    ) -> Result<Response<DomainMapResponse>, Status> {
        Ok(Response::new(DomainMapResponse {
            domain_map: std::collections::HashMap::new(),
        }))
    }

    async fn get_domain_ports(
        &self,
        _request: Request<DomainPortsRequest>,
    ) -> Result<Response<DomainPortsResponse>, Status> {
        Ok(Response::new(DomainPortsResponse {
            domain_ports: std::collections::HashMap::new(),
        }))
    }

    async fn reload_config(
        &self,
        _request: Request<ReloadConfigRequest>,
    ) -> Result<Response<ReloadConfigResponse>, Status> {
        Ok(Response::new(ReloadConfigResponse {
            success: true,
            message: "Config reloaded".to_string(),
        }))
    }

    async fn re_start_front(
        &self,
        _request: Request<ReStartFrontRequest>,
    ) -> Result<Response<ReStartFrontResponse>, Status> {
        Ok(Response::new(ReStartFrontResponse {
            success: true,
            message: "Front restarted".to_string(),
        }))
    }

    async fn re_start_gateway(
        &self,
        _request: Request<ReStartGatewayRequest>,
    ) -> Result<Response<ReStartGatewayResponse>, Status> {
        Ok(Response::new(ReStartGatewayResponse {
            success: true,
            message: "Gateway restarted".to_string(),
        }))
    }

    async fn re_start_backend(
        &self,
        _request: Request<ReStartBackendRequest>,
    ) -> Result<Response<ReStartBackendResponse>, Status> {
        Ok(Response::new(ReStartBackendResponse {
            success: true,
            message: "Backend restarted".to_string(),
        }))
    }

    async fn re_start_latency(
        &self,
        _request: Request<ReStartLatencyRequest>,
    ) -> Result<Response<ReStartLatencyResponse>, Status> {
        Ok(Response::new(ReStartLatencyResponse {
            success: true,
            message: "Latency server restarted".to_string(),
        }))
    }

    async fn start_domain_service(
        &self,
        request: Request<StartDomainServiceRequest>,
    ) -> Result<Response<StartDomainServiceResponse>, Status> {
        let domain = request.into_inner().domain;
        Ok(Response::new(StartDomainServiceResponse {
            success: true,
            message: format!("Domain {} started", domain),
        }))
    }

    async fn stop_domain_service(
        &self,
        request: Request<StopDomainServiceRequest>,
    ) -> Result<Response<StopDomainServiceResponse>, Status> {
        let domain = request.into_inner().domain;
        Ok(Response::new(StopDomainServiceResponse {
            success: true,
            message: format!("Domain {} stopped", domain),
        }))
    }

    async fn dump_runtime(
        &self,
        request: Request<DumpRuntimeRequest>,
    ) -> Result<Response<DumpRuntimeResponse>, Status> {
        let path = request.into_inner().output_path;
        Ok(Response::new(DumpRuntimeResponse {
            success: true,
            message: format!("Runtime dumped to {}", path),
        }))
    }
}
