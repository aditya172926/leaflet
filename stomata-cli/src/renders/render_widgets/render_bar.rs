//! Bar chart rendering utilities
//!
//! Provides functions for creating and rendering vertical bar charts,
//! primarily used for visualizing memory usage and other percentage-based
//! metrics in the system monitor.

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    widgets::{Bar, BarChart, BarGroup, Block, Borders},
};

/// Renders a vertical bar chart with a title to the frame.
///
/// Creates a two-section layout with a title area at the top and the
/// bar chart below. Currently configured for memory usage visualization.
///
/// # Arguments
///
/// * `frame` - Mutable reference to the ratatui frame for rendering
/// * `data` - Slice of f32 values representing percentages to display as bars
///
/// # Layout
///
/// - Top section: Fixed 1-line height for the title "Memory In Use"
/// - Bottom section: Remaining space for the vertical bar chart
/// - Spacing: 1 line between sections
pub fn render_bar(frame: &mut Frame, data: &[f32]) {
    let [title, vertical] = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
        .spacing(1)
        .areas(frame.area());

    frame.render_widget("Memory In Use", title);
    frame.render_widget(vertical_bar_chart(data), vertical);
}

/// Creates a styled vertical bar chart widget from percentage data.
///
/// Builds a bar chart with borders, title, and formatted bars showing
/// memory usage percentages. Each data point becomes a labeled bar with
/// blue styling and a white background for the value text.
///
/// # Arguments
///
/// * `data` - Slice of f32 values representing percentages (0-100)
///
/// # Returns
///
/// A configured `BarChart` widget ready for rendering
///
/// # Styling
///
/// - Border: All sides with "Memory Usage" title
/// - Bar width: 10 characters
/// - Bar color: Blue with white-on-blue value labels
/// - Group label: "Usage"
pub fn vertical_bar_chart(data: &[f32]) -> BarChart<'_> {
    let bars: Vec<Bar> = data
        .iter()
        .map(|value| vertical_bar(value, String::from("Memory")))
        .collect();
    BarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Memory Usage"))
        .data(BarGroup::default().label("Usage".into()).bars(&bars))
        .bar_width(10)
}

/// Creates a single styled bar for a vertical bar chart.
///
/// Constructs a bar with a label, percentage value, and blue styling.
/// The value is displayed with two decimal places of precision.
///
/// # Arguments
///
/// * `data` - Pointer to the f32 percentage value (0-100)
/// * `label` - Label text to display for this bar
fn vertical_bar(data: &f32, label: String) -> Bar<'_> {
    Bar::default()
        .label(label.into())
        .value(*data as u64)
        .style(Style::new().blue())
        .value_style(Style::new().blue().on_white())
        .text_value(format!("{:.2}%", data))
}
