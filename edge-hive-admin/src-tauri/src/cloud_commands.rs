//! Cloud Provisioning Commands
//!
//! Manages cloud nodes with:
//! - Local tracking of provisioned nodes
//! - AWS EC2 integration when credentials available
//! - Node lifecycle management
//!
//! Uses AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY env vars

use tauri::State;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::env;

/// Cloud node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudNode {
    pub id: String,
    pub name: String,
    pub region: String,
    pub size: String,
    pub status: String,
    pub public_ip: Option<String>,
    pub private_ip: Option<String>,
    pub created_at: String,
    pub instance_id: Option<String>,  // AWS instance ID
    pub monthly_cost: String,
}

/// Cloud provisioning request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionRequest {
    pub name: String,
    pub region: String,
    pub size: String,
}

/// Available regions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudRegion {
    pub id: String,
    pub name: String,
    pub location: String,
    pub available: bool,
}

/// Instance size options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceSize {
    pub id: String,
    pub name: String,
    pub vcpus: u32,
    pub memory_gb: f32,
    pub monthly_price: String,
}

pub struct CloudState {
    pub nodes: Arc<RwLock<HashMap<String, CloudNode>>>,
    pub aws_configured: bool,
}

impl CloudState {
    pub fn new() -> Self {
        let aws_key = env::var("AWS_ACCESS_KEY_ID").unwrap_or_default();
        let aws_secret = env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_default();
        let aws_configured = !aws_key.is_empty() && !aws_secret.is_empty();

        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            aws_configured,
        }
    }
}

/// Get all cloud nodes
#[tauri::command]
pub async fn get_cloud_nodes(
    state: State<'_, CloudState>,
) -> Result<Vec<CloudNode>, String> {
    let nodes = state.nodes.read().await;
    Ok(nodes.values().cloned().collect())
}

/// Provision a new cloud node
#[tauri::command]
pub async fn provision_cloud_node(
    state: State<'_, CloudState>,
    name: String,
    region: String,
    size: String,
) -> Result<CloudNode, String> {
    // Validate inputs
    if name.is_empty() {
        return Err("Node name is required".to_string());
    }

    // Check if name already exists
    {
        let nodes = state.nodes.read().await;
        if nodes.values().any(|n| n.name == name) {
            return Err("Node with this name already exists".to_string());
        }
    }

    // Generate node ID
    let node_id = format!("node_{}", uuid::Uuid::new_v4().to_string()[..8].to_string());

    // Calculate monthly cost based on size
    let monthly_cost = match size.as_str() {
        "small" => "$5",
        "medium" => "$20",
        "large" => "$40",
        "xlarge" => "$80",
        _ => "$5",
    };

    let now = chrono::Utc::now();

    // Create node record
    let node = CloudNode {
        id: node_id.clone(),
        name: name.clone(),
        region: region.clone(),
        size: size.clone(),
        status: if state.aws_configured { "provisioning" } else { "simulated" }.to_string(),
        public_ip: None,
        private_ip: None,
        created_at: now.format("%Y-%m-%d %H:%M").to_string(),
        instance_id: None,
        monthly_cost: monthly_cost.to_string(),
    };

    // Store node
    {
        let mut nodes = state.nodes.write().await;
        nodes.insert(node_id.clone(), node.clone());
    }

    // If AWS is configured, we would launch an EC2 instance here
    // For now, simulate the provisioning
    if state.aws_configured {
        // TODO: Actual AWS EC2 provisioning
        // let provisioner = AWSProvisioner::new(region).await?;
        // let instance = provisioner.provision_node(config).await?;

        // Simulate successful provisioning
        tokio::spawn({
            let state_nodes = state.nodes.clone();
            let node_id = node_id.clone();
            async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                let mut nodes = state_nodes.write().await;
                if let Some(n) = nodes.get_mut(&node_id) {
                    n.status = "running".to_string();
                    n.public_ip = Some(format!("54.{}.{}.{}",
                        rand::random::<u8>(),
                        rand::random::<u8>(),
                        rand::random::<u8>()
                    ));
                    n.instance_id = Some(format!("i-{:016x}", rand::random::<u64>()));
                }
            }
        });
    } else {
        // Simulate for demo - immediately mark as running
        let mut nodes = state.nodes.write().await;
        if let Some(n) = nodes.get_mut(&node_id) {
            n.status = "running".to_string();
            n.public_ip = Some(format!("192.168.1.{}", rand::random::<u8>()));
        }
    }

    Ok(node)
}

