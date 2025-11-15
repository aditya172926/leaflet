use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};
use stomata_core::collectors::structs::ProcessData;
use sysinfo::Process;

use crate::{structs::TableRow, utils::bytes_to_mb};

impl TableRow for ProcessData {
    fn to_cells(&self) -> Vec<Cell<'_>> {
        vec![
            Cell::from(self.pid.to_string()),
            Cell::from(self.name.clone()),
            Cell::from(format!("{:.2}%", self.cpu_usage)),
            Cell::from(format!("{} MB", bytes_to_mb(self.memory))),
            Cell::from(self.status.clone()),
        ]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(8),  // PID
            Constraint::Min(20),    // Name (flexible)
            Constraint::Length(10), // CPU%
            Constraint::Length(12), // Memory
            Constraint::Length(10), // Status
        ]
    }
}

impl TableRow for &Process {
    fn to_cells(&self) -> Vec<Cell<'_>> {
        vec![
            Cell::from(self.pid().as_u32().to_string()),
            Cell::from(self.name().to_string_lossy().to_string()),
            Cell::from(format!("{:.2}%", self.cpu_usage())),
            Cell::from(format!("{} MB", bytes_to_mb(self.memory()))),
            Cell::from(self.status().to_string()),
        ]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(8),  // PID
            Constraint::Min(20),    // Name (flexible)
            Constraint::Length(10), // CPU%
            Constraint::Length(12), // Memory
            Constraint::Length(10), // Status
        ]
    }
}

pub fn render_table<'a, T>(headers: Vec<&'a str>, items: &'a [T], title: &'a str) -> Table<'a>
where
    T: TableRow,
{
    let header_style = Style::default().fg(Color::White).bg(Color::Black);

    let header = headers
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);

    let rows: Vec<Row> = items
        .iter()
        .map(|item| {
            let cells = item.to_cells();
            Row::new(cells).height(1)
        })
        .collect();

    Table::new(rows, T::column_widths())
        .row_highlight_style(Style::default().bg(Color::White).fg(Color::Black))
        .highlight_symbol(">>")
        .header(header)
        .block(Block::default().title(title).borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
}
