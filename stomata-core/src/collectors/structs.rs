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

impl MetricsCategory {
    pub fn refresh_metrics(&self, system: &mut System) {
        match self {
            MetricsCategory::ProcessesWithoutTasks => {
                let processes_updated = system.refresh_processes_specifics(
                    sysinfo::ProcessesToUpdate::All,
                    true,
                    ProcessRefreshKind::everything().without_tasks(),
                );
            }
            MetricsCategory::Processes => {
                let processes_updated = system.refresh_processes_specifics(
                    sysinfo::ProcessesToUpdate::All,
                    true,
                    ProcessRefreshKind::everything(),
                );
            }
            MetricsCategory::ProcessWithPid(pid) => {
                system.refresh_processes(
                    sysinfo::ProcessesToUpdate::Some(&[Pid::from_u32(*pid)]),
                    true,
                );
            }
            MetricsCategory::CPU => {
                system.refresh_cpu_usage();
            }
            MetricsCategory::Memory => {
                system.refresh_memory(); // includes swap too
            }
            MetricsCategory::AllResources => {
                system.refresh_all();
            }
            MetricsCategory::Basic => {
                system.refresh_memory();
                system.refresh_cpu_usage();
            }
        }
    }
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

#[derive(Debug, Clone, PartialEq)]
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

pub struct SingleProcessData<'a> {
    pub basic_process_data: ProcessData,
    pub tasks: Vec<&'a Process>,
    pub disk_usage: DiskUsage,
    pub start_time: u64,
    pub running_time: u64,
    pub current_working_dir: Option<String>,
    pub parent_pid: Option<Pid>,
}

impl<'a> From<(&'a Process, &'a System)> for SingleProcessData<'a> {
    fn from((process, system): (&'a Process, &'a System)) -> Self {
        let tasks = if let Some(task_pids) = process.tasks() {
            task_pids
                .iter()
                .filter_map(|p| system.process(*p))
                .collect()
        } else {
            Vec::new()
        };

        let disk_usage = process.disk_usage();
        let current_working_dir = if let Some(cwd) = process.cwd() {
            Some(cwd.to_string_lossy().to_string())
        } else {
            None
        };
        let start_time = process.start_time();
        let running_time = process.run_time();
        let parent_pid = process.parent();

        SingleProcessData {
            basic_process_data: ProcessData::from(process),
            tasks: tasks,
            disk_usage,
            start_time,
            running_time,
            current_working_dir,
            parent_pid,
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
        refresh_kind.refresh_metrics(&mut self.system);

        match refresh_kind {
            MetricsCategory::ProcessesWithoutTasks => {
                processes = self.get_running_processes();
            }
            MetricsCategory::Processes => {
                processes = self.get_running_processes();
            }
            _ => {}
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

    pub fn get_process_for_pid(&mut self, pid: u32) -> Option<SingleProcessData> {
        MetricsCategory::ProcessWithPid(pid).refresh_metrics(&mut self.system);
        if let Some(process) = self.system.process(Pid::from_u32(pid)) {
            Some(SingleProcessData::from((process, &self.system)))
        } else {
            None
        }
    }
}
