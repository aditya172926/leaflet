use chrono::{DateTime, Utc};
use std::collections::VecDeque;
use sysinfo::{DiskUsage, Pid, Process, System};

use crate::collectors::system::metrics::SystemMetrics;

pub enum MetricsCategory {
    ProcessesWithoutTasks, // refreshes processes but not tasks
    Processes,             // refreshes all processes with tasks
    ProcessWithPid(u32),
    Memory,
    CPU,
    AllResources, // refreshes everything
    Basic,        // refreshes CPU + Memory usage
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub hostname: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessData {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub status: String,
}

pub struct SingleProcessData<'a> {
    pub basic_process_data: ProcessData,
    pub tasks: Vec<&'a Process>,
    pub disk_usage: DiskUsage,
    pub start_time: u64,
    pub running_time: u64,
    pub current_working_dir: Option<String>,
    pub parent_pid: Option<Pid>,
}

#[derive(Debug)]
pub struct SystemCollector {
    pub system: System,
    pub system_metrics: MetricsHistory,
}

#[derive(Debug)]
pub enum MetricsHistory {
    Single(SystemMetrics),
    History(VecDeque<SystemMetrics>),
}
