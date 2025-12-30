//! Table widget rendering utilities
//!
//! Provides generic table rendering functionality with support for process data
//! and other tabular information. Implements the `TableRow` trait for converting
//! data structures into table rows with consistent column layouts.

use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};
use stomata_core::collectors::process::metrics::ProcessData;
use sysinfo::Process;

use crate::{structs::TableRow, utils::bytes_to_mb};

/// Implements table row conversion for `ProcessData`.
///
/// Formats process data into a 5-column table row with PID, name, CPU usage,
/// memory consumption, and process status. Memory values are converted from
/// bytes to megabytes for readability.
///
/// # Column Layout
///
/// 1. **PID** (8 chars): Process identifier
/// 2. **Name** (20+ chars, flexible): Process name
/// 3. **CPU%** (10 chars): CPU usage percentage with 2 decimal places
/// 4. **Memory** (12 chars): Memory usage in MB
/// 5. **Status** (10 chars): Process status string
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

/// Implements table row conversion for `sysinfo::Process` references.
///
/// Provides direct rendering of `sysinfo` process objects without intermediate
/// conversion. Uses the same column layout as `ProcessData` for consistency.
///
/// # Column Layout
///
/// 1. **PID** (8 chars): Process identifier
/// 2. **Name** (20+ chars, flexible): Process name
/// 3. **CPU%** (10 chars): CPU usage percentage with 2 decimal places
/// 4. **Memory** (12 chars): Memory usage in MB
/// 5. **Status** (10 chars): Process status string
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

/// Creates a generic styled table widget from any type implementing `TableRow`.
///
/// Builds a table with a header row, selectable data rows, and consistent styling.
/// The table supports keyboard navigation with row highlighting and includes
/// borders with a title.
///
/// # Type Parameters
///
/// * `T` - Any type implementing the `TableRow` trait, which defines how to
///         convert the type into table cells and column widths
///
/// # Arguments
///
/// * `headers` - Column header labels (e.g., `["PID", "Name", "CPU%"]`)
/// * `items` - Slice of data items to display in the table
/// * `title` - Title text displayed in the border
///
/// # Returns
///
/// A configured `Table` widget ready for rendering with a `TableState`
///
/// # Styling
///
/// - **Header**: White text on black background
/// - **Normal rows**: White text on terminal background
/// - **Selected row**: Black text on white background with ">>" highlight symbol
/// - **Border**: All sides with title
///
/// # Examples
///
/// ```ignore
/// use crate::renders::render_widgets::render_table::render_table;
///
/// let headers = vec!["PID", "Name", "CPU%", "Memory", "Status"];
/// let processes: Vec<ProcessData> = get_processes();
/// let table = render_table(headers, &processes, "Process List");
///
/// // Render with state for selection
/// frame.render_stateful_widget(table, area, &mut table_state);
/// ```
///
/// # Notes
///
/// - Column widths are defined by the `TableRow::column_widths()` implementation
/// - The table requires a `TableState` for rendering selection state
/// - All rows have a fixed height of 1 line
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
