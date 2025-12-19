use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub price: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub status: String,
}

#[tauri::command]
pub async fn get_available_plans() -> Result<Vec<Plan>, String> {
    Ok(vec![
        Plan { id: "free".into(), name: "Free".into(), price: 0.0 },
        Plan { id: "pro".into(), name: "Pro".into(), price: 10.0 },
        Plan { id: "team".into(), name: "Team".into(), price: 25.0 },
        Plan { id: "enterprise".into(), name: "Enterprise".into(), price: 100.0 },
    ])
}

#[tauri::command]
pub async fn get_cloud_regions() -> Result<Vec<Region>, String> {
    Ok(vec![
        Region { id: "us-east-1".into(), name: "US East (N. Virginia)".into() },
        Region { id: "us-west-2".into(), name: "US West (Oregon)".into() },
    ])
}

#[tauri::command]
pub async fn provision_cloud_node(name: String, region: String, size: String) -> Result<Node, String> {
    Ok(Node {
        id: "node1".into(),
        name,
        status: "provisioning".into(),
    })
}
