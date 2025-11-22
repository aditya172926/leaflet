use chrono::{DateTime, Utc};

#[derive(Debug, Default, Clone)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu_count: usize,
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub swap_used: u64,
    pub swap_total: u64
}

#[derive(Debug)]
pub struct SystemCollector {
    pub system_metrics: SystemMetrics,
}