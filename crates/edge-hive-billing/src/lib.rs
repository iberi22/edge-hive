//! Edge Hive Billing - Stripe integration for managed tier
//!
//! Handles subscriptions, payments, and usage-based billing.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;
use std::str::FromStr;

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
    stripe_client: stripe::Client,
    webhook_secret: String,
}

impl BillingService {
    /// Create a new billing service
    pub fn new(api_key: &str, webhook_secret: &str) -> Self {
        info!("💳 Billing service initialized");
        let stripe_client = stripe::Client::new(api_key);
        Self {
            stripe_client,
            webhook_secret: webhook_secret.to_string(),
        }
    }

    /// Create a Stripe checkout session
    pub async fn create_checkout_session(
        &self,
        user_id: &str,
        plan: Plan,
        return_url: &str,
    ) -> Result<CheckoutSession, BillingError> {
        info!("Creating checkout session for user {} (plan: {:?})", user_id, plan);

        let price_id = match plan {
            Plan::Pro => "prod_pro",   // Should be actual price IDs in production
            Plan::Team => "prod_team",
            _ => return Err(BillingError::StripeApi("Invalid plan for checkout".to_string())),
        };

        let mut params = stripe::CreateCheckoutSession::new();
        params.success_url = Some(return_url);
        params.cancel_url = Some(return_url);
        params.client_reference_id = Some(user_id);
        params.mode = Some(stripe::CheckoutSessionMode::Subscription);
        params.line_items = Some(vec![stripe::CreateCheckoutSessionLineItems {
            price: Some(price_id.to_string()),
            quantity: Some(1),
            ..Default::default()
        }]);

        let session = stripe::CheckoutSession::create(&self.stripe_client, params)
            .await
            .map_err(|e| BillingError::StripeApi(e.to_string()))?;

        Ok(CheckoutSession {
            id: session.id.to_string(),
            url: session.url.unwrap_or_default(),
        })
    }

    /// Get customer portal URL for managing subscription
    pub async fn get_portal_url(&self, _user_id: &str, return_url: &str) -> Result<String, BillingError> {
        info!("Getting portal URL for user {}", _user_id);

        // First find the customer by user_id metadata or similar
        // For simplicity, we assume we have the stripe_customer_id
        // In reality, we'd query our DB for the customer ID linked to this user_id

        let mut params = stripe::CreateBillingPortalSession::new(stripe::CustomerId::from_str("cus_test_123").unwrap());
        params.return_url = Some(return_url);

        let session = stripe::BillingPortalSession::create(&self.stripe_client, params)
            .await
            .map_err(|e| BillingError::StripeApi(e.to_string()))?;

        Ok(session.url)
    }

    /// Handle Stripe webhook
    pub async fn handle_webhook(
        &self,
        payload: &str,
        signature: &str,
    ) -> Result<WebhookEvent, BillingError> {
        info!("Handling webhook (sig: {}...)", &signature[..20.min(signature.len())]);

        let event = stripe::Webhook::construct_event(payload, signature, &self.webhook_secret)
            .map_err(|_| BillingError::InvalidSignature)?;

        Ok(WebhookEvent {
            event_type: event.type_.to_string(),
            data: serde_json::to_value(event.data.object)
                .map_err(|e| BillingError::StripeApi(e.to_string()))?,
        })
    }

    /// Check if user has active subscription
    pub async fn is_subscription_active(&self, user_id: &str) -> Result<bool, BillingError> {
        // In a real app, this would check our SurrealDB database
        // which is kept in sync via webhooks
        info!("Checking subscription for user {}", user_id);
        Ok(false)
    }

    /// Get current usage metrics
    pub async fn get_usage(&self, user_id: &str) -> Result<UsageMetrics, BillingError> {
        info!("Getting usage metrics for user {}", user_id);
        // This would also query our database/metrics engine
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
        assert!(!active);
    }
}
