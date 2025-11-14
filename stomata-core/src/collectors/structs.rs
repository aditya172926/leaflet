use anyhow::Result;
use chrono::{DateTime, Utc};
use sysinfo::{Process, ProcessRefreshKind, System};

pub enum MetricsCategory {
    ProcessesWithoutTasks, // refreshes processes but not tasks
    Processes,             // refreshes processes with tasks
    Memory,
    CPU,
    AllResources, // refreshes everything
    Basic,        // refreshes CPU + Memory usage
}

#[derive(Debug, Default)]
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

#[derive(Debug, Clone)]
pub struct ProcessData {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub status: String,
}

impl From<&Process> for ProcessData {
    fn from(process: &Process) -> Self {
        ProcessData {
            pid: process.pid().as_u32(),
            name: process.name().to_string_lossy().to_string(),
            cpu_usage: process.cpu_usage(),
            memory: process.memory(),
            status: process.status().to_string(),
        }
    }
}

#[derive(Debug)]
pub struct SystemCollector {
    system: System,
}

impl Default for SystemCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemCollector {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self { system }
    }

    pub fn collect(&mut self, refresh_kind: MetricsCategory) -> Result<SystemMetrics> {
        let mut processes: Vec<ProcessData> = Vec::new();
        match refresh_kind {
            MetricsCategory::ProcessesWithoutTasks => {
                let processes_updated = self.system.refresh_processes_specifics(
                    sysinfo::ProcessesToUpdate::All,
                    true,
                    ProcessRefreshKind::everything().without_tasks(),
                );
                if processes_updated > 0 {
                    processes = self.get_running_processes();
                }
            }
            MetricsCategory::Processes => {
                let processes_updated = self.system.refresh_processes_specifics(
                    sysinfo::ProcessesToUpdate::All,
                    true,
                    ProcessRefreshKind::everything(),
                );
                if processes_updated > 0 {
                    processes = self.get_running_processes();
                }
            }
            MetricsCategory::CPU => {
                self.system.refresh_cpu_usage();
            }
            MetricsCategory::Memory => {
                self.system.refresh_memory(); // includes swap too
            }
            MetricsCategory::AllResources => {
                self.system.refresh_all();
            }
            MetricsCategory::Basic => {
                self.system.refresh_memory();
                self.system.refresh_cpu_usage();
            }
        };
        let cpu_count = self.system.cpus().len();
        let cpu_usage = self.system.global_cpu_usage();
        let memory_used = self.system.used_memory();
        let memory_total = self.system.total_memory();
        let swap_used = self.system.used_swap();
        let swap_total = self.system.total_swap();
        let processes_count = self.system.processes().len();

        Ok(SystemMetrics {
            timestamp: Utc::now(),
            cpu_count,
            cpu_usage,
            memory_used,
            memory_total,
            swap_used,
            swap_total,
            processes_count,
            processes,
        })
    }

    pub fn system_info(&self) -> SystemInfo {
        SystemInfo {
            os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
            hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        }
    }

    pub fn get_running_processes(&self) -> Vec<ProcessData> {
        let processes: Vec<ProcessData> = self
            .system
            .processes()
            .values()
            .map(ProcessData::from)
            .collect();
        return processes;
    }
}
