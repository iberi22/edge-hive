---
title: "MCP Server - Agent Control Guide"
type: GUIDE
id: "guide-mcp-agent-control"
created: 2025-01-15
updated: 2025-01-15
agent: copilot
model: claude-sonnet-4
requested_by: user
summary: |
  Complete guide for AI agents to control Edge Hive admin dashboard
  via the Model Context Protocol (MCP).
keywords: [mcp, agent-control, json-rpc, api, automation]
tags: ["#mcp", "#ai-agents", "#automation", "#api"]
topics: [mcp, ai-agents, dashboard]
related_issues: []
project: edge-hive
module: mcp
language: rust
priority: high
status: implemented
confidence: 0.95
token_estimate: 1200
complexity: moderate
---

# MCP Server - AI Agent Control Guide

## 📋 Overview

The **Edge Hive MCP Server** enables AI agents (Claude, GPT, Gemini, etc.) to control the admin dashboard programmatically using the **Model Context Protocol (MCP)**.

**Protocol:** JSON-RPC 2.0  
**Transport:** Tauri IPC (WebSocket/HTTP planned)  
**Location:** `crates/edge-hive-mcp/`

---

## 🛠️ Available Tools

### 1. `admin_get_dashboard_stats`

**Description:** Get current system metrics.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {},
  "required": []
}
```

**Example Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "admin_get_dashboard_stats",
    "arguments": {}
  }
}
```

**Example Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "cpu_usage": 45.5,
    "total_memory": 16000000,
    "used_memory": 8000000,
    "active_nodes": 3,
    "total_tunnels": 5
  }
}
```

---

### 2. `admin_list_nodes`

**Description:** List all nodes with optional status filter.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "status_filter": {
      "type": "string",
      "enum": ["active", "idle", "error", "all"]
    }
  },
  "required": []
}
```

**Example Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "admin_list_nodes",
    "arguments": {
      "status_filter": "active"
    }
  }
}
```

**Example Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": [
    {
      "id": "node-1",
      "name": "Server 1",
      "status": "active",
      "cpu": 30.0,
      "memory": 4000000,
      "ip": "192.168.1.100"
    }
  ]
}
```

---

### 3. `admin_restart_node`

**Description:** Restart a specific node.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "node_id": {
      "type": "string",
      "description": "The unique identifier of the node"
    }
  },
  "required": ["node_id"]
}
```

**Example Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "admin_restart_node",
    "arguments": {
      "node_id": "node-1"
    }
  }
}
```

**Example Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "success": true,
    "node_id": "node-1",
    "message": "Node node-1 restart initiated"
  }
}
```

---

### 4. `admin_update_node_status`

**Description:** Update node status (e.g., maintenance mode).

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "node_id": {
      "type": "string"
    },
    "status": {
      "type": "string",
      "enum": ["active", "idle", "error", "maintenance"]
    }
  },
  "required": ["node_id", "status"]
}
```

**Example Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "tools/call",
  "params": {
    "name": "admin_update_node_status",
    "arguments": {
      "node_id": "node-1",
      "status": "maintenance"
    }
  }
}
```

**Example Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "result": {
    "success": true,
    "node_id": "node-1",
    "status": "maintenance",
    "message": "Node node-1 status updated to maintenance"
  }
}
```

---

## 🔌 Integration Examples

### Claude Desktop (MCP Client)

Add to `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) or `%APPDATA%\Claude\claude_desktop_config.json` (Windows):

```json
{
  "mcpServers": {
    "edge-hive-admin": {
      "command": "tauri",
      "args": ["mcp"],
      "env": {}
    }
  }
}
```

### VS Code Copilot

1. Open admin dashboard in browser
2. Use MCP client extension
3. Call tools via JSON-RPC

### Programmatic Access (TypeScript)

```typescript
import { mcpClient } from '$lib/mcp-client';

// Get dashboard stats
const stats = await mcpClient.getDashboardStats();
console.log(`CPU: ${stats.cpu_usage}%`);

// List active nodes
const nodes = await mcpClient.listNodes('active');
console.log(`Active nodes: ${nodes.length}`);

// Restart node
const result = await mcpClient.restartNode('node-1');
console.log(result.message);
```

---

## 🧪 Testing

Run unit tests:

```bash
cd crates/edge-hive-mcp
cargo test
```

Expected output:
```
running 4 tests
test tests::test_list_tools ... ok
test tests::test_update_node_status ... ok
test tests::test_get_dashboard_stats ... ok
test tests::test_list_and_filter_nodes ... ok

test result: ok. 4 passed
```

---

## 🚀 Next Steps

1. **WebSocket Transport:** Enable remote MCP access via WebSocket server
2. **Authentication:** Add JWT/OAuth for secure agent access
3. **Real-time Sync:** Implement SSE/WebSocket for UI updates when agents modify state
4. **More Tools:** Extend with deployment, configuration, and monitoring tools

---

## 📚 References

- [Model Context Protocol Spec](https://spec.modelcontextprotocol.io/)
- [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)
- [Tauri IPC Documentation](https://tauri.app/v2/guides/ipc/)
