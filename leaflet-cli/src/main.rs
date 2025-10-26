use std::{collections::VecDeque, thread::sleep, time::Duration};

use anyhow::Result;
use clap::Parser;
use constants::MAX_HISTORY;
use leaflet_core::collectors::structs::{SystemCollector, SystemInfo, SystemMetrics};

use crate::structs::Cli;

mod constants;
mod structs;

#[derive(Debug)]
struct App {
    metrics_history: VecDeque<SystemMetrics>,
    system_info: leaflet_core::collectors::structs::SystemInfo,
}

impl App {
    fn new(system_info: SystemInfo) -> Self {
        Self {
            metrics_history: VecDeque::with_capacity(MAX_HISTORY),
            system_info,
        }
    }

    fn update_metrics(&mut self, metrics: SystemMetrics) {
        if self.metrics_history.len() >= MAX_HISTORY {
            self.metrics_history.pop_front();
        }
        self.metrics_history.push_back(metrics);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut collector = SystemCollector::new();
    let system_info = collector.system_info();

    let mut app = App::new(system_info);
    println!("System Information: {:?}", app.system_info);

    let refresh_interval = cli.interval;
    loop {
        match collector.collect() {
            Ok(collected_metrics) => {
                app.update_metrics(collected_metrics);
            }
            Err(e) => {
                eprintln!("Error in collecting machine metrics {:?}", e);
            }
        };

        println!(
            "\nThe current metrics history for this machine is {:?}",
            app.metrics_history
        );
        sleep(Duration::from_millis(refresh_interval));
    }
    Ok(())
}
