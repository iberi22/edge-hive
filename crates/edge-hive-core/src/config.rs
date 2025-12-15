//! Configuration module for Edge Hive

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Node identity configuration
    pub identity: IdentityConfig,
    /// Server configuration
    pub server: ServerConfig,
    /// Discovery configuration
    pub discovery: DiscoveryConfig,
    /// Tunnel configuration
    pub tunnel: TunnelConfig,
    /// Database configuration
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityConfig {
    /// Path to identity key file
    pub key_path: PathBuf,
    /// Human-readable node name
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// HTTP port
    pub port: u16,
    /// Bind address
    pub bind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Enable mDNS local discovery
    pub mdns_enabled: bool,
    /// Enable Kademlia DHT
    pub dht_enabled: bool,
    /// Bootstrap nodes for DHT
    pub bootstrap_nodes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelConfig {
    /// Enable tunneling
    pub enabled: bool,
    /// Tunnel backend: "libcfd", "cloudflared", "tor"
    pub backend: String,
    /// Cloudflare tunnel token (for named tunnels)
    pub cf_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database path
    pub path: PathBuf,
    /// Database namespace
    pub namespace: String,
    /// Database name
    pub database: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            identity: IdentityConfig {
                key_path: PathBuf::from("~/.edge-hive/identity.key"),
                name: None,
            },
            server: ServerConfig {
                port: 8080,
                bind: "0.0.0.0".into(),
            },
            discovery: DiscoveryConfig {
                mdns_enabled: true,
                dht_enabled: true,
                bootstrap_nodes: vec![],
            },
            tunnel: TunnelConfig {
                enabled: false,
                backend: "libcfd".into(),
                cf_token: None,
            },
            database: DatabaseConfig {
                path: PathBuf::from("~/.edge-hive/data"),
                namespace: "edge_hive".into(),
                database: "main".into(),
            },
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(path).required(false))
            .add_source(config::Environment::with_prefix("EDGE_HIVE"))
            .build()?;

        let config: Config = settings.try_deserialize().unwrap_or_default();
        Ok(config)
    }
}
