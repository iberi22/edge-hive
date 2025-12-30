//! System metrics monitor
use sysinfo::{CpuExt, System, SystemExt};

#[derive(Debug, Clone, serde::Serialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub memory_total: u64,
}

pub struct SystemMonitor {
    system: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }

    pub fn refresh(&mut self) -> SystemMetrics {
        self.system.refresh_cpu();
        self.system.refresh_memory();

        let cpu_usage = self.system.global_cpu_info().cpu_usage() * 100.0;
        let memory_usage = self.system.used_memory();
        let memory_total = self.system.total_memory();

        SystemMetrics {
            cpu_usage,
            memory_usage,
            memory_total,
        }
    }
}
