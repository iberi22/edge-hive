//! Show node status

use edge_hive_identity::NodeIdentity;
use std::path::Path;
use clap::Args;

#[derive(Args, Debug)]
pub struct StatusArgs {
    /// Output format (table, json)
    #[arg(long, default_value = "table")]
    pub format: String,
}

/// Run the status command
pub async fn run(args: StatusArgs, data_dir: &Path) -> anyhow::Result<()> {
    let identity_path = data_dir.join("identity.key");

    if !identity_path.exists() {
        println!("❌ No node identity found");
        println!("   Run: edge-hive init");
        return Ok(());
    }

    let identity = NodeIdentity::load(&identity_path)?;
    let public = identity.public_identity();

    if args.format == "json" {
        let json = serde_json::json!({
            "name": public.name,
            "peer_id": public.peer_id,
            "public_key": public.public_key,
            "created_at": public.created_at.to_rfc3339(),
            "status": "stopped",
            "peers": 0
        });
        println!("{}", serde_json::to_string_pretty(&json)?);
    } else {
        println!("🐝 Edge Hive Status");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("   Node:       {}", public.name);
        println!("   Peer ID:    {}", public.peer_id);
        println!("   Status:     🔴 Stopped");
        println!("   Peers:      0");
        println!("   Uptime:     N/A");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!();
        println!("💡 Start the server with: edge-hive serve");
    }

    Ok(())
}
