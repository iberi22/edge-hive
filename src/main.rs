//! Edge Hive - Decentralized edge computing platform
//!
//! CLI for managing Edge Hive nodes.

use anyhow::Result;
use clap::{Parser, Subcommand};
use edge_hive_identity::NodeIdentity;
use std::path::PathBuf;
use tracing::{info, error};
use tracing_subscriber;
use edge_hive_core::server;
use edge_hive_discovery::{DiscoveryService, PeerInfo};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use reqwest;
use std::fs;
use libp2p::{PeerId, identity, Multiaddr};
use libp2p::futures::StreamExt;
use libp2p::kad::{Config, store::MemoryStore};
use libp2p::multiaddr::Protocol;
use std::convert::TryFrom;


#[derive(Parser)]
#[command(name = "edge-hive")]
#[command(about = "Decentralized edge computing platform", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Configuration directory
    #[arg(long, global = true, default_value = "~/.config/edge-hive")]
    config_dir: PathBuf,

    /// Verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Identity management
    Identity {
        #[command(subcommand)]
        action: IdentityCommands,
    },

    /// Start Edge Hive node
    Start {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,

        /// Enable HTTPS/TLS (uses self-signed certificates for testing)
        #[arg(long)]
        https: bool,

        /// Hostname for TLS certificate SANs
        #[arg(long, default_value = "localhost")]
        hostname: String,

        /// Enable Tor onion service
        #[arg(long)]
        tor: bool,

        /// Enable discovery service
        #[arg(long)]
        discovery: bool,

        /// Bootstrap peer address
        #[arg(long)]
        bootstrap_peer: Option<String>,
    },

    /// Show node status
    Status,

    /// List discovered peers
    Peers {
        /// API server address
        #[arg(short, long, default_value = "http://127.0.0.1:8080")]
        api_server: String,
    },

    /// Run as MCP Server (Model Context Protocol) over stdio
    Mcp(edge_hive_core::commands::mcp::McpArgs),

    /// Manage OAuth2 authentication (client credentials)
    Auth(edge_hive_core::commands::auth::AuthArgs),
}

#[derive(Subcommand)]
enum IdentityCommands {
    /// Generate new identity
    New {
        /// Custom name (optional)
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Show current identity
    Show,

    /// List all identities
    List,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging (skip for MCP to avoid polluting stdout)
    if !matches!(cli.command, Commands::Mcp(_)) {
        let level = if cli.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        };

        tracing_subscriber::fmt()
            .with_max_level(level)
            .init();
    }

    match cli.command {
        Commands::Identity { action } => handle_identity(action, &cli.config_dir)?,
        Commands::Start { port, https, hostname, tor, discovery, bootstrap_peer } => {
            handle_start(port, https, hostname, tor, discovery, bootstrap_peer, &cli.config_dir).await?
        },
        Commands::Status => handle_status(&cli.config_dir)?,
        Commands::Peers { api_server } => handle_peers(api_server).await?,
        Commands::Mcp(args) => {
            edge_hive_core::commands::mcp::run(args).await?;
        }
        Commands::Auth(args) => {
            let data_dir = expand_path(&cli.config_dir);
            edge_hive_core::commands::auth::run(args, &data_dir).await?;
        }
    }

    Ok(())
}

fn handle_identity(action: IdentityCommands, config_dir: &PathBuf) -> Result<()> {
    let identity_path = expand_path(config_dir).join("identity.key");

    match action {
        IdentityCommands::New { name } => {
            info!("Generating new identity...");
            let identity = NodeIdentity::generate()?;

            if let Some(custom_name) = name {
                info!("Using custom name: {}", custom_name);
                // Note: We'd need to add set_name() method to NodeIdentity
            }

            std::fs::create_dir_all(&identity_path.parent().unwrap())?;
            identity.save(&identity_path, None)?;

            let public = identity.public_identity();
            println!("✅ Identity created successfully!");
            println!();
            println!("  Name:     {}", public.name);
            println!("  Peer ID:  {}", public.peer_id);
            println!("  Created:  {}", public.created_at);
            println!();
            println!("  Saved to: {}", identity_path.display());
        }

        IdentityCommands::Show => {
            if !identity_path.exists() {
                error!("No identity found. Create one with: edge-hive identity new");
                std::process::exit(1);
            }

            let identity = NodeIdentity::load(&identity_path, None)?;
            let public = identity.public_identity();

            println!("Current Identity:");
            println!();
            println!("  Name:       {}", public.name);
            println!("  Peer ID:    {}", public.peer_id);
            println!("  Public Key: {}", public.public_key);
            println!("  Created:    {}", public.created_at);
        }

        IdentityCommands::List => {
            println!("🔑 Identities:");
            if identity_path.exists() {
                let identity = NodeIdentity::load(&identity_path, None)?;
                println!("  • {} (current)", identity.name());
            } else {
                println!("  (none found)");
            }
        }
    }

    Ok(())
}

