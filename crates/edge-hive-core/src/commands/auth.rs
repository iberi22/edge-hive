//! OAuth2 client management commands

use clap::Args;
use edge_hive_auth::{
    oauth2::ClientCredentials,
    client::ClientStore,
};
use std::path::Path;
use std::sync::Arc;
use anyhow::Result;

#[derive(Args, Debug)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub command: AuthCommands,
}

#[derive(clap::Subcommand, Debug)]
pub enum AuthCommands {
    /// Manage OAuth2 clients
    Client(ClientCommand),
}

#[derive(Args, Debug)]
pub struct ClientCommand {
    #[command(subcommand)]
    pub action: ClientAction,
}

#[derive(clap::Subcommand, Debug)]
pub enum ClientAction {
    /// Create a new OAuth2 client
    Create {
        /// Client name
        #[arg(short, long)]
        name: String,

        /// Scopes (comma-separated)
        #[arg(short, long, default_value = "mcp:read,mcp:call")]
        scopes: String,
    },

    /// List all OAuth2 clients
    List {
        /// Show revoked clients
        #[arg(short, long)]
        all: bool,
    },

    /// Revoke an OAuth2 client
    Revoke {
        /// Client ID to revoke
        client_id: String,
    },

    /// Delete an OAuth2 client permanently
    Delete {
        /// Client ID to delete
        client_id: String,

        /// Force deletion without confirmation
        #[arg(short, long)]
        force: bool,
    },
}

/// Run the auth command
pub async fn run(args: AuthArgs, data_dir: &Path) -> Result<()> {
    match args.command {
        AuthCommands::Client(client_cmd) => run_client_command(client_cmd, data_dir).await,
    }
}

async fn run_client_command(cmd: ClientCommand, data_dir: &Path) -> Result<()> {
    // Initialize client store (TODO: use SurrealDB instead of in-memory)
    let client_store = Arc::new(ClientStore::new());

    // Load existing clients from file if exists
    let clients_file = data_dir.join("oauth_clients.json");
    if clients_file.exists() {
        let data = std::fs::read_to_string(&clients_file)?;
        let clients: Vec<ClientCredentials> = serde_json::from_str(&data)?;
        for client in clients {
            client_store.add_client(client).await?;
        }
    }

    match cmd.action {
        ClientAction::Create { name, scopes } => {
            create_client(client_store, name, scopes, &clients_file).await?;
        }
        ClientAction::List { all } => {
            list_clients(client_store, all).await?;
        }
        ClientAction::Revoke { client_id } => {
            revoke_client(client_store, client_id, &clients_file).await?;
        }
        ClientAction::Delete { client_id, force } => {
            delete_client(client_store, client_id, force, &clients_file).await?;
        }
    }

    Ok(())
}

async fn create_client(
    store: Arc<ClientStore>,
    name: String,
    scopes_str: String,
    clients_file: &Path,
) -> Result<()> {
    let scopes: Vec<String> = scopes_str
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    // Generate credentials
    let client_id = ClientCredentials::generate_client_id();
    let client_secret = ClientCredentials::generate_client_secret();

    let credentials = ClientCredentials::new(
        client_id.clone(),
        &client_secret,
        scopes.clone(),
        name.clone(),
    );

    // Add to store
    store.add_client(credentials).await?;

    // Save to file
    save_clients(&store, clients_file).await?;

    // Display credentials
    println!("\n✅ OAuth2 Client Created Successfully!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Name:          {}", name);
    println!("🔑 Client ID:     {}", client_id);
    println!("🔐 Client Secret: {}", client_secret);
    println!("🎯 Scopes:        {}", scopes.join(", "));
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n⚠️  IMPORTANT: Save the client secret securely!");
    println!("   It will NOT be shown again.\n");
    println!("📝 Example usage:");
    println!("   curl -X POST http://localhost:8080/mcp/auth/token \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{");
    println!("       \"grant_type\": \"client_credentials\",");
    println!("       \"client_id\": \"{}\",", client_id);
    println!("       \"client_secret\": \"{}\"", client_secret);
    println!("     }}'\n");

    Ok(())
}

async fn list_clients(store: Arc<ClientStore>, show_all: bool) -> Result<()> {
    let clients = store.list_clients().await?;

    let filtered: Vec<_> = if show_all {
        clients
    } else {
        clients.into_iter().filter(|c| !c.revoked).collect()
    };

    if filtered.is_empty() {
        println!("📭 No OAuth2 clients found.");
        println!("   Create one with: edge-hive auth client create --name <name>\n");
        return Ok(());
    }

    println!("\n📋 OAuth2 Clients ({} total)", filtered.len());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    for client in filtered {
        let status = if client.revoked { "❌ REVOKED" } else { "✅ Active" };
        let created = chrono::DateTime::from_timestamp(client.created_at, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        println!("\n  Name:       {}", client.name);
        println!("  Client ID:  {}", client.client_id);
        println!("  Status:     {}", status);
        println!("  Scopes:     {}", client.scopes.join(", "));
        println!("  Created:    {}", created);
        println!("  ─────────────────────────────────────────────────────────────────────");
    }

    println!();
    Ok(())
}

async fn revoke_client(
    store: Arc<ClientStore>,
    client_id: String,
    clients_file: &Path,
) -> Result<()> {
    match store.revoke_client(&client_id).await {
        Ok(_) => {
            save_clients(&store, clients_file).await?;
            println!("\n✅ Client revoked successfully: {}\n", client_id);
            println!("   The client can no longer obtain access tokens.");
            println!("   To permanently delete, use: edge-hive auth client delete {}\n", client_id);
        }
        Err(e) => {
            eprintln!("\n❌ Failed to revoke client: {}\n", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn delete_client(
    store: Arc<ClientStore>,
    client_id: String,
    force: bool,
    clients_file: &Path,
) -> Result<()> {
    if !force {
        println!("\n⚠️  WARNING: This will permanently delete the client!");
        println!("   Client ID: {}\n", client_id);
        print!("   Type 'yes' to confirm: ");

        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim() != "yes" {
            println!("\n❌ Deletion cancelled.\n");
            return Ok(());
        }
    }

    match store.delete_client(&client_id).await {
        Ok(_) => {
            save_clients(&store, clients_file).await?;
            println!("\n✅ Client deleted permanently: {}\n", client_id);
        }
        Err(e) => {
            eprintln!("\n❌ Failed to delete client: {}\n", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn save_clients(store: &ClientStore, clients_file: &Path) -> Result<()> {
    let clients = store.list_clients().await?;
    let json = serde_json::to_string_pretty(&clients)?;
    std::fs::write(clients_file, json)?;
    Ok(())
}
