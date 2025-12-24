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

/// Billing service
pub struct BillingService {
    // stripe_client removed - project is now free-to-use
}

impl BillingService {
    /// Create a new billing service
    pub fn new(_api_key: &str, _webhook_secret: &str) -> Self {
        info!("💳 Billing service initialized (Free Tier Mode)");
        Self {}
    }

    /// Create a Stripe checkout session (Stubbed - Returns Free Tier notice)
    pub async fn create_checkout_session(
        &self,
        _user_id: &str,
        _plan: Plan,
        return_url: &str,
    ) -> Result<CheckoutSession, BillingError> {
        info!("Checkout requested (Free Tier active - redirecting to return URL)");

        Ok(CheckoutSession {
            id: "free_tier_session".to_string(),
            url: return_url.to_string(),
        })
    }

    /// Get customer portal URL for managing subscription (Stubbed)
    pub async fn get_portal_url(&self, _user_id: &str, return_url: &str) -> Result<String, BillingError> {
        info!("Portal requested (Free Tier active - redirecting to return URL)");
        Ok(return_url.to_string())
    }

    /// Handle Stripe webhook (Stubbed)
    pub async fn handle_webhook(
        &self,
        _payload: &str,
        _signature: &str,
    ) -> Result<WebhookEvent, BillingError> {
        info!("Stripe webhook received (Ignored - Free Tier active)");

        Ok(WebhookEvent {
            event_type: "ignored_event".to_string(),
            data: serde_json::Value::Null,
        })
    }

    /// Check if user has active subscription (Always true for Free Tier)
    pub async fn is_subscription_active(&self, _user_id: &str) -> Result<bool, BillingError> {
        Ok(true)
    }

    /// Get current usage metrics
    pub async fn get_usage(&self, _user_id: &str) -> Result<UsageMetrics, BillingError> {
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
        assert!(active); // Now always true
    }
}
