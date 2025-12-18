use tauri::State;
use edge_hive_billing::{BillingService, Plan, Subscription, SubscriptionStatus, UsageMetrics};
use serde::{Deserialize, Serialize};

pub struct BillingState {
    pub service: BillingService,
}

impl BillingState {
    pub fn new() -> Self {
        Self {
            service: BillingService::new("key", "secret"),
        }
    }
}

#[tauri::command]
pub async fn get_subscription_status(
    _state: State<'_, BillingState>
) -> Result<Subscription, String> {
    // Stub data
    Ok(Subscription {
        id: "sub_123".to_string(),
        user_id: "user_123".to_string(),
        plan: Plan::Pro,
        status: SubscriptionStatus::Active,
        current_period_end: chrono::Utc::now() + chrono::Duration::days(30),
        stripe_customer_id: Some("cus_123".to_string()),
        stripe_subscription_id: Some("sub_123".to_string()),
    })
}

#[tauri::command]
pub async fn get_usage_metrics(
    state: State<'_, BillingState>
) -> Result<UsageMetrics, String> {
    state.service.get_usage("user_123").await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_checkout_session(
    state: State<'_, BillingState>,
    plan: String
) -> Result<String, String> {
    let plan_enum = match plan.as_str() {
        "Pro" => Plan::Pro,
        "Team" => Plan::Team,
        "Enterprise" => Plan::Enterprise,
        _ => Plan::Free,
    };

    let session = state.service.create_checkout_session("user_123", plan_enum, "http://localhost:1420/billing")
        .await
        .map_err(|e| e.to_string())?;

    Ok(session.url)
}
