use anyhow::Result;
use chrono::Utc;
use std::collections::VecDeque;
use sysinfo::{Pid, Process, ProcessRefreshKind, System};

use crate::{
    collectors::{structs::{
        MetricsCategory, MetricsHistory, ProcessData, SingleProcessData,
        SystemInfo,
    }, system::collectors::SystemCollector},
    constants::MAX_HISTORY,
};

impl MetricsCategory {
    pub fn refresh_metrics(&self, system: &mut System) {
        match self {
            MetricsCategory::ProcessesWithoutTasks => {
                let _processes_updated = system.refresh_processes_specifics(
                    sysinfo::ProcessesToUpdate::All,
                    true,
                    ProcessRefreshKind::everything().without_tasks(),
                );
            }
            MetricsCategory::Processes => {
                let _processes_updated = system.refresh_processes_specifics(
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

impl SystemCollector {
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

    pub fn get_process_for_pid(&mut self, pid: u32) -> Option<SingleProcessData<'_>> {
        MetricsCategory::ProcessWithPid(pid).refresh_metrics(&mut self.system);
        if let Some(process) = self.system.process(Pid::from_u32(pid)) {
            let single_process_data = SingleProcessData::from((process, &self.system));
            Some(single_process_data)
        } else {
            None
        }
    }
}
