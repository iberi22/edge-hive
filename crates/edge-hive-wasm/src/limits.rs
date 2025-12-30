use wasmtime::ResourceLimiter;

/// Enforces memory and table limits on a Wasmtime store.
pub struct StoreLimits {
    /// Maximum linear memory size in bytes.
    max_memory_size: usize,
    /// Maximum number of tables.
    max_tables: u32,
}

impl StoreLimits {
    /// Create a new `StoreLimits` with the given constraints.
    pub fn new(max_memory_size: usize, max_tables: u32) -> Self {
        Self {
            max_memory_size,
            max_tables,
        }
    }
}

impl ResourceLimiter for StoreLimits {
    fn memory_growing(
        &mut self,
        _current: usize,
        desired: usize,
        _maximum: Option<usize>,
    ) -> anyhow::Result<bool> {
        if desired > self.max_memory_size {
            anyhow::bail!(
                "Memory allocation of {} bytes exceeds the limit of {}",
                desired,
                self.max_memory_size
            );
        }
        Ok(true)
    }

    fn table_growing(&mut self, _current: usize, desired: usize, _maximum: Option<usize>) -> anyhow::Result<bool> {
        if desired as u32 > self.max_tables {
            anyhow::bail!(
                "Table allocation of {} elements exceeds the limit of {}",
                desired,
                self.max_tables
            );
        }
        Ok(true)
    }
}
