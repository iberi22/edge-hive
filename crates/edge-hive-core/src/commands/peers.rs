//! List discovered peers

use clap::Args;
use std::path::Path;

#[derive(Args, Debug)]
pub struct PeersArgs {}

/// Run the peers command
pub async fn run(_args: PeersArgs, _data_dir: &Path) -> anyhow::Result<()> {
    // TODO: Implement peer discovery and listing
    println!("No peers discovered yet.");
    Ok(())
}
