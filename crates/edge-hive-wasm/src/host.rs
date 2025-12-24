//! Host context trait for dependency injection
//!
//! This module defines the trait that allows the WASM runtime to interact
//! with the host environment without creating a circular dependency.

use serde_json::Value;
use std::sync::Arc;

/// Log level for host logging
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Trait for host environment context
///
/// This trait allows the WASM runtime to receive implementations from the host
/// without depending on specific crates like edge-hive-db.
pub trait HostContext: Send + Sync + 'static {
    /// Execute a database query
    ///
    /// # Arguments
    /// * `sql` - SQL query string
    ///
    /// # Returns
    /// Result containing query results as JSON or error message
    fn query(&self, sql: &str) -> Result<Value, String>;

    /// Log a message
    ///
    /// # Arguments
    /// * `level` - Log level
    /// * `msg` - Message to log
    fn log(&self, level: LogLevel, msg: &str);
}

/// Default no-op host context for testing
#[derive(Debug, Clone)]
pub struct NoOpHostContext;

impl HostContext for NoOpHostContext {
    fn query(&self, _sql: &str) -> Result<Value, String> {
        Err("NoOpHostContext: query not implemented".to_string())
    }

    fn log(&self, _level: LogLevel, _msg: &str) {
        // No-op
    }
}

/// Shared host context type
pub type SharedHostContext = Arc<dyn HostContext>;
