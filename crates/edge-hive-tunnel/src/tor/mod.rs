//! Tor Onion Service implementation using tor-hsservice directly
//!
//! This bypasses arti-client (which lacks hsservice in 0.20) and uses
//! the tor-hsservice crate directly for maximum control.

use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::path::{PathBuf};
use std::net::SocketAddr;
use tracing::{info, warn};

mod bootstrap;
mod onion;

pub use bootstrap::TorBootstrap;
pub use onion::OnionService;

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

/// Main Tor node manager
pub struct TorNode {
    config: TorConfig,
    onion_address: Option<String>,
}

impl TorNode {
    /// Create a new Tor node with the given configuration
    pub fn new(config: TorConfig) -> Self {
        Self {
            config,
            onion_address: None,
        }
    }

    /// Bootstrap Tor and launch onion service
    pub async fn start(&mut self) -> Result<String> {
        info!("Starting Tor node...");

        // Ensure data directory exists
        std::fs::create_dir_all(&self.config.data_dir)
            .context("Failed to create Tor data directory")?;

        // Bootstrap Tor connection
        let bootstrap = TorBootstrap::new(self.config.clone());
        bootstrap.connect().await?;

        // Launch onion service
        let onion = OnionService::new(self.config.clone());
        let onion_address = onion.launch().await?;

        info!("🧅 Onion service running: http://{}.onion", onion_address);
        self.onion_address = Some(onion_address.clone());

        Ok(onion_address)
    }

    /// Get the onion address (if started)
    pub fn onion_address(&self) -> Option<&str> {
        self.onion_address.as_deref()
    }

    /// Forward traffic from onion service to local server
    pub async fn forward_traffic(&self) -> Result<()> {
        let local_addr = SocketAddr::from(([127, 0, 0, 1], self.config.local_port));

        info!("Forwarding onion traffic to {}", local_addr);

        // NOTE: Actual traffic forwarding will be implemented when
        // tor-hsservice provides better stream handling APIs

        warn!("Traffic forwarding not yet implemented - waiting for tor-hsservice improvements");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = TorConfig::default()
            .unwrap()
            .with_local_port(3000)
            .with_nickname("test-node");

        assert_eq!(config.local_port, 3000);
        assert_eq!(config.nickname, Some("test-node".to_string()));
    }
}
