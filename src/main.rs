//! Edge Hive - Decentralized edge computing platform
//!
//! CLI for managing Edge Hive nodes.

use anyhow::Result;
use clap::{Parser, Subcommand};
use edge_hive_identity::NodeIdentity;
use std::path::PathBuf;
use tracing::{info, error};
use tracing_subscriber;

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

        /// Enable Tor onion service
        #[arg(long)]
        tor: bool,
    },

    /// Show node status
    Status,
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let level = if cli.verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .init();

    match cli.command {
        Commands::Identity { action } => handle_identity(action, &cli.config_dir)?,
        Commands::Start { port, tor } => handle_start(port, tor, &cli.config_dir)?,
        Commands::Status => handle_status(&cli.config_dir)?,
    }

    Ok(())
}

fn handle_identity(action: IdentityCommands, config_dir: &PathBuf) -> Result<()> {
    let identity_path = expand_path(config_dir).join("identity.key");

    match action {
        IdentityCommands::New { name } => {
            info!("Generating new identity...");
            let mut identity = NodeIdentity::generate()?;
            
            if let Some(custom_name) = name {
                info!("Using custom name: {}", custom_name);
                // Note: We'd need to add set_name() method to NodeIdentity
            }

            std::fs::create_dir_all(&identity_path.parent().unwrap())?;
            identity.save(&identity_path)?;

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
                error!("No identity found. Run: edge-hive identity new");
                std::process::exit(1);
            }

            let identity = NodeIdentity::load(&identity_path)?;
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
                let identity = NodeIdentity::load(&identity_path)?;
                println!("  • {} (current)", identity.name());
            } else {
                println!("  (none found)");
            }
        }
    }

    Ok(())
}

fn handle_start(port: u16, _tor: bool, config_dir: &PathBuf) -> Result<()> {
    let identity_path = expand_path(config_dir).join("identity.key");

    if !identity_path.exists() {
        error!("No identity found. Run: edge-hive identity new");
        std::process::exit(1);
    }

    let identity = NodeIdentity::load(&identity_path)?;
    info!("Starting Edge Hive node: {}", identity.name());
    info!("Identity loaded: {}", identity.peer_id());

    // TODO: Start Axum server, Tor, libp2p
    println!("🚀 Edge Hive node starting...");
    println!("   Name:    {}", identity.name());
    println!("   Port:    {}", port);
    println!("   Peer ID: {}", identity.peer_id());
    println!();
    println!("⚠️  Core server not implemented yet");
    println!("   Track progress: https://github.com/your-org/edge-hive/issues");

    Ok(())
}

fn handle_status(_config_dir: &PathBuf) -> Result<()> {
    println!("📊 Edge Hive Status");
    println!();
    println!("  Version:    0.1.0");
    println!("  Rust:       {}", env!("CARGO_PKG_RUST_VERSION"));
    println!();
    println!("⚠️  Status endpoint not implemented yet");

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
