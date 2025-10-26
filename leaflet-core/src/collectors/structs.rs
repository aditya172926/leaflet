use anyhow::Result;
use chrono::{DateTime, Utc};
use sysinfo::System;

#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu_count: usize,
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub swap_used: u64,
    pub swap_total: u64
}

pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub hostname: String
}

pub struct SystemCollector {
    system: System
}

impl Default for SystemCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemCollector {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {
            system
        }
    }

    pub fn collect(&mut self) -> Result<SystemMetrics> {
        self.system.refresh_all();
        let cpu_count = self.system.cpus().len();
        let cpu_usage = self.system.global_cpu_usage();
        let memory_used = self.system.used_memory();
        let memory_total = self.system.total_memory();
        let swap_used = self.system.used_swap();
        let swap_total = self.system.total_swap();

        Ok(SystemMetrics {
            timestamp: Utc::now(),
            cpu_count,
            cpu_usage,
            memory_used,
            memory_total,
            swap_used,
            swap_total
        })
    }

    pub fn system_info(&self) -> SystemInfo {
        SystemInfo {
            os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
            hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string())
        }
    }
}