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
pub async fn create_checkout_session(plan: String) -> Result<String, String> {
    Ok("https://checkout.stripe.com/...".to_string())
}
