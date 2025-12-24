//! Edge Hive MCP Server
//!
//! Model Context Protocol server that exposes Edge Hive admin operations as tools.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use edge_hive_auth::{middleware::AuthenticatedUser, TokenValidator};

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MCPResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<MCPError>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl MCPError {
    pub fn insufficient_permissions() -> Self {
        Self {
            code: -32000,
            message: "Insufficient permissions".to_string(),
            data: None,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

// ===== Dashboard Data Types =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub cpu_usage: f32,
    pub total_memory: u64,
    pub used_memory: u64,
    pub active_nodes: usize,
    pub total_tunnels: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub status: String,
    pub cpu: f32,
    pub memory: u64,
    pub ip: String,
}

// ===== MCP Server =====

pub struct MCPServer {
    tools: HashMap<String, Tool>,
    stats: Arc<RwLock<DashboardStats>>,
    nodes: Arc<RwLock<Vec<Node>>>,
}

impl MCPServer {
    pub fn new() -> Self {
        let mut tools = HashMap::new();

        // Define admin_get_dashboard_stats tool
        tools.insert(
            "admin_get_dashboard_stats".to_string(),
            Tool {
                name: "admin_get_dashboard_stats".to_string(),
                description: "Get current system metrics (CPU, RAM, Storage, Network)".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
        );

        // Define admin_list_nodes tool
        tools.insert(
            "admin_list_nodes".to_string(),
            Tool {
                name: "admin_list_nodes".to_string(),
                description: "List all nodes in the Edge Hive cluster with their status".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "status_filter": {
                            "type": "string",
                            "description": "Filter nodes by status",
                            "enum": ["active", "idle", "error", "all"]
                        }
                    },
                    "required": []
                }),
            },
        );

        // Define admin_restart_node tool
        tools.insert(
            "admin_restart_node".to_string(),
            Tool {
                name: "admin_restart_node".to_string(),
                description: "Restart a specific node by ID".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "node_id": {
                            "type": "string",
                            "description": "The ID of the node to restart"
                        }
                    },
                    "required": ["node_id"]
                }),
            },
        );

        // Define admin_update_node_status tool
        tools.insert(
            "admin_update_node_status".to_string(),
            Tool {
                name: "admin_update_node_status".to_string(),
                description: "Update the status of a node (for testing/simulation)".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "node_id": {
                            "type": "string",
                            "description": "The ID of the node"
                        },
                        "status": {
                            "type": "string",
                            "enum": ["active", "idle", "error", "maintenance"],
                            "description": "The new status"
                        }
                    },
                    "required": ["node_id", "status"]
                }),
            },
        );

        // Initialize with default stats
        let stats = Arc::new(RwLock::new(DashboardStats {
            cpu_usage: 0.0,
            total_memory: 0,
            used_memory: 0,
            active_nodes: 0,
            total_tunnels: 0,
        }));

        let nodes = Arc::new(RwLock::new(Vec::new()));

        Self { tools, stats, nodes }
    }

    /// Update system stats (called periodically from Tauri backend)
    pub async fn update_stats(&self, stats: DashboardStats) {
        let mut current_stats = self.stats.write().await;
        *current_stats = stats;
    }

    /// Update nodes list (called periodically from Tauri backend)
    pub async fn update_nodes(&self, nodes: Vec<Node>) {
        let mut current_nodes = self.nodes.write().await;
        *current_nodes = nodes;
    }

    pub async fn handle_request(&self, request: MCPRequest) -> MCPResponse {
        match request.method.as_str() {
            "tools/list" => self.list_tools(request.id),
            "tools/call" => self.call_tool(request.id, request.params).await,
            _ => MCPResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(MCPError {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            },
        }
    }

    fn list_tools(&self, id: Option<Value>) -> MCPResponse {
        let tools: Vec<&Tool> = self.tools.values().collect();
        MCPResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!(tools)),
            error: None,
        }
    }

    async fn call_tool(&self, id: Option<Value>, params: Option<Value>) -> MCPResponse {
        let params = match params {
            Some(p) => p,
            None => {
                return MCPResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(MCPError {
                        code: -32602,
                        message: "Invalid params".to_string(),
                        data: None,
                    }),
                }
            }
        };

        let tool_name = match params.get("name").and_then(|v| v.as_str()) {
            Some(name) => name,
            None => {
                return MCPResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(MCPError {
                        code: -32602,
                        message: "Missing tool name".to_string(),
                        data: None,
                    }),
                }
            }
        };

        let arguments = params.get("arguments").cloned();

        match self.execute_tool(tool_name, arguments).await {
            Ok(result) => MCPResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(result),
                error: None,
            },
            Err(error) => MCPResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(error),
            },
        }
    }

    async fn execute_tool(&self, tool_name: &str, arguments: Option<Value>) -> Result<Value, MCPError> {
        match tool_name {
            "admin_get_dashboard_stats" => {
                let stats = self.stats.read().await.clone();
                Ok(json!(stats))
            }
            "admin_list_nodes" => {
                let nodes = self.nodes.read().await.clone();

                // Filter by status if provided
                let filtered_nodes = if let Some(args) = arguments {
                    if let Some(status_filter) = args.get("status_filter").and_then(|v| v.as_str()) {
                        if status_filter != "all" {
                            nodes.into_iter()
                                .filter(|n| n.status == status_filter)
                                .collect()
                        } else {
                            nodes
                        }
                    } else {
                        nodes
                    }
                } else {
                    nodes
                };

                Ok(json!(filtered_nodes))
            }
            "admin_restart_node" => {
                let node_id = arguments
                    .as_ref()
                    .and_then(|args| args.get("node_id"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| MCPError {
                        code: -32602,
                        message: "Missing node_id parameter".to_string(),
                        data: None,
                    })?;

                // In real implementation, this would trigger a Tauri command
                Ok(json!({
                    "success": true,
                    "node_id": node_id,
                    "message": format!("Node {} restart initiated", node_id)
                }))
            }
            "admin_update_node_status" => {
                let node_id = arguments
                    .as_ref()
                    .and_then(|args| args.get("node_id"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| MCPError {
                        code: -32602,
                        message: "Missing node_id parameter".to_string(),
                        data: None,
                    })?;

                let status = arguments
                    .as_ref()
                    .and_then(|args| args.get("status"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| MCPError {
                        code: -32602,
                        message: "Missing status parameter".to_string(),
                        data: None,
                    })?;

                // Update node status in memory
                let mut nodes = self.nodes.write().await;
                if let Some(node) = nodes.iter_mut().find(|n| n.id == node_id) {
                    node.status = status.to_string();
                    Ok(json!({
                        "success": true,
                        "node_id": node_id,
                        "status": status,
                        "message": format!("Node {} status updated to {}", node_id, status)
                    }))
                } else {
                    Err(MCPError {
                        code: -32000,
                        message: format!("Node {} not found", node_id),
                        data: None,
                    })
                }
            }
            _ => Err(MCPError {
                code: -32601,
                message: format!("Unknown tool: {}", tool_name),
                data: None,
            }),
        }
    }
}

// ===== Authenticated MCP Server =====

pub struct AuthenticatedMCPServer {
    inner: MCPServer,
    validator: Arc<TokenValidator>,
}

impl AuthenticatedMCPServer {
    pub fn new(validator: TokenValidator) -> Self {
        Self {
            inner: MCPServer::new(),
            validator: Arc::new(validator),
        }
    }

    pub async fn handle_request(&self, request: MCPRequest, user: AuthenticatedUser) -> MCPResponse {
        let required_scopes = match request.method.as_str() {
            "tools/list" => vec!["mcp:read".to_string()],
            "tools/call" => vec!["mcp:call".to_string()],
            _ => return MCPResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(MCPError {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            },
        };

        if !user.claims.has_all_scopes(&required_scopes) {
            return MCPResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(MCPError::insufficient_permissions()),
            };
        }

        self.inner.handle_request(request).await
    }

    /// Update system stats (called periodically from Tauri backend)
    pub async fn update_stats(&self, stats: DashboardStats) {
        self.inner.update_stats(stats).await;
    }

    /// Update nodes list (called periodically from Tauri backend)
    pub async fn update_nodes(&self, nodes: Vec<Node>) {
        self.inner.update_nodes(nodes).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_hive_auth::JwtClaims;
    use chrono::Utc;

    #[tokio::test]
    async fn test_list_tools() {
        let server = MCPServer::new();
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(1)),
            method: "tools/list".to_string(),
            params: None,
        };

        let response = server.handle_request(request).await;
        assert!(response.error.is_none());
        assert!(response.result.is_some());
    }

    #[tokio::test]
    async fn test_get_dashboard_stats() {
        let server = MCPServer::new();

        // Update stats first
        server.update_stats(DashboardStats {
            cpu_usage: 45.5,
            total_memory: 16_000_000,
            used_memory: 8_000_000,
            active_nodes: 3,
            total_tunnels: 5,
        }).await;

        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(2)),
            method: "tools/call".to_string(),
            params: Some(json!({
                "name": "admin_get_dashboard_stats",
                "arguments": {}
            })),
        };

        let response = server.handle_request(request).await;
        assert!(response.error.is_none());
        let result = response.result.unwrap();
        assert_eq!(result["cpu_usage"], 45.5);
        assert_eq!(result["active_nodes"], 3);
    }

    #[tokio::test]
    async fn test_list_and_filter_nodes() {
        let server = MCPServer::new();

        // Add test nodes
        server.update_nodes(vec![
            Node {
                id: "node-1".to_string(),
                name: "Server 1".to_string(),
                status: "active".to_string(),
                cpu: 30.0,
                memory: 4_000_000,
                ip: "192.168.1.100".to_string(),
            },
            Node {
                id: "node-2".to_string(),
                name: "Server 2".to_string(),
                status: "idle".to_string(),
                cpu: 5.0,
                memory: 2_000_000,
                ip: "192.168.1.101".to_string(),
            },
        ]).await;

        // Test without filter
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(3)),
            method: "tools/call".to_string(),
            params: Some(json!({
                "name": "admin_list_nodes",
                "arguments": {}
            })),
        };

        let response = server.handle_request(request).await;
        assert!(response.error.is_none());
        let result = response.result.unwrap();
        assert_eq!(result.as_array().unwrap().len(), 2);

        // Test with filter
        let request_filtered = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(4)),
            method: "tools/call".to_string(),
            params: Some(json!({
                "name": "admin_list_nodes",
                "arguments": {
                    "status_filter": "active"
                }
            })),
        };

        let response_filtered = server.handle_request(request_filtered).await;
        assert!(response_filtered.error.is_none());
        let result_filtered = response_filtered.result.unwrap();
        assert_eq!(result_filtered.as_array().unwrap().len(), 1);
        assert_eq!(result_filtered[0]["id"], "node-1");
    }

    #[tokio::test]
    async fn test_update_node_status() {
        let server = MCPServer::new();

        // Add test node
        server.update_nodes(vec![
            Node {
                id: "node-1".to_string(),
                name: "Server 1".to_string(),
                status: "active".to_string(),
                cpu: 30.0,
                memory: 4_000_000,
                ip: "192.168.1.100".to_string(),
            },
        ]).await;

        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(5)),
            method: "tools/call".to_string(),
            params: Some(json!({
                "name": "admin_update_node_status",
                "arguments": {
                    "node_id": "node-1",
                    "status": "maintenance"
                }
            })),
        };

        let response = server.handle_request(request).await;
        assert!(response.error.is_none());

        // Verify status changed
        let nodes = server.nodes.read().await;
        assert_eq!(nodes[0].status, "maintenance");
    }

    // ===== Authenticated Server Tests =====

    fn create_test_validator() -> TokenValidator {
        TokenValidator::new("test-secret".as_bytes(), "test_issuer".to_string())
    }

    fn create_test_user(scopes: Vec<String>) -> AuthenticatedUser {
        let now = Utc::now().timestamp();
        AuthenticatedUser {
            claims: JwtClaims {
                sub: "test_client".to_string(),
                aud: "test_audience".to_string(),
                exp: now + 3600,
                iat: now,
                iss: "test_issuer".to_string(),
                jti: "test_jti".to_string(),
                node_id: None,
                scopes,
            },
        }
    }

    #[tokio::test]
    async fn test_auth_server_list_tools_success() {
        let validator = create_test_validator();
        let server = AuthenticatedMCPServer::new(validator);
        let user = create_test_user(vec!["mcp:read".to_string()]);
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(1)),
            method: "tools/list".to_string(),
            params: None,
        };

        let response = server.handle_request(request, user).await;
        assert!(response.error.is_none());
        assert!(response.result.is_some());
    }

    #[tokio::test]
    async fn test_auth_server_list_tools_fail_no_scope() {
        let validator = create_test_validator();
        let server = AuthenticatedMCPServer::new(validator);
        let user = create_test_user(vec![]);
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(1)),
            method: "tools/list".to_string(),
            params: None,
        };

        let response = server.handle_request(request, user).await;
        assert_eq!(response.error, Some(MCPError::insufficient_permissions()));
    }

    #[tokio::test]
    async fn test_auth_server_call_tool_success() {
        let validator = create_test_validator();
        let server = AuthenticatedMCPServer::new(validator);
        let user = create_test_user(vec!["mcp:call".to_string()]);
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(2)),
            method: "tools/call".to_string(),
            params: Some(json!({ "name": "admin_get_dashboard_stats" })),
        };

        let response = server.handle_request(request, user).await;
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_auth_server_call_tool_fail_no_scope() {
        let validator = create_test_validator();
        let server = AuthenticatedMCPServer::new(validator);
        let user = create_test_user(vec!["mcp:read".to_string()]); // wrong scope
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(2)),
            method: "tools/call".to_string(),
            params: Some(json!({ "name": "admin_get_dashboard_stats" })),
        };

        let response = server.handle_request(request, user).await;
        assert_eq!(response.error, Some(MCPError::insufficient_permissions()));
    }
}
