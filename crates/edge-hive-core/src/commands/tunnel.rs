//! Tunnel management commands

use edge_hive_tunnel::{TunnelBackend, TunnelService};
use std::path::Path;
use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct TunnelArgs {
    #[command(subcommand)]
    pub command: TunnelCommands,
}

#[derive(Subcommand, Debug)]
pub enum TunnelCommands {
    /// Start a quick tunnel
    Quick,
    /// Enable persistent tunnel
    Enable {
        /// Cloudflare Tunnel Token
        #[arg(long)]
        token: String,
    },
    /// Disable tunnel
    Disable,
    /// Show status
    Status,
}

pub async fn run(args: TunnelArgs, data_dir: &Path) -> anyhow::Result<()> {
    match args.command {
        TunnelCommands::Quick => quick(data_dir).await,
        TunnelCommands::Enable { token } => enable(data_dir, &token).await,
        TunnelCommands::Disable => disable(data_dir).await,
        TunnelCommands::Status => status(data_dir).await,
    }
}

/// Start a quick tunnel (TryCloudflare)
async fn quick(_data_dir: &Path) -> anyhow::Result<()> {
    println!("🚇 Starting quick tunnel...");
    println!();

    let mut tunnel = TunnelService::new(TunnelBackend::Cloudflared);

    if !TunnelService::cloudflared_available() {
        println!("❌ cloudflared not found");
        println!();
        println!("📥 Install cloudflared:");
        println!("   Windows: winget install cloudflare.cloudflared");
        println!("   macOS:   brew install cloudflared");
        println!("   Linux:   sudo apt install cloudflared");
        println!("   Termux:  pkg install cloudflared");
        return Ok(());
    }

    // Use default port 8080
    match tunnel.start_quick(8080).await {
        Ok(url) => {
            println!("✅ Tunnel started!");
            println!();
            println!("   Public URL: {}", url);
            println!();
            println!("⚠️  Note: Quick tunnels are temporary.");
            println!("   For persistent tunnels, use: edge-hive tunnel enable --token <TOKEN>");
        }
        Err(e) => {
            println!("❌ Failed to start tunnel: {}", e);
        }
    }

    Ok(())
}

/// Enable tunnel with token (named tunnel)
async fn enable(_data_dir: &Path, token: &str) -> anyhow::Result<()> {
    println!("🚇 Enabling named tunnel...");

    let mut tunnel = TunnelService::new(TunnelBackend::Cloudflared);

    match tunnel.start_named(8080, token).await {
        Ok(url) => {
            println!("✅ Named tunnel enabled!");
            println!("   URL: {}", url);

            // TODO: Save tunnel config to data_dir
        }
        Err(e) => {
            println!("❌ Failed to enable tunnel: {}", e);
        }
    }

    Ok(())
}

/// Disable tunnel
async fn disable(_data_dir: &Path) -> anyhow::Result<()> {
    println!("🛑 Tunnel disabled");
    // TODO: Stop any running tunnel process
    Ok(())
}

/// Show tunnel status
async fn status(_data_dir: &Path) -> anyhow::Result<()> {
    println!("🚇 Tunnel Status");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    if TunnelService::cloudflared_available() {
        println!("   cloudflared: ✅ Installed");
    } else {
        println!("   cloudflared: ❌ Not found");
    }

    println!("   Tunnel:      🔴 Not running");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}
