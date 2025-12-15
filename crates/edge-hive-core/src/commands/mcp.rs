use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

// --- Public Interface ---

#[derive(Parser, Debug)]
pub struct McpArgs {}

pub async fn run(_args: McpArgs) -> anyhow::Result<()> {
    let mut server = McpServer::new("edge-hive-mcp", "0.1.0");

    // Register Tools
    server.register_tool(Tool {
        name: "get_status".into(),
        description: "Get current node status".into(),
        input_schema: json!({ "type": "object", "properties": {} }),
        handler: Box::new(|_params| Box::pin(async {
            Ok(json!({
                "content": [{ "type": "text", "text": "Node: grand-alpha-07\nStatus: Running\nTunnel: Active" }]
            }))
        })),
    });

    server.register_tool(Tool {
        name: "provision_node".into(),
        description: "Provision a cloud node".into(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "region": { "type": "string" },
                "size": { "type": "string" }
            },
            "required": ["region"]
        }),
        handler: Box::new(|params| Box::pin(async move {
            let region = params.as_ref()
                .and_then(|p| p.get("region"))
                .and_then(|s| s.as_str())
                .unwrap_or("us-east-1");

            Ok(json!({
                "content": [{ "type": "text", "text": format!("Provisioning node in {}", region) }]
            }))
        })),
    });

    // Register Resources
    server.register_resource(Resource {
        uri: "edge-hive://logs/last".into(),
        name: "Last Logs".into(),
        mime_type: "text/plain".into(),
        handler: Box::new(|_uri| Box::pin(async {
            Ok(json!({
                "contents": [{
                    "uri": "edge-hive://logs/last",
                    "mimeType": "text/plain",
                    "text": "[INFO] Server started\n[INFO] Peer connected"
                }]
            }))
        })),
    });

    server.serve_stdio().await
}

// --- Library Implementation ---

type ToolHandler = Box<dyn Fn(Option<Value>) -> Pin<Box<dyn Future<Output = Result<Value, String>> + Send>> + Send + Sync>;
type ResourceHandler = Box<dyn Fn(String) -> Pin<Box<dyn Future<Output = Result<Value, String>> + Send>> + Send + Sync>;

struct Tool {
    name: String,
    description: String,
    input_schema: Value,
    handler: ToolHandler,
}

struct Resource {
    uri: String,
    name: String,
    mime_type: String,
    handler: ResourceHandler,
}

struct McpServer {
    name: String,
    version: String,
    tools: HashMap<String, Tool>,
    resources: HashMap<String, Resource>,
}

impl McpServer {
    fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            tools: HashMap::new(),
            resources: HashMap::new(),
        }
    }

    fn register_tool(&mut self, tool: Tool) {
        self.tools.insert(tool.name.clone(), tool);
    }

    fn register_resource(&mut self, resource: Resource) {
        self.resources.insert(resource.uri.clone(), resource);
    }

    async fn serve_stdio(self) -> anyhow::Result<()> {
        let server = Arc::new(self);
        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin).lines();

        while let Some(line) = reader.next_line().await? {
            if line.is_empty() { continue; }

            let req: JsonRpcRequest = match serde_json::from_str(&line) {
                Ok(r) => r,
                Err(_) => continue,
            };

            let server_clone = server.clone();
            let response = server_clone.handle_request(req).await;

            let resp_str = serde_json::to_string(&response)?;
            stdout.write_all(resp_str.as_bytes()).await?;
            stdout.write_all(b"\n").await?;
            stdout.flush().await?;
        }
        Ok(())
    }

    async fn handle_request(&self, req: JsonRpcRequest) -> JsonRpcResponse {
        match req.method.as_str() {
            "initialize" => JsonRpcResponse::success(req.id, json!({
                "protocolVersion": "2024-11-05",
                "serverInfo": { "name": self.name, "version": self.version },
                "capabilities": {
                    "tools": {},
                    "resources": {}
                }
            })),
            "tools/list" => {
                let tools_json: Vec<Value> = self.tools.values().map(|t| json!({
                    "name": t.name,
                    "description": t.description,
                    "inputSchema": t.input_schema
                })).collect();
                JsonRpcResponse::success(req.id, json!({ "tools": tools_json }))
            },
            "tools/call" => {
                let name = req.params.as_ref().and_then(|p| p.get("name").and_then(|s| s.as_str()));
                let args = req.params.as_ref().and_then(|p| p.get("arguments")).cloned();

                if let Some(tool_name) = name {
                    if let Some(tool) = self.tools.get(tool_name) {
                        match (tool.handler)(args).await {
                            Ok(res) => JsonRpcResponse::success(req.id, res),
                            Err(e) => JsonRpcResponse::error(req.id, -32000, &e)
                        }
                    } else {
                        JsonRpcResponse::error(req.id, -32601, "Tool not found")
                    }
                } else {
                    JsonRpcResponse::error(req.id, -32602, "Missing tool name")
                }
            },
            "resources/list" => {
                let resources_json: Vec<Value> = self.resources.values().map(|r| json!({
                    "uri": r.uri,
                    "name": r.name,
                    "mimeType": r.mime_type
                })).collect();
                JsonRpcResponse::success(req.id, json!({ "resources": resources_json }))
            },
            "resources/read" => {
                let uri = req.params.as_ref().and_then(|p| p.get("uri").and_then(|s| s.as_str()));
                if let Some(uri_str) = uri {
                    if let Some(res) = self.resources.get(uri_str) {
                         match (res.handler)(uri_str.to_string()).await {
                            Ok(v) => JsonRpcResponse::success(req.id, v),
                            Err(e) => JsonRpcResponse::error(req.id, -32000, &e)
                         }
                    } else {
                        JsonRpcResponse::error(req.id, -32602, "Resource not found")
                    }
                } else {
                    JsonRpcResponse::error(req.id, -32602, "Missing URI")
                }
            },
            "ping" => JsonRpcResponse::success(req.id, json!({})),
            _ => JsonRpcResponse::error(req.id, -32601, "Method not found")
        }
    }
}

// --- JSON-RPC Types ---

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<Value>,
    id: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    result: Option<Value>,
    error: Option<JsonRpcError>,
    id: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}

impl JsonRpcResponse {
    fn success(id: Option<Value>, result: Value) -> Self {
        Self { jsonrpc: "2.0".into(), result: Some(result), error: None, id }
    }
    fn error(id: Option<Value>, code: i32, message: &str) -> Self {
        Self { jsonrpc: "2.0".into(), result: None, error: Some(JsonRpcError { code, message: message.into() }), id }
    }
}
