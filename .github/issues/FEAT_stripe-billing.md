---
title: "Feat: Implement Stripe Billing Integration"
labels:
  - enhancement
  - billing
  - jules
assignees: ["@jules"]
---

## Description
Connect `billing_commands.rs` to a real Stripe backend.

## Requirements
1. `create_checkout_session`: Call Stripe API to generate payment link.
2. `get_subscription_status`: Fetch real subscription state for the user.

## Technical Details
- Use `stripe-rust` or raw HTTP client.
- Securely manage API keys (env vars).
