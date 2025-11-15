use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Gauge},
};

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
