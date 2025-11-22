use std::collections::VecDeque;

use chrono::Utc;
use sysinfo::System;

use crate::{collectors::{structs::{MetricsCategory, MetricsHistory}, system::metrics::{SystemCollector, SystemMetrics}}, constants::MAX_HISTORY};

impl SystemCollector {
    pub fn new(store_history: bool) -> Self {
        let system_metrics = match store_history {
            true => MetricsHistory::History(VecDeque::<SystemMetrics>::with_capacity(MAX_HISTORY)),
            false => MetricsHistory::Single(SystemMetrics::default()),
        };
        Self {
            system_metrics,
        }
    }

    pub fn collect(&mut self, refresh_kind: MetricsCategory, system: &mut System) -> anyhow::Result<SystemMetrics> {
        refresh_kind.refresh_metrics(system);

        let cpu_count = system.cpus().len();
        let cpu_usage = system.global_cpu_usage();
        let memory_used = system.used_memory();
        let memory_total = system.total_memory();
        let swap_used = system.used_swap();
        let swap_total = system.total_swap();

        let system_metric = SystemMetrics {
            timestamp: Utc::now(),
            cpu_count,
            cpu_usage,
            memory_used,
            memory_total,
            swap_used,
            swap_total
        };

        match &mut self.system_metrics {
            MetricsHistory::Single(sys) => *sys = system_metric.clone(),
            MetricsHistory::History(sys_vec) => {
                if sys_vec.len() > MAX_HISTORY {
                    sys_vec.pop_front();
                }
                sys_vec.push_back(system_metric.clone());
            }
        }

        Ok(system_metric)
    }
}