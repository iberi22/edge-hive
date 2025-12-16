use edge_hive_core::{auth::OAuth2State, server};
use edge_hive_discovery::DiscoveryService;
use futures::{SinkExt, StreamExt};
use serde_json::json;
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use tempfile::{tempdir, TempDir};
use tokio::{net::TcpListener, sync::{oneshot, RwLock}};

use edge_hive_realtime::{ClientMessage, ServerMessage};

struct TestServer {
    addr: SocketAddr,
    shutdown_tx: oneshot::Sender<()>,
    _data_dir_guard: TempDir,
}

async fn spawn_test_server() -> TestServer {
    let dir = tempdir().expect("tempdir");
    let data_dir = dir.path().to_path_buf();

    let discovery = DiscoveryService::new().expect("discovery");
    let discovery_svc = Arc::new(RwLock::new(discovery));

    let message_store: server::MessageStore = Arc::new(RwLock::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("local_addr");

    // OAuth state
    let secret = vec![7u8; 32];
    let issuer = format!("http://{}:{}", addr.ip(), addr.port());
    let oauth_state = OAuth2State::load_or_new(&secret, issuer, &data_dir)
        .await
        .expect("oauth");

    // Core server state
    let app_state = server::AppState {
        discovery_svc,
        message_store,
        data_dir: data_dir.clone(),
    };

    // API gateway components (cache + db + realtime)
    let cache = edge_hive_cache::CacheService::new(edge_hive_cache::CacheConfig::default()).await;
    let db_path = data_dir.join("edge-hive.db");
    let db = Arc::new(edge_hive_db::DatabaseService::new(&db_path).await.expect("db"));
    let realtime = edge_hive_realtime::RealtimeServer::new(edge_hive_realtime::RealtimeServerConfig::default())
        .with_db(db.clone());
    let api_state = edge_hive_api::ApiState::new(cache, db, realtime);
    let api_router = edge_hive_api::create_router(api_state);

    let app = server::build_router()
        .merge(api_router)
        .layer(axum::Extension(app_state))
        .layer(axum::Extension(oauth_state));

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        let shutdown = async {
            let _ = shutdown_rx.await;
        };
        let _ = axum::serve(listener, app).with_graceful_shutdown(shutdown).await;
    });

    TestServer {
        addr,
        shutdown_tx,
        _data_dir_guard: dir,
    }
}

