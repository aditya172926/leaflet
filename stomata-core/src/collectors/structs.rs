use std::collections::VecDeque;
use sysinfo::System;

use crate::collectors::{
    SystemInfo,
    process::metrics::{ProcessData, SingleProcessData},
    system::metrics::{SystemCollector, SystemMetrics},
};

#[derive(Debug)]
pub struct StomataSystemMetrics {
    pub system: System,
}

impl StomataSystemMetrics {
    pub fn new() -> Self {
        let system = System::new_all();

        Self { system }
    }

    pub fn fetch(&mut self, fetch_metrics: MetricsToFetch) -> Metrics<'_> {
        match fetch_metrics {
            MetricsToFetch::SystemInfo => Metrics::SystemInfo(SystemInfo::new()),
            MetricsToFetch::SystemResource => Metrics::SystemResource(SystemCollector::fetch(
                MetricsCategory::Basic,
                &mut self.system,
            )),
            MetricsToFetch::Process => Metrics::Processes(ProcessData::fetch(&self.system)),
            MetricsToFetch::SingleProcessPid(pid) => {
                Metrics::SingleProcessPid(SingleProcessData::fetch(&mut self.system, pid))
            }
        }
    }
}

pub enum MetricsToFetch {
    SystemInfo,
    SystemResource,
    Process,
    SingleProcessPid(u32),
}

pub enum Metrics<'a> {
    SystemInfo(SystemInfo),
    SystemResource(SystemCollector),
    Processes(Vec<ProcessData>),
    SingleProcessPid(Option<SingleProcessData<'a>>),
}

pub enum MetricsCategory {
    ProcessesWithoutTasks, // refreshes processes but not tasks
    Processes,             // refreshes all processes with tasks
    ProcessWithPid(u32),
    Memory,
    CPU,
    AllResources, // refreshes everything
    Basic,        // refreshes CPU + Memory usage
}

#[derive(Debug)]
pub enum MetricsHistory {
    Single(SystemMetrics),
    History(VecDeque<SystemMetrics>),
}
