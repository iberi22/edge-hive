//! Host context for WASM plugins
//!
//! Provides interfaces for WASM guests to interact with the host environment,
//! including database access and logging.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

/// Log level for host logging
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Defines the functions provided by the host to WASM guests
pub trait HostContext: Send + Sync + 'static {
    /// Query the database
    fn query(&self, sql: &str) -> Result<Value>;

    /// Log a message
    fn log(&self, level: LogLevel, msg: &str);
}

/// A shared, thread-safe host context
pub type SharedHostContext = Arc<dyn HostContext>;

/// A no-op host context for testing
pub struct NoOpHostContext;

impl NoOpHostContext {
    pub fn new() -> Self {
        Self
    }
}

impl HostContext for NoOpHostContext {
    fn query(&self, _sql: &str) -> Result<Value> {
        Ok(serde_json::json!({}))
    }

    fn log(&self, _level: LogLevel, _msg: &str) {}
}
