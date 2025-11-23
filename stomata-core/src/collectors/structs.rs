use std::collections::VecDeque;
use sysinfo::{Networks, System};

use crate::collectors::{
    SystemInfo,
    network::metrics::NetworkMetrics,
    process::metrics::{ProcessData, SingleProcessData},
    system::metrics::{SystemCollector, SystemMetrics},
};

#[derive(Debug)]
pub struct StomataSystemMetrics {
    pub system: System,
    pub network: Networks,
}

impl StomataSystemMetrics {
    pub fn new() -> Self {
        let system = System::new_all();
        let network = Networks::new();
        Self { system, network }
    }

    pub fn fetch(&mut self, fetch_metrics: MetricsToFetch) -> Metrics<'_> {
        match fetch_metrics {
            MetricsToFetch::SystemInfo => Metrics::SystemInfo(SystemInfo::new()),
            MetricsToFetch::SystemResource => {
                self.refresh_metrics(MetricsCategory::Basic);
                Metrics::SystemResource(SystemCollector::fetch(&mut self.system))
            }
            MetricsToFetch::Process => {
                self.refresh_metrics(MetricsCategory::ProcessesWithoutTasks);
                Metrics::Processes(ProcessData::fetch(&self.system))
            }
            MetricsToFetch::SingleProcessPid(pid) => {
                self.refresh_metrics(MetricsCategory::ProcessWithPid(pid));
                Metrics::SingleProcessPid(SingleProcessData::fetch(&mut self.system, pid))
            }
            MetricsToFetch::Networks => {
                self.refresh_metrics(MetricsCategory::Networks);
                Metrics::Networks(NetworkMetrics::fetch(&self.network))
            }
        }
    }
}

pub enum MetricsToFetch {
    SystemInfo,
    SystemResource,
    Process,
    SingleProcessPid(u32),
    Networks,
}

// Response metrics
pub enum Metrics<'a> {
    SystemInfo(SystemInfo),
    SystemResource(SystemCollector),
    Processes(Vec<ProcessData>),
    SingleProcessPid(Option<SingleProcessData<'a>>),
    Networks(NetworkMetrics),
}

pub enum MetricsCategory {
    ProcessesWithoutTasks, // refreshes processes but not tasks
    Processes,             // refreshes all processes with tasks
    ProcessWithPid(u32),
    Memory,
    CPU,
    AllResources, // refreshes everything
    Basic,        // refreshes CPU + Memory usage
    Networks,
}

#[derive(Debug)]
pub enum MetricsHistory {
    Single(SystemMetrics),
    History(VecDeque<SystemMetrics>),
}
