use tonic::transport::Server;
use crate::grpc::service::PizzaAppService;
use crate::grpc::service::pizza_proto::app_service_server::AppServiceServer;
use crate::error::Result;
use std::net::SocketAddr;

pub struct GrpcServer {
    addr: SocketAddr,
    unix_socket: Option<String>,
}

impl GrpcServer {
    pub fn new(addr: SocketAddr, unix_socket: Option<String>) -> Self {
        Self { addr, unix_socket }
    }

    pub async fn start(&self, service: PizzaAppService) -> Result<()> {
        tracing::info!(addr = %self.addr, "gRPC server starting");

        Server::builder()
            .add_service(AppServiceServer::new(service))
            .serve(self.addr)
            .await
            .map_err(|e| crate::error::GrpcError::ServerError(e.to_string()))?;

        Ok(())
    }

    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }
}
