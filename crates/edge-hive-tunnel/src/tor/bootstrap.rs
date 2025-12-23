//! Tor network bootstrap and connection management

use anyhow::{Context, Result};
use arti_client::{TorClient, TorClientConfig};
use tor_config::CfgPath;
use tor_rtcompat::PreferredRuntime;
use tracing::{info, debug};
use super::TorConfig;

/// Tor bootstrap manager
pub struct TorBootstrap {
    config: TorConfig,
}

impl TorBootstrap {
    /// Create new bootstrap manager
    pub fn new(config: TorConfig) -> Self {
        Self { config }
    }
    
    /// Connect to Tor network and bootstrap consensus
    pub async fn connect(&self) -> Result<TorClient<PreferredRuntime>> {
        info!("Bootstrapping Tor connection...");
        debug!("Using data directory: {}", self.config.data_dir.display());
        
        // Ensure Tor state directory exists
        let state_dir = self.config.data_dir.join("state");
        std::fs::create_dir_all(&state_dir)
            .context("Failed to create Tor state directory")?;

        let mut config_builder = TorClientConfig::builder();
        config_builder.storage().state_dir(CfgPath::new(self.config.data_dir.to_str().unwrap().to_string()));
        let config = config_builder.build()?;

        let tor_client = TorClient::create_bootstrapped(config).await?;
        
        info!("✓ Tor bootstrap complete");
        
        Ok(tor_client)
    }
}
