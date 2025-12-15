//! List discovered peers

use clap::Args;
use std::path::Path;
use reqwest;
use serde::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Args, Debug)]
pub struct PeersArgs {
    /// API server address
    #[arg(short, long, default_value = "http://127.0.0.1:8080")]
    pub api_server: String,
}

#[derive(Debug, Deserialize)]
pub struct PeerInfo {
    pub peer_id: String,
    pub addresses: Vec<String>,
    pub last_seen: DateTime<Utc>,
    pub source: String,
}

/// Run the peers command
pub async fn run(args: PeersArgs, _data_dir: &Path) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/v1/peers", args.api_server);

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
                        println!("  Source: {}", peer.source);
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
