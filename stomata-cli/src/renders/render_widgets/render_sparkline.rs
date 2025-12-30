//! Sparkline widget rendering utilities
//!
//! Provides functions for creating compact line chart widgets that visualize
//! time-series data in a minimal space. Sparklines are ideal for showing
//! trends in metrics like CPU usage, network throughput, or memory over time.

use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Sparkline},
};

// Creates a styled sparkline widget for displaying time-series data.
///
/// Renders a compact line chart with a border and title, showing the
/// progression of values over time. The chart automatically scales to
/// fit the data range within the available space.
///
/// # Arguments
///
/// * `data` - Slice of u64 values representing the time-series data points,
///            ordered from oldest (left) to newest (right)
/// * `title` - Title text displayed in the border
///
/// # Returns
///
/// A configured `Sparkline` widget ready for rendering
///
/// # Examples
///
/// ```ignore
/// use crate::renders::render_widgets::render_sparkline::render_sparkline;
///
/// // CPU usage over time (0-100%)
/// let cpu_history = vec![45, 52, 48, 65, 72, 68, 55, 50];
/// let sparkline = render_sparkline(&cpu_history, "CPU History");
/// frame.render_widget(sparkline, area);
///
/// // Network throughput in KB/s
/// let network_data = vec![120, 340, 560, 420, 380, 450];
/// let sparkline = render_sparkline(&network_data, "Network TX");
/// frame.render_widget(sparkline, area);
/// ```
///
/// # Styling
///
/// - Border: All sides with title at top-left
/// - Line color: White
/// - Background: Transparent (inherits from terminal)
///
/// # Notes
///
/// - Data is displayed left-to-right (oldest to newest)
/// - The chart automatically scales vertically based on min/max values
/// - Works best with at least 10-20 data points for visible trends
/// - Empty data will render an empty chart area
pub fn render_sparkline<'a>(data: &'a [u64], title: &'a str) -> Sparkline<'a> {
    let sparkline = Sparkline::default()
        .block(Block::new().borders(Borders::ALL).title(title))
        .data(data)
        .style(Style::default().fg(Color::White));

    sparkline
}
