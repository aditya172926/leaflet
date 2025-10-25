use chrono::{DateTime, Utc};
use sysinfo::System;

#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu_count: u8,
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub swap_used: u64,
    pub swap_total: u64
}

pub struct SystemCollector {
    system: System
}

impl SystemCollector {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {
            system
        }
    }

    pub fn collect() -> Result<SystemMetrics> {
        
    }
}