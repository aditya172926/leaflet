use std::collections::VecDeque;

use clap::Parser;
use ratatui::{layout::Constraint, widgets::Cell};
use stomata_core::collectors::structs::SystemMetrics;

#[derive(Parser, Debug)]
#[command(name = "stomata")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = 1000)]
    pub interval: u64,
    #[arg(short, long, default_value_t = false)]
    pub store: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Page {
    System,
    Metrics,
    Processes,
}

impl Page {
    pub fn titles() -> Vec<&'static str> {
        vec!["System", "Metrics", "Processes"]
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Page::System,
            1 => Page::Metrics,
            2 => Page::Processes,
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
pub enum MetricsStorage {
    Single(SystemMetrics),
    History(VecDeque<SystemMetrics>),
}
