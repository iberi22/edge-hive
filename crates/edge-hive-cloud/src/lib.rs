//! Edge Hive Cloud - AWS provisioning for managed nodes
//!
//! Automatically provisions EC2 instances with Edge Hive pre-installed.
//!
//! Automatically provisions EC2 instances with Edge Hive pre-installed.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;
use aws_sdk_ec2::{Client as Ec2Client, types::{InstanceType, Tag, TagSpecification, ResourceType}};
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};

/// Errors that can occur during cloud operations
#[derive(Debug, Error)]
pub enum CloudError {
    #[error("AWS API error: {0}")]
    AwsApi(#[from] aws_sdk_ec2::Error),

    #[error("Instance not found: {0}")]
    InstanceNotFound(String),

    #[error("Provisioning failed: {0}")]
    ProvisioningFailed(String),

    #[error("Quota exceeded: {0}")]
    QuotaExceeded(String),
}

/// Supported AWS regions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, clap::ValueEnum)]
pub enum Region {
    #[serde(rename = "us-east-1")]
    UsEast1,
    #[serde(rename = "us-west-2")]
    UsWest2,
    #[serde(rename = "eu-west-1")]
    EuWest1,
    #[serde(rename = "ap-southeast-1")]
    ApSoutheast1,
}

impl Region {
    pub fn as_str(&self) -> &'static str {
        match self {
            Region::UsEast1 => "us-east-1",
            Region::UsWest2 => "us-west-2",
            Region::EuWest1 => "eu-west-1",
            Region::ApSoutheast1 => "ap-southeast-1",
        }
    }
}

/// Instance size options
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, clap::ValueEnum)]
pub enum InstanceSize {
    /// t4g.small - 2 vCPU, 2 GB RAM (~$12/mo)
    Small,
    /// t4g.medium - 2 vCPU, 4 GB RAM (~$24/mo)
    Medium,
    /// t4g.large - 2 vCPU, 8 GB RAM (~$48/mo)
    Large,
}

impl InstanceSize {
    pub fn instance_type(&self) -> InstanceType {
        match self {
            InstanceSize::Small => InstanceType::T4gSmall,
            InstanceSize::Medium => InstanceType::T4gMedium,
            InstanceSize::Large => InstanceType::T4gLarge,
        }
    }

    pub fn monthly_cost_cents(&self) -> u64 {
        match self {
            InstanceSize::Small => 1200,   // $12
            InstanceSize::Medium => 2400,  // $24
            InstanceSize::Large => 4800,   // $48
        }
    }
}

/// Configuration for provisioning a new node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionConfig {
    pub user_id: String,
    pub node_name: String,
    pub region: Region,
    pub size: InstanceSize,
    pub storage_gb: u32,
    pub cf_tunnel_token: Option<String>,
}

/// A provisioned cloud node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudNode {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub instance_id: String,
    pub region: Region,
    pub size: InstanceSize,
    pub public_ip: Option<String>,
    pub tunnel_url: Option<String>,
    pub peer_id: Option<String>,
    pub status: NodeStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub storage_gb: u32,
}

/// Node status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum NodeStatus {
    Pending,
    Provisioning,
    Running,
    Stopping,
    Stopped,
    Terminating,
    Terminated,
    Error,
}

/// AWS Provisioner service
pub struct AWSProvisioner {
    ec2_client: Ec2Client,
}

impl AWSProvisioner {
    /// Create a new AWS provisioner
    pub async fn new(region: Region) -> Result<Self, CloudError> {
        info!("☁️ AWS Provisioner initialized");
        let region_provider = RegionProviderChain::first_try(region.as_str());
        let config = aws_config::defaults(BehaviorVersion::latest()).region(region_provider).load().await;
        let ec2_client = Ec2Client::new(&config);
        Ok(Self { ec2_client })
    }

