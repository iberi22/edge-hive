//! Edge Hive WASM - WebAssembly plugin runtime
//!
//! Provides sandboxed execution of WASM plugins using Wasmtime.

use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;
use tracing::info;
use wasmtime::*;

/// Errors that can occur during WASM operations
#[derive(Debug, Error)]
pub enum WasmError {
    #[error("Failed to load module: {0}")]
    Load(String),

    #[error("Failed to instantiate module: {0}")]
    Instantiate(String),

    #[error("Failed to call function: {0}")]
    Call(String),

    #[error("Wasmtime error: {0}")]
    Wasmtime(#[from] wasmtime::Error),
}

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
}

/// WASM plugin instance
pub struct Plugin {
    #[allow(dead_code)]
    engine: Engine,
    #[allow(dead_code)]
    store: Store<()>,
    instance: Instance,
    info: PluginInfo,
}

impl Plugin {
    /// Load a plugin from a WASM file
    pub fn load(path: &Path) -> Result<Self, WasmError> {
        info!("🔌 Loading plugin from {:?}", path);

        let engine = Engine::default();
        let mut store = Store::new(&engine, ());

        let module = Module::from_file(&engine, path)?;
        let instance = Instance::new(&mut store, &module, &[])?;

        // Try to get plugin info from exported function
        let info = Self::get_plugin_info(&mut store, &instance)
            .unwrap_or_else(|_| PluginInfo {
                name: path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
                version: "0.0.0".into(),
                description: "No description".into(),
                author: None,
            });

        Ok(Self {
            engine,
            store,
            instance,
            info,
        })
    }

    fn get_plugin_info(store: &mut Store<()>, instance: &Instance) -> Result<PluginInfo, WasmError> {
        // Look for an exported `plugin_info` function
        let func = instance
            .get_func(&mut *store, "plugin_info")
            .ok_or_else(|| WasmError::Call("No plugin_info function".into()))?;

        // For now, return a placeholder
        // Real implementation would call the function and parse the result
        Err(WasmError::Call("Plugin info not implemented".into()))
    }

    /// Get plugin information
    pub fn info(&self) -> &PluginInfo {
        &self.info
    }

    /// Call a function on the plugin
    pub fn call(&mut self, name: &str, args: &[Val]) -> Result<Vec<Val>, WasmError> {
        let func = self.instance
            .get_func(&mut self.store, name)
            .ok_or_else(|| WasmError::Call(format!("Function '{}' not found", name)))?;

        let func_type = func.ty(&self.store);
        let mut results = vec![Val::I32(0); func_type.results().len()];

        func.call(&mut self.store, args, &mut results)?;

        Ok(results)
    }
}

/// Plugin manager for loading and managing multiple plugins
pub struct PluginManager {
    plugins: Vec<Plugin>,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        Self { plugins: vec![] }
    }

    /// Load a plugin from a file
    pub fn load(&mut self, path: &Path) -> Result<usize, WasmError> {
        let plugin = Plugin::load(path)?;
        let index = self.plugins.len();
        self.plugins.push(plugin);
        Ok(index)
    }

    /// Get all loaded plugins
    pub fn plugins(&self) -> &[Plugin] {
        &self.plugins
    }

    /// Get a plugin by index
    pub fn get(&mut self, index: usize) -> Option<&mut Plugin> {
        self.plugins.get_mut(index)
    }

    /// List all plugin info
    pub fn list(&self) -> Vec<&PluginInfo> {
        self.plugins.iter().map(|p| p.info()).collect()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert!(manager.plugins().is_empty());
    }
}
