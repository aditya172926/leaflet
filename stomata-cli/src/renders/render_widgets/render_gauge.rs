//! Gauge widget rendering utilities
//!
//! Provides functions for creating styled gauge widgets that display
//! resource usage as a progress bar with percentage and absolute values.
//! Gauges automatically change color based on usage thresholds.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Gauge},
};

/// Creates a styled gauge widget for displaying resource usage.
///
/// Renders a progress bar showing both percentage and absolute values
/// (current/max) with the specified unit. The gauge color changes to red
/// when usage exceeds 90% to indicate critical levels.
///
/// # Arguments
///
/// * `value` - Current usage value (e.g., used memory in GB)
/// * `max` - Maximum available value (e.g., total memory in GB)
/// * `label` - Title text displayed in the gauge border (e.g., "CPU Usage")
/// * `unit` - Unit string for the values (e.g., "GB", "%", "MB/s")
///
/// # Returns
///
/// A configured `Gauge` widget ready for rendering
///
/// # Display Format
///
/// The gauge label shows: `{percentage:.2}% ({value:.2} {unit}/ {max:.2} {unit})`
///
/// Example: `75.50% (6.04 GB/ 8.00 GB)`
///
/// # Color Scheme
///
/// - **Normal (0-90%)**: Light blue gauge on black background
/// - **Critical (>90%)**: Red gauge on black background
/// - Label text: White
/// - Style: Bold
///
/// # Examples
///
/// ```ignore
/// use crate::renders::render_widgets::render_gauge::render_gauge;
///
/// // Memory usage gauge
/// let gauge = render_gauge(6.04, 8.0, "Memory", "GB");
/// frame.render_widget(gauge, area);
///
/// // CPU usage gauge
/// let gauge = render_gauge(85.5, 100.0, "CPU", "%");
/// frame.render_widget(gauge, area);
/// ```
///
/// # Notes
///
/// - Ratio is clamped between 0.0 and 1.0 to prevent rendering issues
/// - Negative values are treated as 0.0
/// - All numeric values are formatted with 2 decimal places
pub fn render_gauge<'a>(value: f64, max: f64, label: &'a str, unit: &'a str) -> Gauge<'a> {
    let ratio = if value > 0.0 { value / max } else { 0.0 };
    let ratio = ratio.clamp(0.0, 1.0);

    let display_label = format!(
        "{:.2}% ({:.2} {}/ {:.2} {})",
        ratio * 100.0,
        value,
        unit,
        max,
        unit
    );

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(label))
        .gauge_style(
            Style::default()
                .fg(if ratio > 0.9 {
                    Color::Red
                } else {
                    Color::LightBlue
                })
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .label(Span::styled(
            display_label,
            Style::default().fg(Color::White),
        ))
        .ratio(ratio);

    return gauge;
}
