use std::{collections::VecDeque, thread::sleep, time::Duration};

use clap::Parser;
use constants::MAX_HISTORY;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Tabs},
};
use stomata_core::collectors::structs::{SystemCollector, SystemInfo, SystemMetrics};

use crate::{
    renders::{
        render_bar::vertical_bar_chart,
        render_gauge::{self, render_gauge},
        render_paragraph::paragraph_widget,
    },
    structs::{Cli, Page},
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
    metrics_collector: SystemCollector,
    tab_index: usize,
    current_page: Page,
}

impl App {
    fn new() -> Self {
        let collector = SystemCollector::new();
        let system_info = collector.system_info();
        Self {
            render: true,
            metrics_history: VecDeque::with_capacity(MAX_HISTORY),
            system_info,
            metrics_collector: collector,
            tab_index: 0,
            current_page: Page::System,
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

    // go to the next tab
    fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % Page::titles().len();
        self.current_page = Page::from_index(self.tab_index);
    }

    // go to the previous tab
    fn previous_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = Page::titles().len() - 1;
        }
        self.current_page = Page::from_index(self.tab_index);
    }

    // render according to the tab selected
    fn render(&mut self, frame: &mut Frame) {
        let chunks =
            Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());

        // render tabs
        self.render_tabs(frame, chunks[0]);

        match self.current_page {
            Page::Metrics => {
                self.draw_chart(frame, chunks[1]);
            }
            Page::System => {}
        }
    }

    // render tabs
    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let titles: Vec<Line> = Page::titles().iter().map(|t| Line::from(*t)).collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Stomata"))
            .select(self.tab_index)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_widget(tabs, area);
    }

    fn draw_chart(&mut self, frame: &mut Frame, area: Rect) -> anyhow::Result<()> {
        match self.metrics_collector.collect() {
            Ok(collected_metrics) => {
                self.update_metrics(collected_metrics);
            }
            Err(e) => {
                eprintln!("Error collecting metrics: {:?}", e);
            }
        };

        let latest_metric = match self.get_latest_metric() {
            Some(metric) => metric,
            None => {
                eprintln!("No metrics available yet.");
                &SystemMetrics::default()
            }
        };

        let layout = Layout::vertical([
            Constraint::Percentage(23),
            Constraint::Percentage(23),
            Constraint::Percentage(24),
            Constraint::Percentage(30),
        ])
        .split(area);

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

        let swap_used = latest_metric.swap_used as f64 / latest_metric.swap_total as f64 * 100.0;

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

        Ok(())
    }

    // handle quit events to close the new terminal
    fn handle_events(&mut self) -> anyhow::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            self.render = false;
                            ratatui::restore();
                        }
                        KeyCode::Right | KeyCode::Tab => {
                            self.next_tab();
                        }
                        KeyCode::Left => {
                            self.previous_tab();
                        }
                        KeyCode::Char('1') => {
                            self.tab_index = 0;
                            self.current_page = Page::System;
                        }
                        KeyCode::Char('2') => {
                            self.tab_index = 1;
                            self.current_page = Page::Metrics;
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut app = App::new();
    let mut terminal = ratatui::init();

    // get the refresh interval from the cli arg. Default 1000 ms
    let refresh_interval = cli.interval;

    // main render loop
    while app.render {
        // draw
        terminal.draw(|frame| app.render(frame))?;

        // handle events
        app.handle_events()?;

        // sleep for refresh interval
        sleep(Duration::from_millis(refresh_interval));
    }

    Ok(())
}
