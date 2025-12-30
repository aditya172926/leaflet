//! Core feature implementation
//!
//! Provides the main interactive TUI application for system monitoring
//! and core functionality. This feature displays real-time system metrics
//! and provides an interactive interface for various system utilities.

use std::{
    io::Stdout,
    time::{Duration, Instant},
};

use ratatui::{
    Terminal,
    crossterm::event::{self, Event},
    prelude::CrosstermBackend,
};

use crate::{renders::core_displays::display_app::App, structs::Cli};

/// Runs the core feature in interactive TUI mode
///
/// Initializes and runs the main application loop for the core feature,
/// which provides system monitoring and interactive utilities. Unlike other
/// features, core only supports interactive mode (TUI) and requires a terminal.
///
/// # Arguments
///
/// * `cli` - Parsed command-line arguments including:
///   - `interval` - Refresh rate in milliseconds (default: 1000ms)
///   - `store` - Whether to store metrics data for historical analysis
/// * `terminal` - Terminal for rendering the TUI. Must be `Some` for core feature.
///   If `None`, the function returns immediately without doing anything.
///
/// # Returns
///
/// * `Ok(true)` - Application exited normally (user pressed quit key)
/// * `Ok(false)` - No terminal provided, nothing executed
///
/// # Errors
///
/// Returns an error if:
/// - Terminal event polling fails
/// - Terminal rendering fails
/// - Event handling encounters an error
///
/// # Render Loop
///
/// The function implements an event-driven render loop:
/// 1. Polls for keyboard input with timeout based on refresh interval
/// 2. Handles user input immediately and redraws
/// 3. Redraws at regular intervals (based on `cli.interval`)
/// 4. Continues until user quits or an error occurs
///
/// # Performance
///
/// The refresh interval controls how often the display updates. Lower values
/// provide more real-time feedback but consume more CPU. Typical values:
/// - Fast: 250-500ms (high CPU usage)
/// - Balanced: 1000ms (default, recommended)
/// - Slow: 2000-5000ms (low CPU usage)
///
/// # Examples
///
/// ```rust,no_run
/// use stomata::features::core::core_feature;
/// use stomata::structs::Cli;
///
/// let cli = Cli {
///     interval: 1000,  // 1 second refresh
///     store: false,    // don't store metrics
///     ..Default::default()
/// };
///
/// let mut terminal = setup_terminal()?;
/// let exited_normally = core_feature::run(&cli, Some(&mut terminal))?;
/// ```
///
/// # Terminal Requirements
///
/// Core feature only works in interactive mode and will return `Ok(false)`
/// if no terminal is provided. Ensure a terminal is always passed when
/// running the core feature.
pub fn run(
    cli: &Cli,
    terminal: Option<&mut Terminal<CrosstermBackend<Stdout>>>,
) -> anyhow::Result<bool> {
    match terminal {
        Some(terminal) => {
            let store_metrics_data = cli.store;
            let mut app = App::new(store_metrics_data);

            // get the refresh interval from the cli arg. Default 1000 ms
            let refresh_interval = Duration::from_millis(cli.interval);
            let mut last_tick = Instant::now();

            // main render loop
            while app.render {
                let timeout = refresh_interval
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(Duration::from_secs(0));

                // poll for inputs only until timeout
                if event::poll(timeout)? {
                    if let Event::Key(key) = event::read()? {
                        // handle events
                        app.handle_events(key)?;
                        // redraw immediately after an event
                        terminal.draw(|frame| app.render(frame))?;
                    }
                }

                if last_tick.elapsed() >= refresh_interval {
                    // draw
                    terminal.draw(|frame| app.render(frame))?;
                    last_tick = Instant::now();
                }
            }
            Ok(app.render)
        }
        None => Ok(false),
    }
}
