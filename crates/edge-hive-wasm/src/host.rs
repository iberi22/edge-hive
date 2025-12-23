//! Host-provided functionalities for WASM guests.
use async_trait::async_trait;
use serde_json::Value;

/// Log levels for guest modules.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl From<u32> for LogLevel {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Trace,
            1 => Self::Debug,
            2 => Self::Info,
            3 => Self::Warn,
            4 => Self::Error,
            _ => Self::Info, // Default to Info
        }
    }
}

/// A trait defining the functions that the host environment must provide to
/// the WASM guest. This allows for decoupling the runtime from the database
/// and other host-specific implementations.
#[async_trait]
pub trait HostContext: Send + Sync + 'static {
    /// Executes a database query and returns the result as a JSON `Value`.
    async fn query(&self, sql: &str) -> Result<Value, String>;

    /// Logs a message at the specified level.
    fn log(&self, level: LogLevel, msg: &str);
}
