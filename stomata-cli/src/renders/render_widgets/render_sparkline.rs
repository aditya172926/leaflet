use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Sparkline},
};

pub fn render_sparkline<'a>(data: &'a [u64], title: &'a str) -> Sparkline<'a> {
    let sparkline = Sparkline::default()
        .block(Block::new().borders(Borders::ALL).title(title))
        .data(data)
        .style(Style::default().fg(Color::White));

    sparkline
}
