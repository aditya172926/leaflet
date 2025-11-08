use ratatui::{layout::Constraint, style::{Color, Style}, widgets::{Cell, Row, Table}};
use stomata_core::collectors::structs::ProcessData;

use crate::structs::TableRow;

impl TableRow for ProcessData {
    fn to_cells(&self) -> Vec<Cell> {
        vec![
            Cell::from(self.pid.to_string()),
            Cell::from(self.name.clone()),
            Cell::from(format!("{:.2}%", self.cpu_usage)),
            Cell::from(format!("{} MB", self.memory)),
            Cell::from(self.status.clone())
        ]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(8),   // PID
            Constraint::Min(20),     // Name (flexible)
            Constraint::Length(10),  // CPU%
            Constraint::Length(12),  // Memory
            Constraint::Length(10),  // Status
        ]
    }
}

pub fn render_table<T>(
    headers: Vec<String>,
    items: Vec<T>
) {
    let header_style = Style::default()
        .fg(Color::White)
        .bg(Color::Black);

    let header = headers
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);

    // let rows = items.iter().enumerate().map(|(index, data)| {

    // })
}