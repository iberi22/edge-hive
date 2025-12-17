---
title: "Feat: Implement Real Server Control Commands"
labels:
  - enhancement
  - backend
  - jules
assignees: ["@jules"]
---

## Description
Replace stubs in `src-tauri/src/commands.rs` for `start_server` and `stop_server`.

## Requirements
1. `start_server(port)`: Spawn `edge-hive-node` process or thread.
2. `stop_server()`: Gracefully shutdown the node.
3. `get_node_status()`: Return real uptime, peer count, and status from the running instance.

## Technical Details
- Use `std::process::Command` or reference `edge_hive_core::Node`.
- Maintain state in `AppState` (Mutex).
