//! List discovered peers

use std::path::Path;
use clap::Args;

#[derive(Args, Debug)]
pub struct PeersArgs {
    /// Output format (table, json)
    #[arg(long, default_value = "table")]
    pub format: String,
}

/// Run the peers command
pub async fn run(args: PeersArgs, _data_dir: &Path) -> anyhow::Result<()> {
    // TODO: Load peers from database or discovery service

    if args.format == "json" {
        println!("[]");
    } else {
        println!("🔍 Discovered Peers");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("   No peers discovered yet.");
        println!();
        println!("💡 Make sure the server is running:");
        println!("   edge-hive serve --discovery");
    }

    Ok(())
}
