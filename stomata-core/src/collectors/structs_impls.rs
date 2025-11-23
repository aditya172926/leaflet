use sysinfo::{Pid, ProcessRefreshKind};

use crate::collectors::structs::{MetricsCategory, StomataSystemMetrics};

impl StomataSystemMetrics {
    pub fn refresh_metrics(&mut self, refresh_category: MetricsCategory) {
        match refresh_category {
            MetricsCategory::ProcessesWithoutTasks => {
                let _processes_updated = self.system.refresh_processes_specifics(
                    sysinfo::ProcessesToUpdate::All,
                    true,
                    ProcessRefreshKind::everything().without_tasks(),
                );
            }
            MetricsCategory::Processes => {
                let _processes_updated = self.system.refresh_processes_specifics(
                    sysinfo::ProcessesToUpdate::All,
                    true,
                    ProcessRefreshKind::everything(),
                );
            }
            MetricsCategory::ProcessWithPid(pid) => {
                self.system.refresh_processes(
                    sysinfo::ProcessesToUpdate::Some(&[Pid::from_u32(pid)]),
                    true,
                );
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
            MetricsCategory::Networks => {
                self.network.refresh(true);
            }
        }
    }
}
