use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use tower::ServiceExt;
use edge_hive_api::{create_router, ApiState};
use edge_hive_cache::CacheService;
use edge_hive_db::DatabaseService;
use edge_hive_realtime::RealtimeServer;
use std::sync::Arc;
use tempfile::tempdir;
use tokio::fs;
use mime_multipart::{generate_boundary, write_multipart};
use std::path::PathBuf;
use serde_json::json;

async fn setup_test_app() -> axum::Router {
    let temp_dir = tempdir().unwrap();
    let data_dir = temp_dir.path().to_path_buf();
    let cache = CacheService::new(Default::default()).await;
    let db_path = data_dir.join("test.db");
    let db = Arc::new(DatabaseService::new(&db_path).await.unwrap());
    let realtime = RealtimeServer::new(Default::default());
    let state = ApiState::new(cache, db, realtime, data_dir);
    create_router(state)
}

fn get_test_wasm_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("tests").join("test.wasm")
}

#[tokio::test]
async fn test_wasm_lifecycle() {
    let app = setup_test_app().await;
    let wasm_path = get_test_wasm_path();

    // 1. Deploy a new function
    let wasm_content = fs::read(&wasm_path).await.unwrap();
    let boundary = generate_boundary();
    let mut body = Vec::new();
    write_multipart(
        &mut body,
        boundary.clone(),
        vec![(
            "wasm_file",
            "test.wasm",
            "application/wasm",
            wasm_content.into(),
        )],
    )
    .unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/edge/test-function")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={}", boundary),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!({
        "status": "deployed",
        "name": "test-function",
        "version": 1
    }));

    // 2. Get function versions
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/edge/test-function/versions")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!({
        "name": "test-function",
        "versions": [1],
        "active_version": 1
    }));

    // 3. Rollback to a specific version
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/edge/test-function/rollback")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"version": 1}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!({
        "status": "rolled_back",
        "name": "test-function",
        "active_version": 1
    }));

    // 4. Delete the function
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/edge/test-function")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!({
        "status": "deleted",
        "name": "test-function"
    }));
}

#[tokio::test]
async fn test_wasm_failures() {
    let app = setup_test_app().await;

    // 1. Deploy invalid WASM
    let boundary = generate_boundary();
    let mut body = Vec::new();
    write_multipart(
        &mut body,
        boundary.clone(),
        vec![(
            "wasm_file",
            "test.wasm",
            "application/wasm",
            b"invalid".to_vec().into(),
        )],
    )
    .unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/edge/test-function")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={}", boundary),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // 2. Get versions for non-existent function
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/edge/non-existent/versions")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // 3. Rollback non-existent function
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/edge/non-existent/rollback")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"version": 1}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // 4. Delete non-existent function
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/edge/non-existent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
