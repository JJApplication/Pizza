use tokio::signal;
use crate::error::Result;
use crate::constants::SHUTDOWN_TIMEOUT_SECS;
use tracing;

pub struct LifecycleManager {
    shutdown_timeout_secs: u64,
}

impl LifecycleManager {
    pub fn new() -> Self {
        Self {
            shutdown_timeout_secs: SHUTDOWN_TIMEOUT_SECS,
        }
    }

    pub async fn wait_for_shutdown(&self) -> Result<()> {
        tracing::info!("Waiting for shutdown signal...");

        #[cfg(unix)]
        {
            signal::unix::SignalKind::terminate();
            signal::unix::SignalKind::interrupt();
        }

        tokio::select! {
            _ = signal::ctrl_c() => {
                tracing::info!("Received SIGINT, shutting down gracefully");
            }
            result = self.wait_terminate() => {
                result?;
            }
        }

        tracing::info!("Shutdown complete");
        Ok(())
    }

    #[cfg(unix)]
    async fn wait_terminate(&self) -> Result<()> {
        let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
            .map_err(|e| crate::error::PizzaError::Io(e))?;
        sigterm.recv().await;
        tracing::info!("Received SIGTERM, shutting down gracefully");
        Ok(())
    }

    #[cfg(not(unix))]
    async fn wait_terminate(&self) -> Result<()> {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}

impl Default for LifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}