async fn handle_start(
    port: u16,
    enable_https: bool,
    hostname: String,
    enable_tor: bool,
    discovery: bool,
    bootstrap_peer: Option<String>,
    config_dir: &PathBuf,
) -> Result<()> {
    let identity_path = expand_path(config_dir).join("identity.key");

    if !identity_path.exists() {
        error!("No identity found. Run: edge-hive identity new");
        std::process::exit(1);
    }

    let identity = NodeIdentity::load(&identity_path, None)?;
    info!("Starting Edge Hive node: {}", identity.name());
    info!("Identity loaded: {}", identity.peer_id());

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    info!("Local peer id: {:?}", local_peer_id);


    let discovery_svc = if discovery {
        let ds = DiscoveryService::new().unwrap();
        let mut kad_config = Config::default();
        kad_config.set_query_timeout(std::time::Duration::from_secs(5 * 60));
        let store = MemoryStore::new(local_peer_id);
        let mut kad_behaviour = libp2p::kad::Behaviour::with_config(local_peer_id, store, kad_config);

        if let Some(bootstrap_peer) = bootstrap_peer {
            let addr: Multiaddr = bootstrap_peer.parse()?;
            if let Some(Protocol::P2p(hash)) = addr.iter().last() {
                let peer_id = PeerId::try_from(hash).unwrap();
                kad_behaviour.add_address(&peer_id, addr);
            }
        }

        let mut swarm = libp2p::SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                Default::default(),
                libp2p::noise::Config::new,
                libp2p::yamux::Config::default,
            )?
            .with_behaviour(|_| kad_behaviour)?
            .with_swarm_config(|c| c.with_idle_connection_timeout(std::time::Duration::from_secs(30)))
            .build();
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        tokio::spawn(async move {
            loop {
                let _ = swarm.select_next_some().await;
            }
        });
        Arc::new(RwLock::new(ds))
    } else {
        Arc::new(RwLock::new(DiscoveryService::default()))
    };

    let data_dir = expand_path(config_dir);
    let messages_path = data_dir.join("messages.json");
    let messages = if messages_path.exists() {
        let messages_json = fs::read_to_string(messages_path)?;
        serde_json::from_str(&messages_json)?
    } else {
        HashMap::new()
    };
    let message_store = Arc::new(RwLock::new(messages));

    // Load or generate JWT secret
    let jwt_secret_path = data_dir.join("jwt_secret.key");
    let jwt_secret = if jwt_secret_path.exists() {
        fs::read(&jwt_secret_path)?
    } else {
        use edge_hive_auth::jwt::JwtKeys;
        let secret = JwtKeys::generate_secret();
        fs::write(&jwt_secret_path, &secret)?;
        secret
    };

    // Start Tor service if enabled
    if enable_tor {
        use edge_hive_tunnel::{TorConfig, TorService};
        
        info!("🧅 Tor onion service enabled");
        
        let tor_config = TorConfig::default()
            .map(|cfg| cfg.with_local_port(port).with_enabled(true))
            .unwrap_or_else(|_| {
                TorConfig {
                    data_dir: data_dir.join("tor"),
                    local_port: port,
                    nickname: Some(identity.name().to_string()),
                    enabled: true,
                }
            });
        
        let mut tor_service = TorService::new(tor_config);
        
        // Start Tor service in background
        tokio::spawn(async move {
            match tor_service.start().await {
                Ok(onion_address) => {
                    info!("🧅 Onion service available at: http://{}.onion", onion_address);
                    info!("🧅 Share this address to allow anonymous access to your node");
                }
                Err(e) => {
                    error!("Failed to start Tor service: {}", e);
                }
            }
        });
    }

    server::run(port, discovery_svc, message_store, data_dir, Some(jwt_secret), enable_https, hostname).await?;

    Ok(())
}

fn handle_status(config_dir: &PathBuf) -> Result<()> {
    let identity_path = expand_path(config_dir).join("identity.key");

    if !identity_path.exists() {
        error!("No identity found. Create one with: edge-hive identity new");
        std::process::exit(1);
    }

    let identity = NodeIdentity::load(&identity_path, None)?;
    let public = identity.public_identity();

    println!("📊 Edge Hive Status");
    println!();
    println!("  Peer ID:    {}", public.peer_id);
    println!("  Version:    0.1.0");
    println!("  Rust:       {}", env!("CARGO_PKG_RUST_VERSION"));

    Ok(())
}

async fn handle_peers(api_server: String) -> Result<()> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/v1/peers", api_server);

    println!("🔍 Querying peers from {}...", url);

    match client.get(&url).send().await {
        Ok(res) => {
            if res.status().is_success() {
                let peers: Vec<PeerInfo> = res.json().await?;
                if peers.is_empty() {
                    println!("\nNo peers discovered yet.");
                } else {
                    println!("\nDiscovered Peers ({}):", peers.len());
                    println!("--------------------------------------------------");
                    for peer in peers {
                        println!("Peer ID: {}", peer.peer_id);
                        println!("  Source: {:?}", peer.source);
                        println!("  Last Seen: {}", peer.last_seen);
                        println!("  Addresses:");
                        for addr in peer.addresses {
                            println!("    - {}", addr);
                        }
                        println!("--------------------------------------------------");
                    }
                }
            } else {
                println!("\nError: Received status code {}", res.status());
            }
        }
        Err(e) => {
            println!("\nError: Failed to connect to the server: {}", e);
        }
    }

    Ok(())
}


/// Expand ~ in path
fn expand_path(path: &PathBuf) -> PathBuf {
    let path_str = path.to_str().unwrap();
    if path_str.starts_with("~") {
        let home = directories::BaseDirs::new()
            .map(|b| b.home_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));

        home.join(&path_str[2..])
    } else {
        path.clone()
    }
}