    /// Provision a new Edge Hive node
    pub async fn provision_node(&self, config: ProvisionConfig) -> Result<CloudNode, CloudError> {
        info!("🚀 Provisioning node '{}' for user {} in {}",
            config.node_name, config.user_id, config.region.as_str());

        let user_data = generate_user_data("placeholder-token", config.cf_tunnel_token.as_deref());
        use base64::{Engine as _, engine::general_purpose};
        let encoded_user_data = general_purpose::STANDARD.encode(user_data);

        let run_instances_output = self.ec2_client.run_instances()
            .image_id("ami-0c55b159cbfafe1f0") // Amazon Linux 2 AMI
            .instance_type(config.size.instance_type())
            .min_count(1)
            .max_count(1)
            .user_data(&encoded_user_data)
            .tag_specifications(
                TagSpecification::builder()
                    .resource_type(ResourceType::Instance)
                    .tags(
                        Tag::builder()
                            .key("Name")
                            .value(&config.node_name)
                            .build()
                    )
                    .build()
            )
            .send()
            .await
            .map_err(|e| CloudError::ProvisioningFailed(e.to_string()))?;

        let instance = run_instances_output.instances()
            .first()
            .cloned()
            .ok_or_else(|| CloudError::ProvisioningFailed("No instances returned".to_string()))?;

        let instance_id = instance.instance_id().ok_or_else(|| CloudError::ProvisioningFailed("No instance ID returned".to_string()))?.to_string();

        let node_id = generate_node_id();

        Ok(CloudNode {
            id: node_id.clone(),
            user_id: config.user_id,
            name: config.node_name,
            instance_id,
            region: config.region,
            size: config.size,
            public_ip: instance.public_ip_address().map(|s: &str| s.to_string()),
            tunnel_url: Some(format!("https://{}.edge-hive.io", node_id)),
            peer_id: None,
            status: NodeStatus::Provisioning,
            created_at: chrono::Utc::now(),
            storage_gb: config.storage_gb,
        })
    }
}

/// Generate EC2 user-data script for node installation
pub fn generate_user_data(user_token: &str, cf_tunnel_token: Option<&str>) -> String {
    let tunnel_config = if let Some(token) = cf_tunnel_token {
        format!(r#"
# Configure Cloudflare Tunnel
edge-hive tunnel enable --token "{}"
"#, token)
    } else {
        String::new()
    };

    format!(r#"#!/bin/bash
set -e

# Log everything
exec > >(tee /var/log/edge-hive-install.log) 2>&1

echo "🐝 Installing Edge Hive..."

# Install Edge Hive
curl -sSL https://edge-hive.io/install.sh | bash

# Initialize with cloud token
edge-hive init --cloud-token "{}"
{}
# Enable and start service
systemctl enable edge-hive
systemctl start edge-hive

echo "✅ Edge Hive installation complete!"
"#, user_token, tunnel_config)
}

/// Generate a random node ID
fn generate_node_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:012x}", nanos % 0xffffffffffff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_pricing() {
        assert_eq!(InstanceSize::Small.monthly_cost_cents(), 1200);
        assert_eq!(InstanceSize::Medium.monthly_cost_cents(), 2400);
    }

    #[test]
    fn test_user_data_generation() {
        let script = generate_user_data("token_123", Some("cf_token_456"));
        assert!(script.contains("edge-hive init"));
        assert!(script.contains("cf_token_456"));
    }

    #[tokio::test]
    #[ignore]
    async fn test_provision_node_aws() {
        let config = ProvisionConfig {
            user_id: "test-user".to_string(),
            node_name: "edge-hive-test-node".to_string(),
            region: Region::UsEast1,
            size: InstanceSize::Small,
            storage_gb: 20,
            cf_tunnel_token: None,
        };

        let provisioner = AWSProvisioner::new(config.region).await.unwrap();
        let result = provisioner.provision_node(config).await;

        assert!(result.is_ok());

        let node = result.unwrap();
        assert_eq!(node.name, "edge-hive-test-node");
        assert!(node.instance_id.starts_with("i-"));
    }
}
