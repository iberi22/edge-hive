//! Start the Edge Hive server

use edge_hive_core::server;
use edge_hive_discovery::DiscoveryService;
use edge_hive_identity::NodeIdentity;
use edge_hive_tunnel::{TunnelBackend, TunnelService};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use clap::Args;

#[derive(Args, Debug)]
pub struct ServeArgs {
    /// Port to listen on
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,

    /// Enable Cloudflare tunnel
    #[arg(long)]
    pub tunnel: bool,

    /// Enable discovery service
    #[arg(long)]
    pub discovery: bool,

    /// Enable Tor onion service
    #[arg(long)]
    pub tor: bool,
}

/// Run the serve command
pub async fn run(
    args: ServeArgs,
    data_dir: &Path,
) -> anyhow::Result<()> {
    let identity_path = data_dir.join("identity.key");

    // Load identity
    let identity = if identity_path.exists() {
        NodeIdentity::load(&identity_path, None)?
    } else {
        warn!("No identity found, generating new one...");
        let identity = NodeIdentity::generate()?;
        identity.save(&identity_path, None)?;
        identity
    };

    println!("🐝 Edge Hive Node");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   Name:    {}", identity.name());
    println!("   Peer ID: {}...", &identity.peer_id()[..16]);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    // Initialize discovery service
    let _discovery = if args.discovery {
        info!("🔍 Starting discovery service...");
        match DiscoveryService::new() {
            Ok(svc) => {
                println!("✅ Discovery: Enabled (mDNS + DHT)");
                Some(Arc::new(RwLock::new(svc)))
            }
            Err(e) => {
                warn!("Failed to start discovery: {}", e);
                println!("⚠️  Discovery: Failed to start");
                None
            }
        }
    } else {
        println!("⏸️  Discovery: Disabled");
        None
    };

    // Initialize tunnel service
    let tunnel = if args.tunnel {
        info!("🚇 Starting tunnel service...");
        let mut tunnel = TunnelService::new(TunnelBackend::Cloudflared);

        match tunnel.start_quick(args.port).await {
            Ok(url) => {
                println!("✅ Tunnel: {}", url);
                Some(tunnel)
            }
            Err(e) => {
                warn!("Failed to start tunnel: {}", e);
                println!("⚠️  Tunnel: Failed ({})", e);
                None
            }
        }
    } else {
        println!("⏸️  Tunnel: Disabled (use --tunnel to enable)");
        None
    };

    // Initialize Tor onion service
    let tor_service = if args.tor {
        info!("🧅 Starting Tor onion service...");
        
        // Import Tor module
        use edge_hive_tunnel::tor::{TorConfig, TorNode};

        let tor_config = TorConfig::default()
            .map_err(|e| anyhow::anyhow!("Failed to create Tor config: {}", e))?
            .with_data_dir(data_dir.join("tor"))
            .with_local_port(args.port);

        let mut tor_node = TorNode::new(tor_config);
        
        match tor_node.start().await {
            Ok(onion_addr) => {
                println!("✅ Tor: http://{}.onion", onion_addr);
                Some(tor_node)
            }
            Err(e) => {
                warn!("Failed to start Tor: {}", e);
                println!("⚠️  Tor: Failed ({})", e);
                None
            }
        }
    } else {
        println!("⏸️  Tor: Disabled (use --tor to enable)");
        None
    };

    println!();
    println!("🌐 HTTP Server: http://0.0.0.0:{}", args.port);
    println!();
    println!("Press Ctrl+C to stop");
    println!();

    // Run the HTTP server
    server::run(args.port).await?;

    // Cleanup
    if let Some(mut t) = tunnel {
        info!("Stopping tunnel...");
        t.stop().await?;
    }

    Ok(())
}
