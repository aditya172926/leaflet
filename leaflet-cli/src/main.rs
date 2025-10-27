use std::{collections::VecDeque, thread::sleep, time::Duration};

use clap::Parser;
use constants::MAX_HISTORY;
use leaflet_core::collectors::structs::{SystemCollector, SystemInfo, SystemMetrics};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    render::vertical_bar_chart,
    structs::Cli,
};

mod constants;
mod render;
mod structs;

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

    fn draw_bar_chart(
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
            let memory_used =
                vec![latest_metric.memory_used as f32 / latest_metric.memory_total as f32 * 100.0];

            terminal.draw(|frame| {
                let layout =
                    Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
                        .split(frame.area());

                frame.render_widget(vertical_bar_chart(&memory_used), layout[0]);

                // --- PARAGRAPH ---
                let text = format!(
                    "Memory Used: {:.2} MB\nTotal Memory: {:.2} MB\nUsage: {:.2}%",
                    latest_metric.memory_used, latest_metric.memory_total, memory_used[0],
                );
                let paragraph = Paragraph::new(text)
                    .block(Block::default().borders(Borders::ALL).title("System Info"));
                frame.render_widget(paragraph, layout[1]);
            })?;
            self.handle_events()?;

            sleep(Duration::from_millis(refresh_interval));
        }
        ratatui::restore();
        Ok(())
    }

    fn handle_events(&mut self) -> anyhow::Result<()> {
        if event::poll(Duration::from_millis(1000))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.render = false;
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let collector = SystemCollector::new();
    let system_info = collector.system_info();

    let mut app = App::new(system_info);
    let terminal = ratatui::init();

    let refresh_interval = cli.interval;
    let _ = app.draw_bar_chart(terminal, refresh_interval, collector);
}
