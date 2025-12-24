//! Example: Starting a Tor Onion Service
//!
//! This example demonstrates how to use the TorService API to expose
//! a local service through Tor.

use edge_hive_tunnel::{TorConfig, TorService};
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 Starting Tor Onion Service Example");

    // Create configuration
    let config = TorConfig {
        data_dir: PathBuf::from("./example-tor-data"),
        local_port: 8080,
        nickname: Some("example-node".to_string()),
        enabled: true,
    };

    // Create and start service
    let mut tor_service = TorService::new(config);
    
    info!("🧅 Starting Tor service...");
    match tor_service.start().await {
        Ok(onion_address) => {
            info!("✅ Onion service started successfully!");
            info!("🧅 Your onion address: http://{}.onion", onion_address);
            info!("📝 Share this address to allow anonymous access");
            
            // Keep service running for 30 seconds
            info!("⏳ Service will run for 30 seconds...");
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            
            // Stop the service
            info!("🛑 Stopping service...");
            tor_service.stop().await?;
            info!("✅ Service stopped");
        }
        Err(e) => {
            eprintln!("❌ Failed to start Tor service: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
