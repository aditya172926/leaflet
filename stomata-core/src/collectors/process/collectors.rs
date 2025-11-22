use sysinfo::{Pid, Process, System};

use crate::collectors::{process::metrics::{ProcessData, SingleProcessData}, structs::MetricsCategory};

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

impl ProcessData {
    pub fn fetch(system: &System) -> Vec<Self> {
        let processes: Vec<ProcessData> = system
            .processes()
            .values()
            .map(ProcessData::from)
            .collect();
        return processes;
    }
}

// Single Process
impl<'a> From<(&'a Process, Vec<&'a Process>)> for SingleProcessData<'a> {
    fn from((process, tasks): (&'a Process, Vec<&'a Process>)) -> Self {
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

impl SingleProcessData<'_> {
    pub fn fetch(system: &mut System, pid: u32) -> Option<SingleProcessData<'_>> {
        MetricsCategory::ProcessWithPid(pid).refresh_metrics(system);
        if let Some(process) = system.process(Pid::from_u32(pid)) {
            let tasks = if let Some(task_pids) = process.tasks() {
                task_pids
                    .iter()
                    .filter_map(|p| system.process(*p))
                    .collect()
            } else {
                Vec::new()
            };

            let single_process_data = SingleProcessData::from((process, tasks));
            Some(single_process_data)
        } else {
            None
        }
    }
}