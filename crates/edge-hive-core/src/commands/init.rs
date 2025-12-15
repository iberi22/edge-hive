//! Initialize node identity

use edge_hive_identity::NodeIdentity;
use std::path::Path;
use tracing::info;
use clap::Args;

#[derive(Args, Debug)]
pub struct InitArgs {
    /// custom node name
    #[arg(short, long)]
    pub name: Option<String>,

    /// force overwrite existing identity
    #[arg(short, long)]
    pub force: bool,
}

/// Run the init command
pub async fn run(args: InitArgs, data_dir: &Path) -> anyhow::Result<()> {
    let identity_path = data_dir.join("identity.key");

    // Check if identity already exists
    if identity_path.exists() && !args.force {
        println!("⚠️  Identity already exists at {:?}", identity_path);
        println!("   Use --force to regenerate");

        // Load and show existing identity
        let identity = NodeIdentity::load(&identity_path, None)?;
        println!("\n📋 Current Identity:");
        println!("   Name:    {}", identity.name());
        println!("   Peer ID: {}", identity.peer_id());
        return Ok(());
    }

    info!("🔑 Generating new node identity...");

    // Generate new identity
    let mut identity = NodeIdentity::generate()?;

    // Set custom name if provided (TODO: add set_name to identity crate if needed, or re-gen with seed)
    // For now we just use the name for display if passed, actual logic would go here
    if let Some(n) = args.name {
        println!("TODO: Custom name support: {}", n);
    }

    // Save identity
    identity.save(&identity_path, None)?;

    let public = identity.public_identity();

    println!("✅ Node identity created!");
    println!();
    println!("   Name:       {}", public.name);
    println!("   Peer ID:    {}", public.peer_id);
    println!("   Public Key: {}...", &public.public_key[..32]);
    println!("   Created:    {}", public.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
    println!();
    println!("📁 Identity saved to: {:?}", identity_path);
    println!();
    println!("🚀 Next steps:");
    println!("   1. Start server:  edge-hive serve");
    println!("   2. Enable tunnel: edge-hive tunnel quick");

    Ok(())
}
