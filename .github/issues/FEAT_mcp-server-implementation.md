---
title: "[MCP] Implement Server MCP for Agent Dashboard Control"
labels:
  - enhancement
  - mcp
  - architecture
  - copilot
assignees: []
---

## Description

Create an MCP (Model Context Protocol) server that exposes Edge Hive admin operations as tools for AI agents.

## Goals

1. Allow agents to control the dashboard programmatically
2. Enable real-time UI updates when agents perform actions
3. Provide a standardized API for multi-agent orchestration

## Tasks

- [x] Create MCP server in `crates/edge-hive-mcp/` (Completed: 2025-01-15)
- [x] Expose tools: (Completed: 2025-01-15)
  - `admin_get_dashboard_stats` - Get current system metrics ✅
  - `admin_list_nodes` - Get all nodes and their status ✅
  - `admin_restart_node` - Restart a specific node ✅
  - `admin_update_node_status` - Update node status ✅
- [x] Implement TypeScript MCP client for frontend ✅
- [x] Integrate with Tauri backend via IPC ✅
- [x] Add shared state with RwLock for thread-safety ✅
- [x] Document MCP tool schemas in `docs/agent-docs/GUIDE_MCP_AGENT_CONTROL.md` ✅
- [ ] Implement WebSocket/SSE for real-time UI sync (Next phase)
- [ ] Add authentication/authorization for MCP access (Next phase)
- [ ] Add more tools: `admin_get_logs`, `admin_update_config`

## Completion Status

**✅ PHASE 1 COMPLETE** (2025-01-15)

### What was implemented

- JSON-RPC 2.0 protocol handler
- 4 admin tools with input validation
- Thread-safe shared state (Arc<RwLock<T>>)
- Tauri IPC integration
- TypeScript client with type safety
- Dashboard store synchronization
- Unit tests (4/4 passing)

### Commits

- `3c9b1af` - feat(mcp): implement MCP server for agent control
- `<latest>` - docs(agent): add MCP agent control guide

### Next Phase

WebSocket transport, authentication, and real-time sync.

## Architecture

```
Agent (Claude/GPT)
    ↓ MCP Protocol
MCP Server (Rust)
    ↓ IPC/WebSocket
Tauri App (UI + Backend)
    ↓ React to changes
Dashboard (Svelte)
```

## Reference

- MCP Specification: <https://spec.modelcontextprotocol.io>
- Existing store: `app/src/stores/dashboard.ts`
