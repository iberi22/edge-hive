---
title: "Feat: Implement Cloud Provisioning Logic"
labels:
  - enhancement
  - cloud
  - jules
assignees: ["@jules"]
---

## Description
Implement `provision_cloud_node` in `commands.rs` to actually deploy resources.

## Requirements
1. Accept `region` and `size`.
2. Trigger infrastructure deployment (Terraform or direct API).
3. Return `CloudNode` struct with real ID and initial status.

## Technical Details
- Integrate with `edge-hive-infra` crate if available.
- Handle long-running operations (async/await).
