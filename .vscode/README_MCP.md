# Edge Hive MCP Server Configuration

This directory contains the Model Context Protocol (MCP) server configuration for Edge Hive.

## 🚀 Quick Start

### 1. Generate OAuth2 Credentials

```bash
# Create OAuth2 client for VS Code
edge-hive auth client create --name "vscode-local" --scopes "mcp:read,mcp:call,mcp:resources"

# Save the credentials - they will only be shown once!
# Client ID: cli_abc123...
# Client Secret: secret_xyz789...
```

### 2. Configure Environment Variables

```bash
# Copy example file
cp .env.example .env

# Edit .env and add your credentials
EDGE_HIVE_CLIENT_ID=cli_abc123...
EDGE_HIVE_CLIENT_SECRET=secret_xyz789...
```

### 3. Start Edge Hive Server

#### HTTP Mode (for testing)

```bash
edge-hive serve --port 8080 --discovery
```

#### HTTPS Mode (with OAuth2)

```bash
edge-hive serve --port 8443 --https --hostname localhost --discovery
```

### 4. Configure VS Code

The `.vscode/mcp.json` file is already configured. Just reload VS Code:

1. Press `Ctrl+Shift+P`
2. Type "Reload Window"
3. Select "Developer: Reload Window"

## 📋 Available MCP Servers

| Server | Mode | URL | Auth | Description |
|--------|------|-----|------|-------------|
| `edge-hive-local-http` | stdio | - | None | Local MCP via stdin/stdout (no auth) |
| `edge-hive-local-https` | SSE | <https://localhost:8443/mcp/stream> | OAuth2 | Local MCP with HTTPS + OAuth2 |
| `edge-hive-docker-node1` | SSE | <https://172.20.0.10:8443/mcp/stream> | OAuth2 | Docker bootstrap node |
| `edge-hive-production` | SSE | <https://mcp.edge-hive.example.com/mcp/stream> | OAuth2 | Production server |

## 🛠️ MCP Tools

### `get_status`

Get node status information.

```json
{
  "name": "get_status",
  "arguments": {}
}
```

### `provision_node`

Provision a new edge node.

```json
{
  "name": "provision_node",
  "arguments": {
    "name": "my-new-node"
  }
}
```

## 📦 MCP Resources

| Resource | URI | Description |
|----------|-----|-------------|
| Last N logs | `edge-hive://logs/last` | Get recent log entries |
| Node status | `edge-hive://status` | Current node status JSON |

## 🔐 OAuth2 Flow

1. **Client Credentials Grant** (RFC 6749)
   - VS Code sends `client_id` + `client_secret` to `/mcp/auth/token`
   - Edge Hive validates credentials
   - Returns JWT access token (valid for 1 hour)

2. **Bearer Token Authentication**
   - VS Code sends `Authorization: Bearer <token>` with every request
   - Edge Hive validates JWT signature and expiration
   - Checks scopes for permission

3. **SSE Stream** (Server-Sent Events)
   - Long-lived connection for MCP notifications
   - Automatic reconnection with token refresh

## 🧪 Testing MCP Integration

### Test with cURL

```bash
# 1. Get access token
curl -X POST https://localhost:8443/mcp/auth/token \
  -H "Content-Type: application/json" \
  -d '{
    "grant_type": "client_credentials",
    "client_id": "cli_abc123...",
    "client_secret": "secret_xyz789..."
  }' -k

# 2. Call MCP tool
curl https://localhost:8443/mcp/tools/call \
  -H "Authorization: Bearer eyJhbGc..." \
  -H "Content-Type: application/json" \
  -d '{
    "method": "tools/call",
    "params": {
      "name": "get_status",
      "arguments": {}
    }
  }' -k

# 3. Get resource
curl https://localhost:8443/mcp/resources/edge-hive://status \
  -H "Authorization: Bearer eyJhbGc..." -k
```

### Test with VS Code

1. Open Command Palette (`Ctrl+Shift+P`)
2. Type "Copilot: Select MCP Server"
3. Choose `edge-hive-local-https`
4. Test with a prompt that uses MCP tools

## 🐳 Docker Multi-Node Setup

```bash
# Start 3-node cluster with HTTPS
docker-compose up --build

# Create OAuth2 client for Docker node 1
docker exec edge-hive-node1 edge-hive auth client create \
  --name "vscode-docker" \
  --scopes "mcp:read,mcp:call"

# Test from host machine
curl -X POST https://172.20.0.10:8443/mcp/auth/token \
  -H "Content-Type: application/json" \
  -d '{
    "grant_type": "client_credentials",
    "client_id": "cli_...",
    "client_secret": "..."
  }' -k
```

## 🔧 Troubleshooting

### "Connection refused" error

- Make sure Edge Hive server is running
- Check that port is correct (8080 for HTTP, 8443 for HTTPS)
- For HTTPS, use `-k` flag in curl to skip certificate verification

### "Invalid client credentials" error

- Verify `client_id` and `client_secret` in `.env`
- Regenerate credentials with `edge-hive auth client create`
- Check that client is not revoked: `edge-hive auth client list`

### "Token expired" error

- JWT tokens expire after 1 hour
- Request a new token with the same credentials
- VS Code should auto-refresh tokens

### Self-signed certificate warning

- Expected for local development
- Use `-k` flag in curl
- For production, use Let's Encrypt or a trusted CA

## 📚 References

- [Model Context Protocol Spec](https://spec.modelcontextprotocol.io/)
- [OAuth 2.0 RFC 6749](https://datatracker.ietf.org/doc/html/rfc6749)
- [VS Code MCP Extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode.vscode-copilot-mcp)
