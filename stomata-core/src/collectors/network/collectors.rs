use chrono::Utc;
use sysinfo::Networks;

use crate::collectors::network::metrics::{NetworkInterfaces, NetworkMetrics};

impl NetworkMetrics {
    pub fn fetch(networks: &Networks) -> Self {
        let timestamp = Utc::now();
        let interfaces: Vec<NetworkInterfaces> = networks
            .list()
            .iter()
            .map(|(name, data)| NetworkInterfaces {
                name: name.clone(),
                errors_on_received: data.errors_on_received(),
                total_errors_on_received: data.total_errors_on_received(),
                errors_on_transmitted: data.errors_on_transmitted(),
                total_errors_on_transmitted: data.total_errors_on_transmitted(),
                packets_received: data.packets_received(),
                total_packets_received: data.total_packets_received(),
                packets_transmitted: data.packets_transmitted(),
                total_packets_transmitted: data.total_packets_transmitted(),
                bytes_received: data.received(),
                total_bytes_received: data.total_received(),
                bytes_transmitted: data.transmitted(),
                total_bytes_transmitted: data.total_transmitted(),
            })
            .collect();

        Self {
            timestamp,
            interfaces,
        }
    }
}
