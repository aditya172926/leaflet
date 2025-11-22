use sysinfo::{Pid, ProcessRefreshKind, System};

use crate::collectors::structs::MetricsCategory;

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
