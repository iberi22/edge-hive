use thiserror::Error;

/// Errors that can occur during WASM operations.
#[derive(Debug, Error)]
pub enum WasmError {
    #[error("Failed to load module from bytes: {0}")]
    Load(String),

    #[error("Failed to instantiate module: {0}")]
    Instantiate(String),

    #[cfg(feature = "host")]
    #[error("Failed to call function '{func}': {source}")]
    Call {
        func: String,
        source: anyhow::Error,
    },

    #[cfg(feature = "host")]
    #[error("Wasmtime error: {0}")]
    Wasmtime(#[from] wasmtime::Error),

    #[error("Invalid WASM file: {0}")]
    InvalidFile(String),

    #[error("Memory access error: {0}")]
    Memory(String),

    #[error("Function not found: {0}")]
    FunctionNotFound(String),
}