fn extract_record_id(value: &serde_json::Value) -> Option<String> {
    let id_value = value.get("id")?;

    if let Some(s) = id_value.as_str() {
        // Often like "table:xyz".
        return Some(
            s.split(':')
                .nth(1)
                .unwrap_or(s)
                .to_string(),
        );
    }

    // Surreal can also serialize a Thing-like object.
    if let Some(obj) = id_value.as_object() {
        // Common SurrealDB shape: {"tb": "items", "id": "xyz"} (or nested id).
        if let Some(tb) = obj.get("tb").and_then(|v| v.as_str()) {
            if let Some(inner) = obj.get("id") {
                if let Some(inner_str) = inner.as_str() {
                    return Some(inner_str.to_string());
                }
                if let Some(inner_obj) = inner.as_object() {
                    // Sometimes nested like {"String": "xyz"}.
                    if let Some((_, v)) = inner_obj.iter().next() {
                        if let Some(s) = v.as_str() {
                            return Some(s.to_string());
                        }
                    }
                }
            }

            // Fallback: table-qualified string if present.
            let _ = tb;
        }

        if let Some(inner) = obj.get("id") {
            if let Some(inner_str) = inner.as_str() {
                return Some(inner_str.to_string());
            }
        }
    }

    None
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn e2e_vps_http_health_data_edge_functions() {
    let TestServer { addr, shutdown_tx, _data_dir_guard } = spawn_test_server().await;
    let base = format!("http://{}:{}", addr.ip(), addr.port());
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("reqwest client");

    // Core health
    let core_health = client
        .get(format!("{base}/health"))
        .send()
        .await
        .expect("GET /health");
    assert!(core_health.status().is_success());

    // API health
    let api_health: serde_json::Value = client
        .get(format!("{base}/api/v1/health"))
        .send()
        .await
        .expect("GET /api/v1/health")
        .json()
        .await
        .expect("json");
    assert_eq!(api_health.get("status").and_then(|v| v.as_str()), Some("ok"));

    // Edge functions: list
    let functions: serde_json::Value = client
        .get(format!("{base}/api/v1/edge"))
        .send()
        .await
        .expect("GET /api/v1/edge")
        .json()
        .await
        .expect("json");
    assert!(functions.is_array());

    // Edge functions: execute
    let exec: serde_json::Value = client
        .post(format!("{base}/api/v1/edge/example-function"))
        .json(&json!({"hello": "world"}))
        .send()
        .await
        .expect("POST /api/v1/edge/example-function")
        .json()
        .await
        .expect("json");
    assert!(exec.get("result").is_some());

    // Data CRUD (schemaless table)
    let created_resp = client
        .post(format!("{base}/api/v1/data/items"))
        .json(&json!({"name": "alpha", "kind": "test"}))
        .send()
        .await
        .expect("POST /api/v1/data/items");
    let created_status = created_resp.status();
    let created_text = created_resp.text().await.expect("created body");
    assert!(
        created_status.is_success(),
        "POST /api/v1/data/items failed: status={} body={}",
        created_status,
        created_text
    );
    let created: serde_json::Value = serde_json::from_str(&created_text).expect("created json");

    let record_id = extract_record_id(&created).unwrap_or_else(|| {
        panic!("record id (created={})", created);
    });

    // Query should include at least one record
    let list1: serde_json::Value = client
        .get(format!("{base}/api/v1/data/items"))
        .send()
        .await
        .expect("GET /api/v1/data/items")
        .json()
        .await
        .expect("json");
    assert!(list1.is_array());

    // Update
    let updated: serde_json::Value = client
        .put(format!("{base}/api/v1/data/items/{record_id}"))
        .json(&json!({"name": "beta"}))
        .send()
        .await
        .expect("PUT /api/v1/data/items/:id")
        .json()
        .await
        .expect("json");
    assert!(updated.get("name").is_some());

    // Delete
    let deleted = client
        .delete(format!("{base}/api/v1/data/items/{record_id}"))
        .send()
        .await
        .expect("DELETE /api/v1/data/items/:id");
    assert_eq!(deleted.status(), reqwest::StatusCode::NO_CONTENT);

    let _ = shutdown_tx.send(());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn e2e_vps_realtime_live_query_events() {
    let TestServer { addr, shutdown_tx, _data_dir_guard } = spawn_test_server().await;

    let base = format!("http://{}:{}", addr.ip(), addr.port());
    let ws_url = format!("ws://{}:{}/api/v1/realtime", addr.ip(), addr.port());

    // Connect WS
    let (mut ws, _) = tokio_tungstenite::connect_async(ws_url.as_str())
        .await
        .expect("connect ws");

    // Subscribe to topic "items"
    let subscribe = ClientMessage::Subscribe {
        topic: "items".to_string(),
        filter: None,
    };
    ws.send(tokio_tungstenite::tungstenite::Message::Text(
        serde_json::to_string(&subscribe).unwrap(),
    ))
    .await
    .expect("send subscribe");

    // Wait for subscribe ACK so the Live Query pump is guaranteed to be running.
    let _ = tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            let next = ws
                .next()
                .await
                .expect("ws msg")
                .expect("ws ok");
            if let tokio_tungstenite::tungstenite::Message::Text(t) = next {
                if let Ok(parsed) = serde_json::from_str::<ServerMessage>(&t) {
                    match parsed {
                        ServerMessage::Ack { message } => {
                            if message.contains("Subscribed") {
                                return;
                            }
                        }
                        ServerMessage::Error { message } => {
                            panic!("subscribe error: {message}");
                        }
                        _ => {}
                    }
                }
            }
        }
    })
    .await
    .expect("subscribe ack timeout");

    // Trigger a change via HTTP
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("reqwest client");
    let created_resp = client
        .post(format!("{base}/api/v1/data/items"))
        .json(&json!({"name": "rt"}))
        .send()
        .await
        .expect("POST /api/v1/data/items");
    assert!(
        created_resp.status().is_success(),
        "POST /api/v1/data/items (realtime) failed: status={}",
        created_resp.status()
    );

    // Expect at least one event for topic "items"
    let msg = tokio::time::timeout(Duration::from_secs(6), async {
        loop {
            let next = ws
                .next()
                .await
                .expect("ws msg")
                .expect("ws ok");
            if let tokio_tungstenite::tungstenite::Message::Text(t) = next {
                if let Ok(parsed) = serde_json::from_str::<ServerMessage>(&t) {
                    if let ServerMessage::Event { topic, action, .. } = parsed {
                        if topic == "items" && action == "create" {
                            return;
                        }
                    }
                }
            }
        }
    })
    .await;

    assert!(msg.is_ok(), "did not receive create event within timeout");

    let _ = shutdown_tx.send(());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn e2e_mcp_oauth_token_then_tools_call() {
    let TestServer { addr, shutdown_tx, _data_dir_guard } = spawn_test_server().await;
    let base = format!("http://{}:{}", addr.ip(), addr.port());
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("reqwest client");

    // Create client
    let created_resp = client
        .post(format!("{base}/mcp/auth/clients"))
        .json(&json!({"name": "e2e", "scopes": ["mcp:read", "mcp:call"]}))
        .send()
        .await
        .expect("POST /mcp/auth/clients");
    let created_status = created_resp.status();
    let created_text = created_resp.text().await.expect("created client body");
    assert!(
        created_status.is_success(),
        "POST /mcp/auth/clients failed: status={} body={}",
        created_status,
        created_text
    );
    let created: serde_json::Value = serde_json::from_str(&created_text).expect("created client json");

    let client_id = created
        .get("client_id")
        .and_then(|v| v.as_str())
        .unwrap_or_else(|| panic!("client_id (created={})", created));
    let client_secret = created
        .get("client_secret")
        .and_then(|v| v.as_str())
        .unwrap_or_else(|| panic!("client_secret (created={})", created));

    // Request token
    let token: serde_json::Value = client
        .post(format!("{base}/mcp/auth/token"))
        .json(&json!({
            "grant_type": "client_credentials",
            "client_id": client_id,
            "client_secret": client_secret,
            "scope": "mcp:call"
        }))
        .send()
        .await
        .expect("POST /mcp/auth/token")
        .json()
        .await
        .expect("json");

    let access_token = token.get("access_token").and_then(|v| v.as_str()).expect("access_token");

    // Call tool
    let tools: serde_json::Value = client
        .post(format!("{base}/mcp/tools/call"))
        .bearer_auth(access_token)
        .json(&json!({
            "method": "tools/call",
            "params": {"name": "get_status", "arguments": {}}
        }))
        .send()
        .await
        .expect("POST /mcp/tools/call")
        .json()
        .await
        .expect("json");

    assert!(tools.get("content").is_some());

    let _ = shutdown_tx.send(());
}
