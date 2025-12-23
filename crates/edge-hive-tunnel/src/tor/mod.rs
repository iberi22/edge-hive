//! Tor Onion Service implementation using tor-hsservice directly
//!
//! This bypasses arti-client (which lacks hsservice in 0.20) and uses
//! the tor-hsservice crate directly for maximum control.

use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::path::{PathBuf};
use tracing::{info};

mod bootstrap;
mod onion;

pub use bootstrap::TorBootstrap;
pub use onion::OnionService;
use tor_rtcompat::PreferredRuntime;

/// Configuration for Tor integration
#[derive(Debug, Clone)]
pub struct TorConfig {
    /// Directory for Tor data (state, keys, cache)
    pub data_dir: PathBuf,

    /// Local port to forward onion traffic to
    pub local_port: u16,

    /// Custom nickname for the onion service
    pub nickname: Option<String>,
}

impl TorConfig {
    /// Create default Tor configuration
    pub fn default() -> Result<Self> {
        let project_dirs = ProjectDirs::from("io", "edge-hive", "Edge Hive")
            .context("Failed to determine project directories")?;

        let data_dir = project_dirs.data_dir().join("tor");

        Ok(Self {
            data_dir,
            local_port: 8080,
            nickname: None,
        })
    }

    /// Set custom data directory
    pub fn with_data_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.data_dir = path.into();
        self
    }

    /// Set local port
    pub fn with_local_port(mut self, port: u16) -> Self {
        self.local_port = port;
        self
    }

    /// Set onion service nickname
    pub fn with_nickname<S: Into<String>>(mut self, name: S) -> Self {
        self.nickname = Some(name.into());
        self
    }
}

/// Main Tor service manager
pub struct TorService {
    config: TorConfig,
    onion_address: Option<String>,
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl TorService {
    /// Create a new Tor service with the given configuration
    pub fn new(config: TorConfig) -> Self {
        Self {
            config,
            onion_address: None,
            shutdown_tx: None,
        }
    }

    /// Bootstrap Tor and launch onion service
    pub async fn start(&mut self) -> Result<String> {
        info!("Starting Tor service...");

        // Ensure data directory exists
        std::fs::create_dir_all(&self.config.data_dir)
            .context("Failed to create Tor data directory")?;

        // Bootstrap Tor connection
        let bootstrap = TorBootstrap::new(self.config.clone());
        let tor_client = bootstrap.connect().await?;

        // Launch onion service
        let onion = OnionService::new(self.config.clone(), tor_client);
        let onion_address = onion.launch().await?;

        info!("🧅 Onion service running: http://{}.onion", onion_address);
        self.onion_address = Some(onion_address.clone());

        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
        self.shutdown_tx = Some(shutdown_tx);

        tokio::spawn(async move {
            shutdown_rx.await.ok();
            info!("Tor service shutting down...");
        });

        Ok(onion_address)
    }

    /// Get the onion address (if started)
    pub fn onion_address(&self) -> Option<&str> {
        self.onion_address.as_deref()
    }

    /// Stop the Tor service
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            shutdown_tx.send(()).ok();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_builder() {
        let config = TorConfig::default()
            .unwrap()
            .with_local_port(3000)
            .with_nickname("test-node");

        assert_eq!(config.local_port, 3000);
        assert_eq!(config.nickname, Some("test-node".to_string()));
    }

    #[tokio::test]
    #[ignore] // This test requires a network connection and can be slow
    async fn test_tor_service_start_stop() {
        let temp_dir = tempdir().unwrap();
        let config = TorConfig::default()
            .unwrap()
            .with_data_dir(temp_dir.path())
            .with_local_port(8080);

        let mut service = TorService::new(config);
        let onion_address = service.start().await.unwrap();

        assert!(onion_address.ends_with(".onion"));
        assert!(service.onion_address().is_some());

        service.stop().await.unwrap();
        assert!(service.shutdown_tx.is_none());
    }
}
