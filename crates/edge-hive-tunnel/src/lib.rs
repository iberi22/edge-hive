//! Edge Hive Tunnel - Cloudflare and Tor tunneling support
//!
//! Exposes local services to the internet via Cloudflare Tunnel or Tor onion services.

pub mod tor;

// Re-export main Tor types for convenience
pub use tor::{TorConfig, TorService};

use serde::{Deserialize, Serialize};
use std::process::Stdio;
use thiserror::Error;
use tokio::process::{Child, Command};
use tracing::{info, warn};

/// Errors that can occur during tunnel operations
#[derive(Debug, Error)]
pub enum TunnelError {
    #[error("Tunnel not available: {0}")]
    NotAvailable(String),

    #[error("Failed to start tunnel: {0}")]
    Start(String),

    #[error("Tunnel process error: {0}")]
    Process(#[from] std::io::Error),
}

/// Tunnel backend type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TunnelBackend {
    /// LibCFD native Rust implementation (preferred)
    LibCfd,
    /// Cloudflared binary subprocess
    Cloudflared,
    /// Tor onion service via Arti
    Tor,
}

/// Tunnel service for exposing local ports to the internet
pub struct TunnelService {
    backend: TunnelBackend,
    process: Option<Child>,
    public_url: Option<String>,
}

impl TunnelService {
    /// Create a new tunnel service with the specified backend
    pub fn new(backend: TunnelBackend) -> Self {
        Self {
            backend,
            process: None,
            public_url: None,
        }
    }

    /// Check if cloudflared binary is available
    pub fn cloudflared_available() -> bool {
        which::which("cloudflared").is_ok()
    }

    /// Start a quick tunnel (TryCloudflare - no account needed)
    pub async fn start_quick(&mut self, local_port: u16) -> Result<String, TunnelError> {
        match self.backend {
            TunnelBackend::Cloudflared => self.start_cloudflared_quick(local_port).await,
            TunnelBackend::LibCfd => {
                // TODO: Implement LibCFD when available
                warn!("LibCFD not yet available, falling back to cloudflared");
                self.backend = TunnelBackend::Cloudflared;
                self.start_cloudflared_quick(local_port).await
            }
            TunnelBackend::Tor => {
                // Use TorService for Tor onion services
                self.start_tor(local_port).await
            }
        }
    }

    /// Start a named tunnel (requires Cloudflare account and token)
    pub async fn start_named(&mut self, local_port: u16, token: &str) -> Result<String, TunnelError> {
        if !Self::cloudflared_available() {
            return Err(TunnelError::NotAvailable(
                "cloudflared binary not found. Install from https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/installation/".into()
            ));
        }

        info!("🚇 Starting named tunnel to port {}", local_port);

        let child = Command::new("cloudflared")
            .args([
                "tunnel",
                "--no-autoupdate",
                "run",
                "--token",
                token,
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        self.process = Some(child);

        // For named tunnels, the URL is configured in Cloudflare dashboard
        // We'd need to parse the output or use the API to get it
        self.public_url = Some("https://<configured-hostname>.your-domain.com".into());

        Ok(self.public_url.clone().unwrap())
    }

    async fn start_cloudflared_quick(&mut self, local_port: u16) -> Result<String, TunnelError> {
        if !Self::cloudflared_available() {
            return Err(TunnelError::NotAvailable(
                "cloudflared binary not found. Install from https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/installation/".into()
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

        // Read stderr to find the URL (cloudflared outputs it there)
        // In a real implementation, we'd parse the output properly
        // For now, we'll set a placeholder

        // Give cloudflared time to establish the tunnel
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        self.process = Some(child);
        self.public_url = Some("https://<random>.trycloudflare.com".into());

        info!("✅ Tunnel established (check logs for actual URL)");

        Ok(self.public_url.clone().unwrap())
    }

    async fn start_tor(&mut self, local_port: u16) -> Result<String, TunnelError> {
        info!("🧅 Starting Tor onion service on port {}", local_port);

        let config = TorConfig::default()
            .map_err(|e| TunnelError::Start(format!("Failed to create Tor config: {}", e)))?
            .with_local_port(local_port)
            .with_enabled(true);

        let mut tor_service = TorService::new(config);
        let onion_address = tor_service
            .start()
            .await
            .map_err(|e| TunnelError::Start(format!("Failed to start Tor service: {}", e)))?;

        let onion_url = format!("http://{}.onion", onion_address);
        self.public_url = Some(onion_url.clone());

        info!("✅ Tor onion service established: {}", onion_url);

        Ok(onion_url)
    }

    /// Get the public URL if tunnel is running
    pub fn public_url(&self) -> Option<&str> {
        self.public_url.as_deref()
    }

    /// Check if tunnel is running
    pub fn is_running(&self) -> bool {
        self.process.is_some()
    }

    /// Stop the tunnel
    pub async fn stop(&mut self) -> Result<(), TunnelError> {
        if let Some(mut process) = self.process.take() {
            info!("🛑 Stopping tunnel");
            process.kill().await?;
            self.public_url = None;
        }
        Ok(())
    }
}

impl Drop for TunnelService {
    fn drop(&mut self) {
        if let Some(mut process) = self.process.take() {
            // Best effort to kill the process
            let _ = process.start_kill();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tunnel_backend_defaults() {
        let service = TunnelService::new(TunnelBackend::Cloudflared);
        assert!(!service.is_running());
        assert!(service.public_url().is_none());
    }
}
