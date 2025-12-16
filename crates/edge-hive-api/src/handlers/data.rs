//! Database CRUD handlers with automatic caching

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::state::ApiState;

#[derive(Deserialize)]
pub struct QueryParams {
    limit: Option<u64>,
    offset: Option<u64>,
}

/// Query records from a table (auto-cached)
pub async fn query_records(
    State(state): State<ApiState>,
    Path(table): Path<String>,
) -> Result<Json<Vec<Value>>, StatusCode> {
    // Cache key: "data:{table}:query"
    let cache_key = format!("data:{}:query", table);
    
    // Try cache first
    let mut cache = state.cache.lock().await;
    if let Some(cached) = cache.get(&cache_key).await {
        if let Ok(records) = serde_json::from_slice::<Vec<Value>>(&cached) {
            return Ok(Json(records));
        }
    }

    // Placeholder: would query from edge-hive-db
    let records = vec![];
    
    // Cache the result
    if let Ok(serialized) = serde_json::to_vec(&records) {
        let _ = cache.set(cache_key, serialized).await;
    }

    Ok(Json(records))
}

/// Insert a record into a table
pub async fn insert_record(
    State(state): State<ApiState>,
    Path(table): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // Invalidate cache for this table
    let cache_key = format!("data:{}:query", table);
    let mut cache = state.cache.lock().await;
    cache.delete(&cache_key).await;

    // Placeholder: would insert into edge-hive-db
    Ok(Json(payload))
}

/// Update a record by ID
pub async fn update_record(
    State(state): State<ApiState>,
    Path((table, id)): Path<(String, String)>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // Invalidate cache
    let cache_key = format!("data:{}:query", table);
    let mut cache = state.cache.lock().await;
    cache.delete(&cache_key).await;

    // Placeholder: would update in edge-hive-db
    Ok(Json(payload))
}

/// Delete a record by ID
pub async fn delete_record(
    State(state): State<ApiState>,
    Path((table, id)): Path<(String, String)>,
) -> StatusCode {
    // Invalidate cache
    let cache_key = format!("data:{}:query", table);
    let mut cache = state.cache.lock().await;
    cache.delete(&cache_key).await;

    // Placeholder: would delete from edge-hive-db
    StatusCode::NO_CONTENT
}
