use std::{collections::VecDeque, thread::sleep, time::Duration};

use clap::Parser;
use constants::MAX_HISTORY;
use leaflet_core::collectors::structs::{SystemCollector, SystemInfo, SystemMetrics};
use ratatui::{crossterm::event::{self, Event, KeyEventKind}, DefaultTerminal};

use crate::{render::render_bar, structs::Cli};

mod constants;
mod structs;
mod render;

#[derive(Debug)]
struct App {
    render: bool,
    metrics_history: VecDeque<SystemMetrics>,
    system_info: leaflet_core::collectors::structs::SystemInfo,
}

impl App {
    fn new(system_info: SystemInfo) -> Self {
        Self {
            render: true,
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

    fn get_latest_metric(&self) -> Option<&SystemMetrics> {
        self.metrics_history.back()
    }

    fn draw_bar_chart(mut self, mut terminal: DefaultTerminal, refresh_interval: u64, mut collector: SystemCollector) -> anyhow::Result<()> {
        loop {
            match collector.collect() {
                Ok(collected_metrics) => {
                    self.update_metrics(collected_metrics);
                }
                Err(e) => {
                    eprintln!("Error in collecting machine metrics {:?}", e);
                }
            };
    
            println!(
                "\nThe current metrics history for this machine is {:?}",
                self.metrics_history
            );

            let latest_metric = match self.get_latest_metric() {
                Some(metric) => {
                    vec![metric.memory_used / metric.memory_total * 100]
                },
                None => vec![0],
            };
            terminal.draw(|frame| render_bar(frame, &latest_metric))?;
            self.handle_events()?;

            sleep(Duration::from_millis(refresh_interval));
        }
        Ok(())
    }

    fn handle_events(&mut self) -> anyhow::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == event::KeyCode::Char('q') {
                ratatui::restore();
                self.render = false;
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let mut collector = SystemCollector::new();
    let system_info = collector.system_info();

    let mut app = App::new(system_info);
    let terminal = ratatui::init();
    println!("System Information: {:?}", app.system_info);

    let refresh_interval = cli.interval;
    app.draw_bar_chart(terminal, refresh_interval, collector);
}
