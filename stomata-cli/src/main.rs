use std::{collections::VecDeque, thread::sleep, time::Duration};

use clap::Parser;
use constants::MAX_HISTORY;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
};
use stomata_core::collectors::structs::{SystemCollector, SystemInfo, SystemMetrics};

use crate::{
    renders::{
        render_bar::vertical_bar_chart,
        render_gauge::{self, render_gauge},
        render_paragraph::paragraph_widget,
    },
    structs::Cli,
    utils::bytes_to_mb,
};

mod constants;
mod renders;
mod structs;
mod utils;

#[derive(Debug)]
struct App {
    render: bool,
    metrics_history: VecDeque<SystemMetrics>,
    system_info: stomata_core::collectors::structs::SystemInfo,
    // selected_tab: 
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

    fn draw_chart(
        &mut self,
        mut terminal: DefaultTerminal,
        refresh_interval: u64,
        mut collector: SystemCollector,
    ) -> anyhow::Result<()> {
        while self.render {
            match collector.collect() {
                Ok(collected_metrics) => {
                    self.update_metrics(collected_metrics);
                }
                Err(e) => {
                    eprintln!("Error collecting metrics: {:?}", e);
                    continue;
                }
            };

            let latest_metric = match self.get_latest_metric() {
                Some(metric) => metric,
                None => {
                    eprintln!("No metrics available yet.");
                    continue;
                }
            };

            terminal.draw(|frame| {
                let layout = Layout::vertical([
                    Constraint::Percentage(23),
                    Constraint::Percentage(23),
                    Constraint::Percentage(24),
                    Constraint::Percentage(30),
                ])
                .split(frame.area());

                let layout_paragraph =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(layout[3]);

                // render memory usage gauge
                frame.render_widget(
                    render_gauge(
                        bytes_to_mb(latest_metric.memory_used),
                        bytes_to_mb(latest_metric.memory_total),
                        "Memory Usage",
                        "MB",
                    ),
                    layout[0],
                );

                // render swap usage gauge
                frame.render_widget(
                    render_gauge(
                        bytes_to_mb(latest_metric.swap_used),
                        bytes_to_mb(latest_metric.swap_total),
                        "Swap Usage",
                        "MB",
                    ),
                    layout[1],
                );

                // render cpu usage gauge
                frame.render_widget(
                    render_gauge(latest_metric.cpu_usage as f64, 100.0, "CPU Usage", "%"),
                    layout[2],
                );

                // --- PARAGRAPH ---
                let memory_used =
                    latest_metric.memory_used as f64 / latest_metric.memory_total as f64 * 100.0;

                let swap_used =
                    latest_metric.swap_used as f64 / latest_metric.swap_total as f64 * 100.0;

                let text = format!(
                    "Memory Used: {:.2} Bytes\nTotal Memory: {:.2} Bytes\nUsage: {:.2}%",
                    latest_metric.memory_used, latest_metric.memory_total, memory_used,
                );

                let text_swap = format!(
                    "Swap Used: {:.2} Bytes\nTotal Swap: {:.2} Bytes\nUsage: {:.2}%",
                    latest_metric.swap_used, latest_metric.swap_total, swap_used,
                );

                let paragraph = paragraph_widget(&text, "Memory Info");
                let swap_paragraph = paragraph_widget(&text_swap, "Swap Info");
                frame.render_widget(paragraph, layout_paragraph[0]);
                frame.render_widget(swap_paragraph, layout_paragraph[1]);
            })?;
            self.handle_events()?;

            sleep(Duration::from_millis(refresh_interval));
        }
        ratatui::restore();
        Ok(())
    }

    // handle quit events to closet= the new terminal
    fn handle_events(&mut self) -> anyhow::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                self.render = false;
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // initialize the system collector from stomata-core
    let collector = SystemCollector::new();
    let system_info = collector.system_info();

    let mut app = App::new(system_info);
    let terminal = ratatui::init();

    // get the refresh interval from the cli arg. Default 1000 ms
    let refresh_interval = cli.interval;
    let _ = app.draw_chart(terminal, refresh_interval, collector);
}
