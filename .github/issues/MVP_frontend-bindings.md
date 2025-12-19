---
title: "[MVP] Frontend Bindings Integration - Complete tauriClient.ts"
labels:
  - frontend
  - jules
  - MVP
assignees: []
---

## Description

Ensure all new backend commands have proper frontend bindings in `tauriClient.ts`.

## Missing Bindings to Add

### Billing (6 new methods)
```typescript
getAvailablePlans: async () => invoke('get_available_plans'),
getBillingPortalUrl: async () => invoke('get_billing_portal_url'),
updateSubscription: async (plan: string) => invoke('update_subscription', { plan }),
isStripeConfigured: async () => invoke('is_stripe_configured'),
cancelSubscription: async () => invoke('cancel_subscription'),
recordUsage: async (data) => invoke('record_usage', data),
```

### Cloud (6 new methods)
```typescript
getCloudRegions: async () => invoke('get_cloud_regions'),
getInstanceSizes: async () => invoke('get_instance_sizes'),
terminateCloudNode: async (nodeId: string) => invoke('terminate_cloud_node', { nodeId }),
restartCloudNode: async (nodeId: string) => invoke('restart_cloud_node', { nodeId }),
getCloudNode: async (nodeId: string) => invoke('get_cloud_node', { nodeId }),
isAwsConfigured: async () => invoke('is_aws_configured'),
```

### Settings (already added, verify)
- [ ] Verify all settings methods work

## Tasks

- [ ] Add missing billing bindings
- [ ] Add missing cloud bindings
- [ ] Update TypeScript types
- [ ] Fix any lint errors
- [ ] Test all bindings work

## Acceptance Criteria

- [ ] All backend commands have frontend bindings
- [ ] TypeScript compiles without errors
- [ ] API calls work from UI

## Estimated Effort
2-3 hours
