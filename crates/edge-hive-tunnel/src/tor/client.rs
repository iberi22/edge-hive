//! Tor client implementation using arti-client.
use anyhow::{anyhow, Result};
use arti_client::{
    config::{CfgPath, TorClientConfigBuilder},
    TorClient,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tracing::{info, instrument};

/// Represents the bootstrap status of the Tor client.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TorStatus {
    /// Not yet started.
    Idle,
    /// Bootstrapping the connection.
    Bootstrapping(u8),
    /// Ready to be used.
    Ready,
}

/// Configuration for the Tor client service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorConfig {
    /// Directory to store Tor state, caches, and keys.
    pub data_dir: PathBuf,
    /// Whether the Tor client should be enabled.
    pub enabled: bool,
    /// Optional nickname for the hidden service.
    pub nickname: Option<String>,
    /// Local port to forward onion traffic to.
    pub local_port: u16,
}

impl TorConfig {
    /// Create a new TorConfig, resolving the data directory.
    pub fn new(data_dir: PathBuf, enabled: bool) -> Self {
        Self {
            data_dir,
            enabled,
            nickname: None,
            local_port: 8080,
        }
    }

    /// Set the nickname for the hidden service.
    pub fn with_nickname(mut self, nickname: String) -> Self {
        self.nickname = Some(nickname);
        self
    }

    /// Set the local port for the hidden service.
    pub fn with_local_port(mut self, port: u16) -> Self {
        self.local_port = port;
        self
    }
}

/// Main Tor service manager.
pub struct TorService {
    config: TorConfig,
    client: Option<TorClient<tor_rtcompat::PreferredRuntime>>,
    running: bool,
    onion_address: Option<String>,
}

impl TorService {
    /// Create a new Tor service with the given configuration.
    pub fn new(config: TorConfig) -> Self {
        Self {
            config,
            client: None,
            running: false,
            onion_address: None,
        }
    }

    /// Start the Tor client and bootstrap a connection to the Tor network.
    pub async fn start(&mut self) -> Result<String> {
        if !self.config.enabled || self.running {
            return self.onion_address.clone().ok_or_else(|| anyhow!("Tor client already running without onion service"));
        }

        info!("🧅 Starting Tor client...");

        // Ensure data directory exists
        tokio::fs::create_dir_all(&self.config.data_dir).await?;

        let mut config_builder = TorClientConfigBuilder::default();

        let cache_dir = self.config.data_dir.join("cache");
        let state_dir = self.config.data_dir.join("state");

        config_builder
            .storage()
            .cache_dir(CfgPath::new_literal(cache_dir))
            .state_dir(CfgPath::new_literal(state_dir));

        let config = config_builder.build()?;

        let runtime = tor_rtcompat::PreferredRuntime::current()?;
        let client = TorClient::with_runtime(runtime)
            .config(config)
            .create_unbootstrapped()?;

        client.bootstrap().await?;

        self.client = Some(client.clone());
        self.running = true;
        info!("✅ Tor client bootstrapped.");

        // Launch Onion Service
        let onion_manager = super::OnionService::new(self.config.clone(), client);
        let onion_address = onion_manager.launch().await?;
        self.onion_address = Some(onion_address.clone());

        info!("✅ Tor Onion Service active at: {}.onion", onion_address);
        Ok(onion_address)
    }

    /// Stop the Tor service.
    pub async fn stop(&mut self) -> Result<()> {
        if self.running {
            info!("🛑 Stopping Tor client");
            self.client = None;
            self.running = false;
        }
        Ok(())
    }

    /// Check if the Tor client is running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get the current bootstrap status of the Tor client.
    pub fn status(&self) -> TorStatus {
        match &self.client {
            Some(client) => {
                let status = client.bootstrap_status();
                let progress = (status.as_frac() * 100.0) as u8;
                if progress >= 100 {
                    TorStatus::Ready
                } else {
                    TorStatus::Bootstrapping(progress)
                }
            }
            None => TorStatus::Idle,
        }
    }

    /// Get the onion address if the service is running.
    pub fn onion_address(&self) -> Option<&str> {
        self.onion_address.as_deref()
    }

    /// Connect to a given onion service address and port.
    /// Returns an anonymous stream that can be used for communication.
    #[instrument(skip(self))]
    pub async fn connect_onion(
        &self,
        address: &str,
        port: u16,
    ) -> Result<impl AsyncRead + AsyncWrite + Send + Unpin> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("Tor client is not running"))?;

        info!("🧅 Connecting to onion service: {}:{}", address, port);
        let stream = client.connect((address, port)).await?;
        info!("✅ Connected to onion service.");

        Ok(stream.compat())
    }
}
