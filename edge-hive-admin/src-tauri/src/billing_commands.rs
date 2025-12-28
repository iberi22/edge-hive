use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub plan: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub price: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageMetric {
    pub metric: String,
    pub value: f32,
    pub limit: f32,
}

pub struct BillingState {
}

impl BillingState {
    pub fn new() -> Self {
        Self {}
    }
}

#[tauri::command]
pub async fn get_subscription_status() -> Result<Subscription, String> {
    Ok(Subscription {
        id: "sub_123".into(),
        plan: "Pro".into(),
        status: "active".into(),
    })
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
pub async fn create_checkout_session(_plan: String) -> Result<String, String> {
    Ok("https://checkout.stripe.com/...".to_string())
}

#[tauri::command]
pub async fn get_usage_metrics() -> Result<Vec<UsageMetric>, String> {
    Ok(vec![
        UsageMetric { metric: "bandwidth".into(), value: 1.2, limit: 10.0 },
        UsageMetric { metric: "storage".into(), value: 0.5, limit: 5.0 },
        UsageMetric { metric: "compute".into(), value: 120.0, limit: 1000.0 },
    ])
}
