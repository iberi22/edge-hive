//! Edge Hive WASM - A modular WebAssembly runtime for Edge Functions.
//!
//! This crate provides the necessary infrastructure to run sandboxed WebAssembly
//! modules, with a clean separation between the host and guest environments.

// Guest-facing prelude, always available.
pub mod prelude;
pub mod error;

// Host-facing components, compiled only with the "host" feature.
#[cfg(feature = "host")]
pub mod function;
#[cfg(feature = "host")]
pub mod host;
#[cfg(feature = "host")]
pub mod runtime;

#[cfg(feature = "host")]
pub use function::EdgeFunction;
#[cfg(feature = "host")]
pub use host::HostContext;
#[cfg(feature = "host")]
pub use runtime::WasmRuntime;

pub use error::WasmError;


/// Validates the magic bytes of a WASM module.
pub fn validate_wasm_bytes(bytes: &[u8]) -> Result<(), WasmError> {
    if !bytes.starts_with(b"\0asm") {
        return Err(WasmError::InvalidFile("Invalid magic bytes".into()));
    }
    Ok(())
}
