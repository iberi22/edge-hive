---
title: "[MCP] Implement Agent-Controllable Admin Interface API"
labels:
  - enhancement
  - mcp
  - jules
  - architecture
assignees: []
---

## Description

Ensure the Admin Interface is fully controllable by AI Agents via the Model Context Protocol (MCP). The UI should be a reflection of an API that agents can also consume.

## Requirements

- [ ] **API-First Design:** The UI must consume an internal API that exposes all administrative actions (restart node, view logs, update config).
- [ ] **MCP Tool Exposure:** Create MCP tools that wrap these API endpoints.
  - `admin_get_stats`: Retrieve dashboard metrics.
  - `admin_list_nodes`: Get node status.
  - `admin_restart_service`: Restart specific services.
- [ ] **State Reflection:** Changes made by agents via MCP should update the UI in real-time (via WebSockets/SSE).

## Goal

"Quiero que tengamos una interfaz completamente que pueda ser tambien administrada por los agentes via MCP"
