---
title: "[MVP] E2E Testing & Integration Verification"
labels:
  - testing
  - jules
  - MVP
assignees: []
---

## Description

Complete end-to-end testing of all implemented features before release.

## Test Categories

### 1. Auth System
- [ ] Register new user
- [ ] Login with valid credentials
- [ ] Login with invalid credentials (should fail)
- [ ] Get current user
- [ ] Logout
- [ ] Token validation

### 2. Storage CRUD
- [ ] List buckets (empty initially)
- [ ] Create bucket
- [ ] Upload file (base64)
- [ ] List files in bucket
- [ ] Download file
- [ ] Delete file
- [ ] Delete bucket

### 3. Settings
- [ ] Create API key
- [ ] List API keys
- [ ] Revoke API key
- [ ] Create backup
- [ ] List backups
- [ ] Restore backup
- [ ] Save SMTP config
- [ ] Get SMTP config

### 4. Billing
- [ ] Get subscription status
- [ ] Get available plans
- [ ] Check Stripe configured (false without env var)
- [ ] Get usage metrics

### 5. Cloud
- [ ] Get regions
- [ ] Get instance sizes
- [ ] Provision node (simulated)
- [ ] List nodes
- [ ] Terminate node
- [ ] Check AWS configured

### 6. Server Control
- [ ] Start server
- [ ] Get node status
- [ ] Stop server

### 7. Cache & Tunnel
- [ ] Get cache stats
- [ ] Clear cache
- [ ] Start tunnel
- [ ] Get tunnel status
- [ ] Stop tunnel

## Test Script

```typescript
// tests/e2e.test.ts
describe('Edge Hive E2E', () => {
  it('Auth flow works', async () => {
    // Register
    const user = await api.signUp('test@example.com', 'password123');
    expect(user.email).toBe('test@example.com');

    // Login
    const auth = await api.signIn('test@example.com', 'password123');
    expect(auth.token).toBeDefined();

    // Current user
    const current = await api.getCurrentUser();
    expect(current.email).toBe('test@example.com');
  });

  it('Storage flow works', async () => {
    // Create bucket
    const bucket = await api.createBucket('test-bucket', false);
    expect(bucket.name).toContain('test-bucket');

    // Upload file
    const file = await api.uploadFile(bucket.id, 'test.txt', btoa('Hello World'));
    expect(file.name).toBe('test.txt');

    // Download file
    const content = await api.downloadFile(bucket.id, file.id);
    expect(atob(content)).toBe('Hello World');

    // Cleanup
    await api.deleteFile(bucket.id, file.id);
    await api.deleteBucket(bucket.id);
  });
});
```

## Acceptance Criteria

- [ ] All test categories pass
- [ ] No console errors during testing
- [ ] UI displays correct data
- [ ] No memory leaks

## Estimated Effort
4-6 hours
