use chrono::{DateTime, Utc};

pub struct NetworkMetrics {
    pub timestamp: DateTime<Utc>,
    pub interfaces: Vec<NetworkInterfaces>,
}

#[derive(Default)]
pub struct NetworkInterfaces {
    pub name: String,
    pub errors_on_received: u64,
    pub total_errors_on_received: u64,
    pub errors_on_transmitted: u64,
    pub total_errors_on_transmitted: u64,
    pub packets_received: u64,
    pub total_packets_received: u64,
    pub packets_transmitted: u64,
    pub total_packets_transmitted: u64,
    pub bytes_received: u64,
    pub total_bytes_received: u64,
    pub bytes_transmitted: u64,
    pub total_bytes_transmitted: u64,
}
