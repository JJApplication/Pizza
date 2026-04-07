use std::net::SocketAddr;
use std::sync::Arc;
use tokio::task::JoinSet;
use crate::error::Result;
use crate::gateway::server::http_server::GatewayServer;

pub struct ServerInfo {
    pub name: String,
    pub addr: SocketAddr,
    pub tls: bool,
    pub started: bool,
}

pub struct GatewayManager {
    servers: Vec<GatewayServer>,
    infos: Vec<ServerInfo>,
    join_set: JoinSet<Result<()>>,
}

impl GatewayManager {
    pub fn new() -> Self {
        Self {
            servers: Vec::new(),
            infos: Vec::new(),
            join_set: JoinSet::new(),
        }
    }

    pub fn add_server(&mut self, server: GatewayServer, info: ServerInfo) {
        self.servers.push(server);
        self.infos.push(info);
    }

    pub async fn start_all(&mut self) -> Result<()> {
        for server in self.servers.drain(..) {
            let mut srv = server;
            self.join_set.spawn(async move {
                srv.start().await
            });
        }

        for info in &mut self.infos {
            info.started = true;
        }

        tracing::info!("All gateway servers started");
        Ok(())
    }

    pub async fn stop_all(&mut self) -> Result<()> {
        for server in &mut self.servers {
            server.shutdown();
        }

        while let Some(result) = self.join_set.join_next().await {
            if let Err(e) = result {
                tracing::error!(error = %e, "Error waiting for server shutdown");
            }
        }

        for info in &mut self.infos {
            info.started = false;
        }

        tracing::info!("All gateway servers stopped");
        Ok(())
    }

    pub async fn restart_all(&mut self, proxy: Arc<crate::gateway::ReverseProxy>) -> Result<()> {
        self.stop_all().await?;

        for info in &self.infos {
            let mut server = GatewayServer::new(info.addr, proxy.clone());
            if info.tls {
                server = server.with_tls("cert.pem".to_string(), "key.pem".to_string());
            }
            self.servers.push(server);
        }

        self.start_all().await
    }

    pub fn server_infos(&self) -> &[ServerInfo] {
        &self.infos
    }

    pub fn is_started(&self) -> bool {
        self.infos.iter().any(|i| i.started)
    }
}

impl Default for GatewayManager {
    fn default() -> Self {
        Self::new()
    }
}
