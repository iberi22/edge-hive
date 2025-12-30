//! Edge Hive Tunnel - Cloudflare and Tor tunneling support
//!
//! Exposes local services to the internet via Cloudflare Tunnel or provides a client to connect to the Tor network.

pub mod tor;

// Re-export main Tor types for convenience
pub use tor::{TorConfig, TorService, TorStatus};

use serde::{Deserialize, Serialize};
use std::process::Stdio;
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::process::{Child, Command};
use tracing::{info, warn};
use directories::ProjectDirs;

/// Errors that can occur during tunnel operations
#[derive(Debug, Error)]
pub enum TunnelError {
    #[error("Tunnel not available: {0}")]
    NotAvailable(String),

    #[error("Failed to start tunnel: {0}")]
    Start(String),

    #[error("Tunnel process error: {0}")]
    Process(#[from] std::io::Error),

    #[error("Tor client error: {0}")]
    Tor(#[from] anyhow::Error),

    #[error("Tunnel is not the correct type for this operation")]
    IncorrectBackend,
}

/// Tunnel backend type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TunnelBackend {
    /// LibCFD native Rust implementation (preferred)
    LibCfd,
    /// Cloudflared binary subprocess
    Cloudflared,
    /// Tor client via Arti
    Tor,
}

/// Represents the current state of the active tunnel.
pub enum TunnelState {
    /// A cloudflared subprocess is running.
    Cloudflared(Child),
    /// A Tor client service is running.
    Tor(TorService),
    /// The tunnel is inactive.
    Inactive,
}

/// Tunnel service for exposing local ports or connecting to Tor.
pub struct TunnelService {
    backend: TunnelBackend,
    state: TunnelState,
    public_url: Option<String>,
}

impl TunnelService {
    /// Create a new tunnel service with the specified backend.
    pub fn new(backend: TunnelBackend) -> Self {
        Self {
            backend,
            state: TunnelState::Inactive,
            public_url: None,
        }
    }

    /// Check if cloudflared binary is available.
    pub fn cloudflared_available() -> bool {
        which::which("cloudflared").is_ok()
    }

    /// Start a tunnel. The behavior depends on the selected backend.
    /// For Cloudflared, it returns a public URL.
    /// For Tor, it starts the client and returns a confirmation message.
    pub async fn start(&mut self, local_port: u16) -> Result<String, TunnelError> {
        match self.backend {
            TunnelBackend::Cloudflared => self.start_cloudflared_quick(local_port).await,
            TunnelBackend::LibCfd => {
                warn!("LibCFD not yet available, falling back to cloudflared");
                self.backend = TunnelBackend::Cloudflared;
                self.start_cloudflared_quick(local_port).await
            }
            TunnelBackend::Tor => self.start_tor_client().await,
        }
    }

    async fn start_cloudflared_quick(&mut self, local_port: u16) -> Result<String, TunnelError> {
        if !Self::cloudflared_available() {
            return Err(TunnelError::NotAvailable(
                "cloudflared binary not found".into(),
            ));
        }

        info!("🚇 Starting quick tunnel to port {}", local_port);
        let child = Command::new("cloudflared")
            .args([
                "tunnel",
                "--no-autoupdate",
                "--url",
                &format!("http://localhost:{}", local_port),
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // In a real implementation, we'd parse the output to get the URL.
        // For now, we'll wait a bit and set a placeholder.
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        self.state = TunnelState::Cloudflared(child);
        let url = "https://<random>.trycloudflare.com".to_string();
        self.public_url = Some(url.clone());

        info!("✅ Cloudflared tunnel established (check logs for actual URL)");
        Ok(url)
    }

    async fn start_tor_client(&mut self) -> Result<String, TunnelError> {
        info!("🧅 Initializing Tor client...");

        let proj_dirs = ProjectDirs::from("com", "EdgeHive", "EdgeHive")
            .ok_or_else(|| TunnelError::Start("Could not determine project directories".into()))?;
        let data_dir = proj_dirs.data_dir().to_path_buf();

        let config = TorConfig::new(data_dir, true);
        let mut tor_service = TorService::new(config);

        let onion_address = tor_service.start().await?;

        self.state = TunnelState::Tor(tor_service);
        self.public_url = Some(format!("{}.onion", onion_address));

        info!("✅ {}", self.public_url.as_ref().unwrap());
        Ok(self.public_url.clone().unwrap())
    }

    /// Connect to a Tor onion service. This requires the backend to be `TunnelBackend::Tor`.
    pub async fn connect_onion(
        &self,
        address: &str,
        port: u16,
    ) -> Result<impl AsyncRead + AsyncWrite + Send + Unpin, TunnelError> {
        if let TunnelState::Tor(tor_service) = &self.state {
            let stream = tor_service.connect_onion(address, port).await?;
            Ok(stream)
        } else {
            Err(TunnelError::IncorrectBackend)
        }
    }

    /// Get the current status of the Tor client.
    /// Returns `None` if the backend is not `TunnelBackend::Tor`.
    pub fn tor_status(&self) -> Option<TorStatus> {
        if let TunnelState::Tor(tor_service) = &self.state {
            Some(tor_service.status())
        } else {
            None
        }
    }

    /// Get the public URL if a cloudflared tunnel is running.
    pub fn public_url(&self) -> Option<&str> {
        self.public_url.as_deref()
    }

    /// Check if the tunnel service is active.
    pub fn is_running(&self) -> bool {
        !matches!(self.state, TunnelState::Inactive)
    }

    /// Stop the tunnel.
    pub async fn stop(&mut self) -> Result<(), TunnelError> {
        match &mut self.state {
            TunnelState::Cloudflared(process) => {
                info!("🛑 Stopping cloudflared tunnel");
                process.kill().await?;
                self.public_url = None;
            }
            TunnelState::Tor(tor_service) => {
                tor_service.stop().await?;
            }
            TunnelState::Inactive => {}
        }
        self.state = TunnelState::Inactive;
        Ok(())
    }
}

impl Drop for TunnelService {
    fn drop(&mut self) {
        if let TunnelState::Cloudflared(ref mut process) = self.state {
            // Best effort to kill the process
            let _ = process.start_kill();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tunnel_service_creation() {
        let service = TunnelService::new(TunnelBackend::Cloudflared);
        assert!(!service.is_running());
        assert!(service.public_url().is_none());
    }
}
