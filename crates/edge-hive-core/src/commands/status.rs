//! Show node status
use anyhow::{bail, Ok};
use clap::Args;
use edge_hive_identity::NodeIdentity;
use std::path::Path;

#[derive(Args, Debug)]
pub struct StatusArgs {}

/// Run the status command
pub async fn run(_args: StatusArgs, data_dir: &Path) -> anyhow::Result<()> {
    let identity_path = data_dir.join("identity.key");

    if !identity_path.exists() {
        bail!("Identity file not found. Run `edge-hive init` first.");
    }

    let identity = NodeIdentity::load(&identity_path, None)?;
    println!("Node Status:");
    println!("  Peer ID: {}", identity.peer_id());
    println!("  Name: {}", identity.name());

    Ok(())
}
