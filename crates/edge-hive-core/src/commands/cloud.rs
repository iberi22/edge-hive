use clap::{Args, Subcommand};
use edge_hive_cloud::{AWSProvisioner, ProvisionConfig, Region, InstanceSize};

#[derive(Args, Debug)]
pub struct CloudArgs {
    #[clap(subcommand)]
    pub command: CloudCommand,
}

#[derive(Subcommand, Debug)]
pub enum CloudCommand {
    /// Provision a new cloud node
    Provision(ProvisionArgs),
}

#[derive(Args, Debug)]
pub struct ProvisionArgs {
    #[clap(long)]
    pub node_name: String,
    #[clap(long)]
    pub region: Region,
    #[clap(long)]
    pub size: InstanceSize,
    #[clap(long, default_value = "20")]
    pub storage_gb: u32,
}

pub async fn handle_cloud_command(args: CloudArgs) -> anyhow::Result<()> {
    match args.command {
        CloudCommand::Provision(provision_args) => {
            provision_cloud_node(provision_args).await?;
        }
    }
    Ok(())
}

async fn provision_cloud_node(args: ProvisionArgs) -> anyhow::Result<()> {
    let provisioner = AWSProvisioner::new().await?;
    let config = ProvisionConfig {
        user_id: "local-user".to_string(), // Placeholder
        node_name: args.node_name,
        region: args.region,
        size: args.size,
        storage_gb: args.storage_gb,
        cf_tunnel_token: None, // Placeholder
    };

    let node = provisioner.provision_node(config).await?;

    println!("Successfully provisioned cloud node:");
    println!("{:#?}", node);

    Ok(())
}
