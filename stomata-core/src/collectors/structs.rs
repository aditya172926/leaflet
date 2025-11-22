use std::collections::VecDeque;
use sysinfo::System;

use crate::collectors::{SystemInfo, process::metrics::{ProcessData, SingleProcessData}, system::metrics::{SystemCollector, SystemMetrics}};

pub struct StomataSystemMetrics<'a> {
    system: System,
    pub system_info: SystemInfo,
    pub system_metrics: SystemCollector,
    pub process_metrics: ProcessData,
    pub single_process_data_metrics: SingleProcessData<'a>
}

impl<'a> StomataSystemMetrics<'a> {
    pub fn new() -> Self {
        let system = System::new_all();

        Self {
            system,
            system_info: SystemInfo::new(),
            system_metrics: SystemCollector::new(false),
            process_metrics: ProcessData::default(),
            single_process_data_metrics: SingleProcessData::default()
        }
    }
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
