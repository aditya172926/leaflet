use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout}, style::{Style, Stylize}, widgets::{Bar, BarChart, BarGroup, Block, Borders}, DefaultTerminal, Frame
};

pub fn render_bar(frame: &mut Frame, data: &[f32]) {
    let [title, vertical] = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1).areas(frame.area());

    frame.render_widget("Memory In Use", title);
    frame.render_widget(vertical_bar_chart(data), vertical);
}

fn vertical_bar_chart(data: &[f32]) -> BarChart {
    let bars: Vec<Bar> = data.iter().map(|(value)| vertical_bar(value, String::from("Memory"))).collect();
    BarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Memory Usage"))
        .data(BarGroup::default().label("Usage".into()).bars(&bars))
        .bar_width(10)
}

fn vertical_bar(data: &f32, label: String) -> Bar {
    Bar::default()
        .label(label.into())
        .value(*data as u64)
        .style(Style::new().blue())
        .value_style(Style::new().blue().on_white())
        .text_value(format!("{}%", data))
}