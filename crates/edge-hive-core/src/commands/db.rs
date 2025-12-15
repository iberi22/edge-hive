//! Database management commands

use clap::{Args, Subcommand};
use std::path::Path;

#[derive(Args, Debug)]
pub struct DbArgs {
    #[command(subcommand)]
    pub command: DbCommands,
}

#[derive(Subcommand, Debug)]
pub enum DbCommands {
    /// Show database status
    Status,
    /// Backup the database
    Backup(BackupArgs),
    /// Restore the database
    Restore(RestoreArgs),
}

#[derive(Args, Debug)]
pub struct BackupArgs {
    /// Path to backup file
    pub path: String,
}

#[derive(Args, Debug)]
pub struct RestoreArgs {
    /// Path to backup file
    pub path: String,
}

use edge_hive_db::DatabaseService;

pub async fn run(args: DbArgs, data_dir: &Path) -> anyhow::Result<()> {
    let db_path = data_dir.join("db");
    let db = DatabaseService::new(&db_path).await?;

    match args.command {
        DbCommands::Status => {
            let status = db.query("INFO FOR DB;").await?;
            println!("Database Status:\n{:#?}", status);
        }
        DbCommands::Backup(backup_args) => {
            println!("Backing up database to {}...", backup_args.path);
            db.query(&format!("EXPORT DATABASE '{}'", backup_args.path)).await?;
            println!("Backup complete.");
        }
        DbCommands::Restore(restore_args) => {
            println!("Restoring database from {}...", restore_args.path);
            db.query(&format!("IMPORT DATABASE '{}'", restore_args.path)).await?;
            println!("Restore complete.");
        }
    }
    Ok(())
}
