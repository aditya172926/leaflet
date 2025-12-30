//! Core application state and rendering logic
//!
//! This module contains the main `App` struct that manages the entire
//! application state, handles user input, and coordinates rendering of
//! different pages in the TUI.

use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Tabs},
};
use stomata_core::collectors::structs::{Metrics, MetricsToFetch, StomataSystemMetrics};

use crate::{
    renders::core_displays::traits::{Display, SingleProcessDisplay},
    structs::{Page, SingleProcessUI, UIState},
    utils::bytes_to_mb,
};

/// Main application state manager
///
/// Manages the entire application lifecycle including:
/// - System metrics collection and display
/// - Tab navigation and page routing
/// - User input handling
/// - UI state management across different pages
#[derive(Debug)]
pub struct App {
    /// Whether the application should continue rendering
    pub render: bool,

    /// System metrics collector and storage
    pub metrics: StomataSystemMetrics,

    /// Index of the currently selected tab (0-based)
    pub tab_index: usize,

    /// The currently active page being displayed
    pub current_page: Page,

    /// Whether to store historical metrics data
    pub store_data: bool,

    /// UI state for stateful widgets (tables, lists, charts)
    pub ui_state: UIState,
}

impl App {
    /// Creates a new application instance
    ///
    /// Initializes the app with default values and prepares the metrics
    /// collection system. The app starts on the System page with rendering enabled.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stomata::renders::core_displays::display_app::App;
    ///
    /// // Create app without metrics storage (lower memory usage)
    /// let app = App::new(false);
    ///
    /// // Create app with metrics storage (enables historical charts)
    /// let app_with_history = App::new(true);
    /// ```
    pub fn new(store_metrics: bool) -> Self {
        Self {
            render: true,
            metrics: StomataSystemMetrics::new(),
            tab_index: 0,
            current_page: Page::System,
            store_data: store_metrics, // by default don't store history data
            ui_state: UIState::default(),
        }
    }

