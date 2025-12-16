//! Database CRUD handlers with automatic caching

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::Value;
use crate::state::ApiState;

#[derive(Deserialize)]
pub struct QueryParams {
    limit: Option<u64>,
    offset: Option<u64>,
}

fn is_safe_ident(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn is_safe_record_id(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

/// Query records from a table (auto-cached)
pub async fn query_records(
    Extension(state): Extension<ApiState>,
    Path(table): Path<String>,
) -> Result<Json<Vec<Value>>, StatusCode> {
    if !is_safe_ident(&table) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Cache key: "data:{table}:query"
    let cache_key = format!("data:{}:query", table);

    // Try cache first
    {
        let mut cache = state.cache.lock().await;
        if let Some(cached) = cache.get(&cache_key).await {
            if let Ok(records) = serde_json::from_slice::<Vec<Value>>(&cached) {
                return Ok(Json(records));
            }
        }
    }

    // Query from SurrealDB
    let sql = format!("SELECT * FROM {};", table);
    let records: Vec<Value> = state
        .db
        .query_json(&sql)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Cache the result
    if let Ok(serialized) = serde_json::to_vec(&records) {
        let mut cache = state.cache.lock().await;
        let _ = cache.set(cache_key, serialized).await;
    }

    Ok(Json(records))
}

/// Insert a record into a table
pub async fn insert_record(
    Extension(state): Extension<ApiState>,
    Path(table): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    if !is_safe_ident(&table) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Invalidate cache for this table
    let cache_key = format!("data:{}:query", table);
    let mut cache = state.cache.lock().await;
    cache.delete(&cache_key).await;

    let payload_json = serde_json::to_string(&payload).map_err(|_| StatusCode::BAD_REQUEST)?;
    let sql = format!("CREATE {} CONTENT {};", table, payload_json);

    let created: Vec<Value> = state
        .db
        .query_json(&sql)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(first) = created.into_iter().next() {
        return Ok(Json(first));
    }

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

/// Update a record by ID
pub async fn update_record(
    Extension(state): Extension<ApiState>,
    Path((table, id)): Path<(String, String)>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    if !is_safe_ident(&table) || !is_safe_record_id(&id) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Invalidate cache
    let cache_key = format!("data:{}:query", table);
    let mut cache = state.cache.lock().await;
    cache.delete(&cache_key).await;

    let payload_json = serde_json::to_string(&payload).map_err(|_| StatusCode::BAD_REQUEST)?;
    let sql = format!("UPDATE {}:{} MERGE {};", table, id, payload_json);

    let updated: Vec<Value> = state
        .db
        .query_json(&sql)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(first) = updated.into_iter().next() {
        return Ok(Json(first));
    }

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

/// Delete a record by ID
pub async fn delete_record(
    Extension(state): Extension<ApiState>,
    Path((table, id)): Path<(String, String)>,
) -> StatusCode {
    if !is_safe_ident(&table) || !is_safe_record_id(&id) {
        return StatusCode::BAD_REQUEST;
    }

    // Invalidate cache
    let cache_key = format!("data:{}:query", table);
    let mut cache = state.cache.lock().await;
    cache.delete(&cache_key).await;

    let sql = format!("DELETE {}:{};", table, id);
    let _ = state.db.query(&sql).await;

    // Placeholder: would delete from edge-hive-db
    StatusCode::NO_CONTENT
}
