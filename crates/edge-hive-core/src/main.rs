use clap::{Parser, Subcommand};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use directories::ProjectDirs;

mod config;
mod auth;
mod server;
mod tls;
pub mod commands {
    pub mod init;
    pub mod serve;
    pub mod ping;
    pub mod status;
    pub mod peers;
    pub mod tunnel;
    pub mod plugin;
    pub mod mcp; // Added MCP module
    pub mod auth; // OAuth2 client management
    pub mod cloud;
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Verbose output (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Config file path
    #[arg(short, long)]
    config: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a new node identity
    Init(commands::init::InitArgs),

    /// Start the Edge Hive server
    Serve(commands::serve::ServeArgs),

    /// Show node status
    Status(commands::status::StatusArgs),

    /// List discovered peers
    Peers(commands::peers::PeersArgs),

    /// Manage Cloudflare tunnel
    Tunnel(commands::tunnel::TunnelArgs),

    /// Manage plugins
    Plugin(commands::plugin::PluginArgs),

    /// Run as MCP Server (Model Context Protocol)
    Mcp(commands::mcp::McpArgs),

    /// Manage OAuth2 authentication
    Auth(commands::auth::AuthArgs),

    /// Ping the server to check if it's running
    Ping(commands::ping::PingArgs),

    /// Manage cloud nodes
    Cloud(commands::cloud::CloudArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Initialize logging (skip for MCP to avoid polluting stdout)
    if !matches!(args.command, Commands::Mcp(_)) {
        let log_level = match args.verbose {
            0 => Level::INFO,
            1 => Level::DEBUG,
            _ => Level::TRACE,
        };

        let subscriber = FmtSubscriber::builder()
            .with_max_level(log_level)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        tracing::info!("🐝 Edge Hive v{}", env!("CARGO_PKG_VERSION"));
    }

    // Get data directory
    let data_dir = get_data_dir();
    if !matches!(args.command, Commands::Mcp(_)) {
        std::fs::create_dir_all(&data_dir)?;
    }

    match args.command {
        Commands::Init(a) => commands::init::run(a, &data_dir).await?,
        Commands::Serve(a) => commands::serve::run(a, &data_dir).await?,
        Commands::Status(a) => commands::status::run(a, &data_dir).await?,
        Commands::Peers(a) => commands::peers::run(a, &data_dir).await?,
        Commands::Tunnel(a) => commands::tunnel::run(a, &data_dir).await?,
        Commands::Plugin(a) => commands::plugin::run(a, &data_dir).await?,
        Commands::Mcp(a) => commands::mcp::run(a).await?, // Run MCP
        Commands::Auth(a) => commands::auth::run(a, &data_dir).await?, // OAuth2 management
        Commands::Ping(a) => commands::ping::run(a).await?,
        Commands::Cloud(a) => commands::cloud::handle_cloud_command(a).await?,
    }

    Ok(())
}

fn get_data_dir() -> std::path::PathBuf {
    // Check environment variable first
    if let Ok(dir) = std::env::var("EDGE_HIVE_DATA_DIR") {
        return std::path::PathBuf::from(dir);
    }

    // Check if running in Termux
    #[cfg(target_os = "android")]
    {
        if std::env::var("TERMUX_VERSION").is_ok() {
            return std::path::PathBuf::from("/data/data/com.termux/files/home/.edge-hive");
        }
    }

    // Use standard directories
    if let Some(proj_dirs) = ProjectDirs::from("io", "edge-hive", "edge-hive") {
        return proj_dirs.data_local_dir().to_path_buf();
    }

    // Fallback to home directory
    if let Some(user_dirs) = directories::UserDirs::new() {
        return user_dirs.home_dir().join(".edge-hive");
    }

    // Final fallback
    std::path::PathBuf::from(".edge-hive")
}