    /// Advances to the next tab, wrapping to the first tab after the last
    ///
    /// Updates both `tab_index` and `current_page` to maintain consistency.
    pub fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % Page::titles().len();
        self.current_page = Page::from_index(self.tab_index);
    }

    /// Moves to the previous tab, wrapping to the last tab before the first
    ///
    /// Updates both `tab_index` and `current_page` to maintain consistency.
    pub fn previous_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = Page::titles().len() - 1;
        }
        self.current_page = Page::from_index(self.tab_index);
    }

    /// Renders the current page to the terminal frame
    ///
    /// Divides the screen into a tab bar and content area, then renders
    /// the appropriate content based on the current page. Fetches fresh
    /// metrics data for the current page before rendering.
    ///
    /// # Arguments
    ///
    /// * `frame` - The ratatui frame to render into
    ///
    /// # Page-specific behavior
    ///
    /// - **System**: Displays static system information (OS, hostname, etc.)
    /// - **Metrics**: Shows real-time resource usage (CPU, memory, disk)
    /// - **Processes**: Lists all running processes with sortable columns
    /// - **SingleProcess**: Detailed view of a specific process
    /// - **Network**: Network interface statistics and traffic
    pub fn render(&mut self, frame: &mut Frame) {
        let chunks =
            Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());

        // render tabs
        self.render_tabs(frame, chunks[0]);

        match &self.current_page {
            Page::Metrics => {
                if let Metrics::SystemResource(system_collector) =
                    self.metrics.fetch(MetricsToFetch::SystemResource)
                {
                    let _ = system_collector.display(frame, chunks[1], None);
                };
            }
            Page::System => {
                if let Metrics::SystemInfo(system_info) =
                    self.metrics.fetch(MetricsToFetch::SystemInfo)
                {
                    let _ = system_info.display(frame, chunks[1], None);
                };
            }
            Page::Processes => {
                if let Metrics::Processes(processes) = self.metrics.fetch(MetricsToFetch::Process) {
                    self.ui_state.process_table.process_count = processes.len();
                    let _ = processes.display(frame, chunks[1], Some(&mut self.ui_state));
                }
            }
            Page::SingleProcess(pid) => {
                let total_memory = bytes_to_mb(self.metrics.system.total_memory());
                if let Metrics::SingleProcessPid(Some(process)) =
                    self.metrics.fetch(MetricsToFetch::SingleProcessPid(*pid))
                {
                    self.ui_state
                        .single_process_disk_usage
                        .update_disk_history(process.basic_process_data.pid, &process.disk_usage);

                    let _ = SingleProcessUI { data: process }.display_process_metrics(
                        frame,
                        chunks[1],
                        total_memory,
                        &mut self.ui_state,
                    );
                }
            }
            Page::Network => {
                if let Metrics::Networks(network_metrics) =
                    self.metrics.fetch(MetricsToFetch::Networks)
                {
                    let _ = network_metrics.display(frame, chunks[1], Some(&mut self.ui_state));
                }
            }
        }
    }

    /// Renders the tab bar at the top of the screen
    ///
    /// Displays all available pages as tabs with the current tab highlighted
    /// in green and bold.
    ///
    /// # Arguments
    ///
    /// * `frame` - The ratatui frame to render into
    /// * `area` - The rectangular area to render the tabs in
    pub fn render_tabs(&self, frame: &mut Frame, area: Rect) {
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

    /// Handles keyboard events from the user
    ///
    /// Processes both global keyboard shortcuts (navigation, quit) and
    /// page-specific shortcuts (e.g., process list navigation). Only
    /// key press events are processed; key release events are ignored.
    ///
    /// # Arguments
    ///
    /// * `key` - The keyboard event to process
    ///
    /// # Errors
    ///
    /// Returns an error if event processing fails (currently always returns `Ok`).
    pub fn handle_events(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        if key.kind == KeyEventKind::Press {
            self.process_global_events(key);
            match self.current_page {
                Page::Processes => {
                    self.process_page_events(key);
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Processes global keyboard shortcuts available on all pages
    ///
    /// # Keybindings
    ///
    /// - `q` - Quit the application
    /// - `Tab` or `Right Arrow` - Next tab
    /// - `Left Arrow` - Previous tab
    /// - `1` - Jump to System page
    /// - `2` - Jump to Metrics page
    /// - `3` - Jump to Processes page
    /// - `4` - Jump to Network page
    ///
    /// # Arguments
    ///
    /// * `key` - The keyboard event to process
    fn process_global_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.render = false;
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
            KeyCode::Char('3') => {
                self.tab_index = 2;
                self.current_page = Page::Processes;
            }
            KeyCode::Char('4') => {
                self.tab_index = 3;
                self.current_page = Page::Network;
            }
            _ => {}
        }
    }

    /// Processes page-specific keyboard events for the Processes page
    ///
    /// Handles navigation through the process list and opening detailed
    /// process views.
    ///
    /// # Keybindings (Processes page only)
    ///
    /// - `Up Arrow` - Select previous process in the list
    /// - `Down Arrow` - Select next process in the list
    /// - `Enter` - Open detailed view for the selected process
    ///
    /// # Arguments
    ///
    /// * `key` - The keyboard event to process
    fn process_page_events(&mut self, key: KeyEvent) {
        let max_processes = self.ui_state.process_table.process_count;
        match key.code {
            KeyCode::Down => {
                if let Some(selected_row) = self.ui_state.process_table.process_list.selected() {
                    let next_row = (selected_row + 1).min(max_processes.saturating_sub(1));
                    self.ui_state
                        .process_table
                        .process_list
                        .select(Some(next_row));
                }
            }
            KeyCode::Up => {
                if let Some(selected_row) = self.ui_state.process_table.process_list.selected() {
                    let next_row = selected_row.saturating_sub(1);
                    self.ui_state
                        .process_table
                        .process_list
                        .select(Some(next_row));
                }
            }
            KeyCode::Enter => {
                if let Some(selected_process_pid) = self.ui_state.process_table.selected_pid {
                    self.current_page = Page::SingleProcess(selected_process_pid);
                }
            }
            _ => {}
        }
    }
}
