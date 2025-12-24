//! Tor Onion Service implementation using tor-hsservice directly
//!
//! This bypasses arti-client (which lacks hsservice in 0.20) and uses
//! the tor-hsservice crate directly for maximum control.

use anyhow::{Context, Result};
use std::net::SocketAddr;
use tracing::{info, warn};

mod bootstrap;
mod config;
mod onion;

pub use bootstrap::TorBootstrap;
pub use config::TorConfig;
pub use onion::OnionService;

/// Main Tor service manager
pub struct TorService {
    config: TorConfig,
    onion_address: Option<String>,
    running: bool,
}

impl TorService {
    /// Create a new Tor service with the given configuration
    pub fn new(config: TorConfig) -> Self {
        Self {
            config,
            onion_address: None,
            running: false,
        }
    }

    /// Start Tor service: bootstrap Tor and launch onion service
    pub async fn start(&mut self) -> Result<String> {
        if self.running {
            return Err(anyhow::anyhow!("Tor service is already running"));
        }

        info!("🧅 Starting Tor service...");

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
        self.running = true;

        Ok(onion_address)
    }

    /// Stop the Tor service
    pub async fn stop(&mut self) -> Result<()> {
        if !self.running {
            return Ok(());
        }

        info!("🛑 Stopping Tor service");
        self.running = false;
        self.onion_address = None;

        // NOTE: Actual cleanup will be implemented when
        // tor-hsservice provides better shutdown APIs
        warn!("Tor service cleanup not yet fully implemented");

        Ok(())
    }

    /// Get the onion address (if started)
    pub fn onion_address(&self) -> Option<&str> {
        self.onion_address.as_deref()
    }

    /// Check if Tor service is running
    pub fn is_running(&self) -> bool {
        self.running
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
        let config = TorConfig {
            data_dir: std::path::PathBuf::from("/tmp/tor-test"),
            local_port: 8080,
            nickname: None,
            enabled: false,
        }
        .with_local_port(3000)
        .with_nickname("test-node")
        .with_enabled(true);

        assert_eq!(config.local_port, 3000);
        assert_eq!(config.nickname, Some("test-node".to_string()));
        assert!(config.enabled);
    }

    #[test]
    fn test_tor_service_creation() {
        let config = TorConfig {
            data_dir: std::path::PathBuf::from("/tmp/tor-test"),
            local_port: 8080,
            nickname: None,
            enabled: false,
        };
        let service = TorService::new(config);
        
        assert!(!service.is_running());
        assert!(service.onion_address().is_none());
    }

    #[tokio::test]
    async fn test_tor_service_start_generates_address() {
        let config = TorConfig {
            data_dir: std::path::PathBuf::from("/tmp/tor-test-start"),
            local_port: 8080,
            nickname: Some("test-service".to_string()),
            enabled: true,
        };
        
        let mut service = TorService::new(config);
        
        // Note: This will generate an onion address but won't actually
        // connect to the Tor network in tests
        let result = service.start().await;
        
        // Should succeed in generating address
        assert!(result.is_ok());
        assert!(service.is_running());
        assert!(service.onion_address().is_some());
        
        let address = service.onion_address().unwrap();
        // v3 onion addresses are 56 characters
        assert_eq!(address.len(), 56);
        
        // Stop the service
        let stop_result = service.stop().await;
        assert!(stop_result.is_ok());
        assert!(!service.is_running());
    }

    #[tokio::test]
    async fn test_tor_service_double_start_fails() {
        let config = TorConfig {
            data_dir: std::path::PathBuf::from("/tmp/tor-test-double"),
            local_port: 8080,
            nickname: None,
            enabled: true,
        };
        
        let mut service = TorService::new(config);
        
        // First start should succeed
        let result1 = service.start().await;
        assert!(result1.is_ok());
        
        // Second start should fail
        let result2 = service.start().await;
        assert!(result2.is_err());
    }
}
