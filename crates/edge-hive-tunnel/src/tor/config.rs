//! Tor configuration management

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for Tor integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorConfig {
    /// Directory for Tor data (state, keys, cache)
    pub data_dir: PathBuf,

    /// Local port to forward onion traffic to
    pub local_port: u16,

    /// Custom nickname for the onion service
    pub nickname: Option<String>,

    /// Enable Tor service
    pub enabled: bool,
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
            enabled: false,
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

    /// Enable Tor service
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Default for TorConfig {
    fn default() -> Self {
        Self::default().unwrap_or_else(|_| Self {
            data_dir: PathBuf::from("data/tor"),
            local_port: 8080,
            nickname: None,
            enabled: false,
        })
    }
}
