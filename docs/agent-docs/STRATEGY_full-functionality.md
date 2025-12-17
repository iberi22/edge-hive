---
title: "Strategy: Full Backend Functionality Implementation"
status: "active"
owner: "@jules"
---

# Objective
Transition `edge-hive-admin` from a mocked UI to a fully functional control plane by replacing Rust stubs with real logic.

# Execution Plan

## 1. Core Services (High Priority)
Replace `TODO` stubs in `commands.rs` and `lib.rs` with actual calls to `edge-hive-core`.
- **Server Control**: `start_server` / `stop_server` spawning real processes.
- **Node Status**: `get_node_status` reading from `EdgeHiveNode` state.
- **Peer Discovery**: `get_peers` querying the DHT/Libp2p swarm.

## 2. Cloud Integration
Connect `provision_cloud_node` to Terraform/AWS SDK.
- **Provisioning**: Trigger Terraform scripts via `edge-hive-infra`.
- **Status Sync**: Poll cloud provider API for node health.

## 3. Billing & Identity
Connect `billing_commands.rs` to Stripe and `auth_commands.rs` to real JWT/DB.
- **Stripe**: Create actual checkout sessions using `async-stripe`.
- **Auth**: Validate credentials against `surrealdb`.

# Task Assignment (Jules)
Jules will be assigned to:
1. Create the `FEAT` issues below.
2. Implement the Rust logic for each module.
