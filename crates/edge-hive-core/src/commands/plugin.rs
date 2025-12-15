//! Plugin management commands

use std::path::Path;
use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct PluginArgs {
    #[command(subcommand)]
    pub command: PluginCommands,
}

#[derive(Subcommand, Debug)]
pub enum PluginCommands {
    /// List installed plugins
    List,
    /// Install a plugin
    Install {
        /// URL or path to WASM module
        source: String
    },
    /// Remove a plugin
    Remove {
        /// Name of the plugin
        name: String
    },
}

pub async fn run(args: PluginArgs, data_dir: &Path) -> anyhow::Result<()> {
    match args.command {
        PluginCommands::List => list(data_dir).await,
        PluginCommands::Install { source } => install(data_dir, &source).await,
        PluginCommands::Remove { name } => remove(data_dir, &name).await,
    }
}

/// List installed plugins
async fn list(data_dir: &Path) -> anyhow::Result<()> {
    let plugins_dir = data_dir.join("plugins");

    println!("🔌 Installed Plugins");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    if !plugins_dir.exists() {
        println!("   No plugins installed");
        println!();
        println!("📥 Install a plugin:");
        println!("   edge-hive plugin install <URL or path>");
        return Ok(());
    }

    // TODO: List plugins from plugins directory
    println!("   No plugins installed");

    Ok(())
}

/// Install a plugin
async fn install(data_dir: &Path, source: &str) -> anyhow::Result<()> {
    println!("📥 Installing plugin from: {}", source);

    let plugins_dir = data_dir.join("plugins");
    std::fs::create_dir_all(&plugins_dir)?;

    // TODO: Download and validate WASM plugin
    println!("⚠️  Plugin installation not yet implemented");

    Ok(())
}

/// Remove a plugin
async fn remove(_data_dir: &Path, name: &str) -> anyhow::Result<()> {
    println!("🗑️  Removing plugin: {}", name);

    // TODO: Remove plugin from plugins directory
    println!("⚠️  Plugin removal not yet implemented");

    Ok(())
}