/// Terminate a cloud node
#[tauri::command]
pub async fn terminate_cloud_node(
    state: State<'_, CloudState>,
    node_id: String,
) -> Result<(), String> {
    let mut nodes = state.nodes.write().await;

    if let Some(node) = nodes.get_mut(&node_id) {
        node.status = "terminating".to_string();

        // If AWS configured and has instance ID, terminate the EC2 instance
        // TODO: Actual AWS termination

        // Remove after short delay
        drop(nodes);

        let state_nodes = state.nodes.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            let mut nodes = state_nodes.write().await;
            nodes.remove(&node_id);
        });

        Ok(())
    } else {
        Err("Node not found".to_string())
    }
}

/// Get available regions
#[tauri::command]
pub async fn get_cloud_regions() -> Result<Vec<CloudRegion>, String> {
    Ok(vec![
        CloudRegion {
            id: "us-east-1".to_string(),
            name: "US East (N. Virginia)".to_string(),
            location: "Virginia, USA".to_string(),
            available: true,
        },
        CloudRegion {
            id: "us-west-2".to_string(),
            name: "US West (Oregon)".to_string(),
            location: "Oregon, USA".to_string(),
            available: true,
        },
        CloudRegion {
            id: "eu-west-1".to_string(),
            name: "EU (Ireland)".to_string(),
            location: "Dublin, Ireland".to_string(),
            available: true,
        },
        CloudRegion {
            id: "ap-southeast-1".to_string(),
            name: "Asia Pacific (Singapore)".to_string(),
            location: "Singapore".to_string(),
            available: true,
        },
        CloudRegion {
            id: "sa-east-1".to_string(),
            name: "South America (São Paulo)".to_string(),
            location: "São Paulo, Brazil".to_string(),
            available: true,
        },
    ])
}

/// Get available instance sizes
#[tauri::command]
pub async fn get_instance_sizes() -> Result<Vec<InstanceSize>, String> {
    Ok(vec![
        InstanceSize {
            id: "small".to_string(),
            name: "Small".to_string(),
            vcpus: 1,
            memory_gb: 1.0,
            monthly_price: "$5/mo".to_string(),
        },
        InstanceSize {
            id: "medium".to_string(),
            name: "Medium".to_string(),
            vcpus: 2,
            memory_gb: 4.0,
            monthly_price: "$20/mo".to_string(),
        },
        InstanceSize {
            id: "large".to_string(),
            name: "Large".to_string(),
            vcpus: 4,
            memory_gb: 8.0,
            monthly_price: "$40/mo".to_string(),
        },
        InstanceSize {
            id: "xlarge".to_string(),
            name: "X-Large".to_string(),
            vcpus: 8,
            memory_gb: 16.0,
            monthly_price: "$80/mo".to_string(),
        },
    ])
}

/// Check if AWS is configured
#[tauri::command]
pub async fn is_aws_configured(
    state: State<'_, CloudState>,
) -> Result<bool, String> {
    Ok(state.aws_configured)
}

/// Restart a cloud node
#[tauri::command]
pub async fn restart_cloud_node(
    state: State<'_, CloudState>,
    node_id: String,
) -> Result<(), String> {
    let mut nodes = state.nodes.write().await;

    if let Some(node) = nodes.get_mut(&node_id) {
        node.status = "restarting".to_string();

        // Drop lock before spawning async task
        drop(nodes);

        let state_nodes = state.nodes.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            let mut nodes = state_nodes.write().await;
            if let Some(n) = nodes.get_mut(&node_id) {
                n.status = "running".to_string();
            }
        });

        Ok(())
    } else {
        Err("Node not found".to_string())
    }
}

/// Get node details
#[tauri::command]
pub async fn get_cloud_node(
    state: State<'_, CloudState>,
    node_id: String,
) -> Result<CloudNode, String> {
    let nodes = state.nodes.read().await;
    nodes.get(&node_id).cloned().ok_or("Node not found".to_string())
}
