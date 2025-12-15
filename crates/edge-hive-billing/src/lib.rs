//! Edge Hive Billing - Stripe integration for managed tier
//!
//! Handles subscriptions, payments, and usage-based billing.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;

/// Errors that can occur during billing operations
#[derive(Debug, Error)]
pub enum BillingError {
    #[error("Stripe API error: {0}")]
    StripeApi(String),

    #[error("Invalid webhook signature")]
    InvalidSignature,

    #[error("Subscription not found: {0}")]
    SubscriptionNotFound(String),

    #[error("Payment failed: {0}")]
    PaymentFailed(String),
}

/// Subscription plans
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Plan {
    /// Free self-hosted tier
    Free,
    /// $25/mo - 1 managed node
    Pro,
    /// $100/mo - 5 managed nodes
    Team,
    /// Custom pricing
    Enterprise,
}

impl Plan {
    pub fn price_cents(&self) -> u64 {
        match self {
            Plan::Free => 0,
            Plan::Pro => 2500,      // $25.00
            Plan::Team => 10000,    // $100.00
            Plan::Enterprise => 0,   // Custom
        }
    }

    pub fn max_nodes(&self) -> u32 {
        match self {
            Plan::Free => 0,
            Plan::Pro => 1,
            Plan::Team => 5,
            Plan::Enterprise => u32::MAX,
        }
    }

    pub fn storage_gb(&self) -> u32 {
        match self {
            Plan::Free => 0,
            Plan::Pro => 10,
            Plan::Team => 100,
            Plan::Enterprise => u32::MAX,
        }
    }

    pub fn egress_gb(&self) -> u32 {
        match self {
            Plan::Free => 0,
            Plan::Pro => 50,
            Plan::Team => 500,
            Plan::Enterprise => u32::MAX,
        }
    }
}

/// User subscription status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub user_id: String,
    pub plan: Plan,
    pub status: SubscriptionStatus,
    pub current_period_end: chrono::DateTime<chrono::Utc>,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SubscriptionStatus {
    Active,
    PastDue,
    Canceled,
    Trialing,
}

/// Usage metrics for billing
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageMetrics {
    pub storage_bytes: u64,
    pub egress_bytes: u64,
    pub api_requests: u64,
    pub active_nodes: u32,
}

/// Billing service (stub implementation)
pub struct BillingService {
    // stripe_client: stripe::Client,
    // webhook_secret: String,
}

impl BillingService {
    /// Create a new billing service
    pub fn new(_api_key: &str, _webhook_secret: &str) -> Self {
        info!("💳 Billing service initialized");
        Self {}
    }

    /// Create a Stripe checkout session
    pub async fn create_checkout_session(
        &self,
        user_id: &str,
        plan: Plan,
        return_url: &str,
    ) -> Result<CheckoutSession, BillingError> {
        info!("Creating checkout session for user {} (plan: {:?})", user_id, plan);

        // TODO: Implement Stripe checkout
        // let session = stripe::CheckoutSession::create(&self.stripe_client, params).await?;

        Ok(CheckoutSession {
            id: format!("cs_test_{}", uuid_v4()),
            url: format!("https://checkout.stripe.com/pay/cs_test_xxx?success_url={}", return_url),
        })
    }

    /// Get customer portal URL for managing subscription
    pub async fn get_portal_url(&self, user_id: &str) -> Result<String, BillingError> {
        info!("Getting portal URL for user {}", user_id);

        // TODO: Implement Stripe portal session
        Ok(format!("https://billing.stripe.com/p/session/test_{}", user_id))
    }

    /// Handle Stripe webhook
    pub async fn handle_webhook(
        &self,
        payload: &str,
        signature: &str,
    ) -> Result<WebhookEvent, BillingError> {
        info!("Handling webhook (sig: {}...)", &signature[..20.min(signature.len())]);

        // TODO: Verify signature and parse event
        // let event = stripe::Webhook::construct_event(payload, signature, &self.webhook_secret)?;

        // Parse the JSON payload
        let event: serde_json::Value = serde_json::from_str(payload)
            .map_err(|e| BillingError::StripeApi(e.to_string()))?;

        let event_type = event["type"].as_str().unwrap_or("unknown");

        Ok(WebhookEvent {
            event_type: event_type.to_string(),
            data: event,
        })
    }

    /// Check if user has active subscription
    pub async fn is_subscription_active(&self, user_id: &str) -> Result<bool, BillingError> {
        // TODO: Check database or Stripe
        info!("Checking subscription for user {}", user_id);
        Ok(false) // Default: no active subscription
    }

    /// Get current usage metrics
    pub async fn get_usage(&self, user_id: &str) -> Result<UsageMetrics, BillingError> {
        info!("Getting usage metrics for user {}", user_id);
        Ok(UsageMetrics::default())
    }
}

/// Checkout session response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutSession {
    pub id: String,
    pub url: String,
}

/// Parsed webhook event
#[derive(Debug, Clone)]
pub struct WebhookEvent {
    pub event_type: String,
    pub data: serde_json::Value,
}

/// Simple UUID v4 generator (stub)
fn uuid_v4() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:016x}", nanos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_pricing() {
        assert_eq!(Plan::Pro.price_cents(), 2500);
        assert_eq!(Plan::Team.price_cents(), 10000);
        assert_eq!(Plan::Pro.max_nodes(), 1);
        assert_eq!(Plan::Team.max_nodes(), 5);
    }

    #[tokio::test]
    async fn test_billing_service_creation() {
        let service = BillingService::new("sk_test_xxx", "whsec_xxx");
        let active = service.is_subscription_active("user_123").await.unwrap();
        assert!(!active);
    }
}
