use std::collections::VecDeque;

use anyhow::Result;
use chrono::{DateTime, Utc};
use sysinfo::{DiskUsage, Pid, Process, ProcessRefreshKind, System};

pub enum MetricsCategory {
    ProcessesWithoutTasks, // refreshes processes but not tasks
    Processes,             // refreshes all processes with tasks
    ProcessWithPid(u32),
    Memory,
    CPU,
    AllResources, // refreshes everything
    Basic,        // refreshes CPU + Memory usage
}

#[derive(Debug, Default, Clone)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu_count: usize,
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub swap_used: u64,
    pub swap_total: u64,
    pub processes_count: usize,
    pub processes: Vec<ProcessData>,
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
    pub disk_read_usage: VecDeque<u64>, // record of bytes read since last refresh
    pub disk_write_usage: VecDeque<u64>, // record of bytes written since last refresh
    pub start_time: u64,
    pub running_time: u64,
    pub current_working_dir: Option<String>,
    pub parent_pid: Option<Pid>,
}

#[derive(Debug)]
pub struct SystemCollector {
    pub system: System,
}