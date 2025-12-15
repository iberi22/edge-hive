---
title: "[CLOUD] Stripe Billing Integration"
labels:
  - cloud
  - billing
  - stripe
assignees: []
---

## User Story

**As a** user
**I want** to manage my subscription and billing
**So that** I can pay for managed cloud nodes

## Technical Specs

### Dependencies

```toml
[dependencies]
stripe-rust = "0.26"
jsonwebtoken = "9"
```

### Billing Service

```rust
pub struct BillingService {
    stripe: stripe::Client,
    webhook_secret: String,
}

impl BillingService {
    /// Create a checkout session for new subscription
    pub async fn create_checkout(&self, user_id: &str, plan: Plan) -> Result<CheckoutSession>;

    /// Handle Stripe webhook events
    pub async fn handle_webhook(&self, payload: &str, signature: &str) -> Result<()>;

    /// Get Stripe customer portal URL
    pub async fn get_portal_url(&self, user_id: &str) -> Result<String>;

    /// Check subscription status
    pub async fn is_active(&self, user_id: &str) -> Result<bool>;
}
```

### Stripe Products

| Product ID | Name | Price |
|------------|------|-------|
| `prod_pro` | Edge Hive Pro | $25/mo |
| `prod_team` | Edge Hive Team | $100/mo |
| `prod_storage` | Storage Add-on | $0.10/GB |
| `prod_egress` | Egress Add-on | $0.05/GB |

### Webhook Events

| Event | Action |
|-------|--------|
| `checkout.session.completed` | Provision node |
| `customer.subscription.deleted` | Terminate node |
| `customer.subscription.updated` | Update node config |
| `invoice.payment_failed` | Suspend node |

## Acceptance Criteria

- [ ] Stripe checkout flow works
- [ ] Webhook endpoint secured and verified
- [ ] Customer portal link works
- [ ] Subscription status synced with app
- [ ] Failed payments handled gracefully

## Branch

`feat/stripe-billing`
