use chrono::Utc;
use sysinfo::System;

use crate::collectors::{structs::MetricsCategory, system::metrics::{SystemCollector, SystemMetrics}};

impl SystemCollector {
    pub fn fetch(refresh_kind: MetricsCategory, system: &mut System) -> Self {
        refresh_kind.refresh_metrics(system);

        let cpu_count = system.cpus().len();
        let cpu_usage = system.global_cpu_usage();
        let memory_used = system.used_memory();
        let memory_total = system.total_memory();
        let swap_used = system.used_swap();
        let swap_total = system.total_swap();

        Self {
            system_metrics: SystemMetrics {
            timestamp: Utc::now(),
            cpu_count,
            cpu_usage,
            memory_used,
            memory_total,
            swap_used,
            swap_total
        }}
    }
}