//! Example: Using the Tor Client
//!
//! This example demonstrates how to use the TorService to connect to an
//! onion service and fetch some data.

use edge_hive_tunnel::{TorConfig, TorService};
use std::path::PathBuf;
use tokio::io::AsyncReadExt;
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 Starting Tor Client Example");

    // Create configuration
    let config = TorConfig {
        data_dir: PathBuf::from("./example-tor-data"),
        enabled: true,
    };

    // Create and start service
    let mut tor_service = TorService::new(config);
    
    info!("🧅 Starting Tor client...");
    if let Err(e) = tor_service.start().await {
        eprintln!("❌ Failed to start Tor client: {}", e);
        return Err(e.into());
    }

    info!("✅ Tor client started successfully!");

    // Let's connect to the DuckDuckGo onion service
    let onion_address = "duckduckgogg42xjoc72x3sjasowoarfbgcmvfimaftt6twagswzczad.onion";
    let port = 80;

    info!("🧅 Connecting to DuckDuckGo onion service...");
    match tor_service.connect_onion(onion_address, port).await {
        Ok(mut stream) => {
            info!("✅ Connected!");
            
            // Send a simple HTTP GET request
            let request = format!(
                "GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
                onion_address
            );
            
            tokio::io::AsyncWriteExt::write_all(&mut stream, request.as_bytes()).await?;

            // Read the response
            let mut response = String::new();
            stream.read_to_string(&mut response).await?;

            info!("📝 Received response (first 200 chars):\n{}", &response[..200]);
        }
        Err(e) => {
            eprintln!("❌ Failed to connect to onion service: {}", e);
        }
    }

    // Stop the service
    info!("🛑 Stopping service...");
    tor_service.stop().await?;
    info!("✅ Service stopped");

    Ok(())
}
