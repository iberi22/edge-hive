//! Prelude for Edge Function guest development.
//!
//! This module provides common types and macros for writing Edge Functions
//! that can be compiled to WASM and executed by the Edge Hive runtime.

pub use serde::{Deserialize, Serialize};
pub use serde_json::Value;

/// Represents an incoming HTTP request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Value,
}

/// Represents an outgoing HTTP response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status: u16,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Value,
}

impl Response {
    /// Creates a new JSON response.
    pub fn json(status: u16, body: Value) -> Self {
        let mut headers = std::collections::HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        Self {
            status,
            headers,
            body,
        }
    }

    /// Creates a new plain text response.
    pub fn text(status: u16, body: &str) -> Self {
        let mut headers = std::collections::HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        Self {
            status,
            headers,
            body: Value::String(body.to_string()),
        }
    }
}
