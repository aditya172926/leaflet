use std::collections::{HashMap, VecDeque};

use clap::Parser;
use ratatui::{
    Frame, layout::Constraint, widgets::{Cell, TableState}
};
use stomata_core::collectors::{
    network::metrics::NetworkInterfaces, process::metrics::SingleProcessData,
};
use sysinfo::DiskUsage;

use crate::constants::{CLAMP_TREND_VALUE, MAX_HISTORY_IN_MEMORY, MAX_NETWORK_IN_MEMORY};

#[derive(Debug, Clone, Copy)]
pub enum Feature {
    #[cfg(feature = "core")]
    Core,
    #[cfg(feature = "web3")]
    Web3
}

pub enum AppState {
    FeatureSelection,
    RunningFeature(Feature)
}
pub struct StomataState {
    pub state: AppState,
    pub selected_feature: usize,
    pub available_features: Vec<Feature>
}

#[derive(Parser, Debug)]
#[command(name = "stomata")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = 1000)]
    pub interval: u64,
    #[arg(short, long, default_value_t = false)]
    pub store: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    System,
    Metrics,
    Processes,
    SingleProcess(u32), // pid
    Network,
}

impl Page {
    pub fn titles() -> Vec<&'static str> {
        vec!["System", "Metrics", "Processes", "Network"]
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Page::System,
            1 => Page::Metrics,
            2 => Page::Processes,
            3 => Page::Network,
            _ => Page::System,
        }
    }
}

// Trait that any type must implement to be displayable in a table
pub trait TableRow {
    fn to_cells(&self) -> Vec<Cell<'_>>;
    fn column_widths() -> Vec<Constraint>;
}

#[derive(Debug)]
pub struct UIState {
    pub process_table: ProcessesUIState,
    pub single_process_disk_usage: SingleProcessDiskUsage,
    pub networks_state: Option<HashMap<String, NetworkInterfaceData>>,
}

#[derive(Debug)]
pub struct ProcessesUIState {
    pub process_list: TableState,
    pub process_count: usize,
    pub selected_pid: Option<u32>,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            process_table: ProcessesUIState {
                process_list: TableState::default().with_selected(0),
                process_count: 0,
                selected_pid: None,
            },
            single_process_disk_usage: SingleProcessDiskUsage::default(),
            networks_state: None,
        }
    }
}

pub struct SingleProcessUI<'a> {
    pub data: SingleProcessData<'a>,
}

#[derive(Debug)]
pub struct SingleProcessDiskUsage {
    pub pid: u32,
    pub disk_read_usage: VecDeque<u64>,
    pub disk_write_usage: VecDeque<u64>,
}

impl Default for SingleProcessDiskUsage {
    fn default() -> Self {
        Self {
            pid: 0,
            disk_read_usage: VecDeque::<u64>::with_capacity(MAX_HISTORY_IN_MEMORY),
            disk_write_usage: VecDeque::<u64>::with_capacity(MAX_HISTORY_IN_MEMORY),
        }
    }
}

impl SingleProcessDiskUsage {
    pub fn update_disk_history(&mut self, pid: u32, disk_usage: &DiskUsage) {
        // reset the UI state data for disk write/read when changed at current displaying pid
        if pid != self.pid {
            self.disk_read_usage.clear();
            self.disk_write_usage.clear();
            self.pid = pid;
        }

        if self.disk_read_usage.len() > 60 {
            self.disk_read_usage.pop_front();
        }
        self.disk_read_usage.push_back(disk_usage.read_bytes);

        if self.disk_write_usage.len() > 60 {
            self.disk_write_usage.pop_front();
        }
        self.disk_write_usage.push_back(disk_usage.written_bytes);
    }
}

#[derive(Debug)]
pub struct NetworkInterfaceData {
    pub received_bytes: Ring<u64, MAX_NETWORK_IN_MEMORY>,
    pub transmitted_bytes: Ring<u64, MAX_NETWORK_IN_MEMORY>,
    pub packets_received: Ring<u64, MAX_NETWORK_IN_MEMORY>,
    pub packets_transmitted: Ring<u64, MAX_NETWORK_IN_MEMORY>,
    pub errors_received: Ring<u64, MAX_NETWORK_IN_MEMORY>,
    pub errors_transmitted: Ring<u64, MAX_NETWORK_IN_MEMORY>,
}

impl Default for NetworkInterfaceData {
    fn default() -> Self {
        Self {
            received_bytes: Ring::new(),
            transmitted_bytes: Ring::new(),
            packets_received: Ring::new(),
            packets_transmitted: Ring::new(),
            errors_received: Ring::new(),
            errors_transmitted: Ring::new(),
        }
    }
}

impl NetworkInterfaceData {
    pub fn update_network_history(&mut self, network_data: &NetworkInterfaces) {
        self.received_bytes
            .push_clamped(network_data.bytes_received);
        self.transmitted_bytes
            .push_clamped(network_data.bytes_transmitted);
        self.packets_received
            .push_clamped(network_data.packets_received);
        self.packets_transmitted
            .push_clamped(network_data.packets_transmitted);
        self.errors_received
            .push_clamped(network_data.errors_on_received);
        self.errors_transmitted
            .push_clamped(network_data.errors_on_transmitted);
    }
}

#[derive(Debug)]
pub struct Ring<T, const N: usize> {
    inner: VecDeque<T>,
}

impl<T, const N: usize> Ring<T, N> {
    pub fn new() -> Self {
        Self {
            inner: VecDeque::with_capacity(N),
        }
    }

    pub fn push(&mut self, value: T) {
        if self.inner.len() == N {
            self.inner.pop_front();
        }
        self.inner.push_back(value);
    }

    pub fn make_contiguous(&mut self) -> &mut [T] {
        self.inner.make_contiguous()
    }
}

impl<T, const N: usize> Ring<T, N>
where
    T: Copy + Ord + From<u8>,
{
    pub fn push_clamped(&mut self, value: T) {
        if self.inner.is_empty() {
            self.push(value);
            return;
        }

        // // count non-zero values
        // let non_zero = self.inner.iter().filter(|v| **v > T::from(0)).count();

        // // case 1: interface idle or warming up → log real spike
        // if non_zero < 3 {
        //     return self.push(value);
        // }

        // collect historical values
        let mut data: Vec<T> = self.inner.iter().copied().collect();
        data.push(value);

        // compute percentile index
        let p_index = ((data.len() - 1) as f64 * CLAMP_TREND_VALUE).round() as usize;

        // nth_element selection
        let (_, p_val, _) = data.select_nth_unstable(p_index);

        // clamp
        let clamped = if value > *p_val { *p_val } else { value };

        self.push(clamped);
    }
}
