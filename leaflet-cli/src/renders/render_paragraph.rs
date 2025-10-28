use ratatui::widgets::{Block, Borders, Paragraph};

pub fn paragraph_widget<'a>(text: &'a str, title: &'a str) -> Paragraph<'a> {
    Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(title))
}
