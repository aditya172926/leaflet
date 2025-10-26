use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout}, style::{Style, Stylize}, widgets::{Bar, BarChart, BarGroup, Block}, DefaultTerminal, Frame
};

pub fn render_bar(frame: &mut Frame, data: &[u64]) {
    let [title, vertical] = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1).areas(frame.area());

    frame.render_widget("Memory In Use", title);
    frame.render_widget(vertical_bar_chart(data), vertical);
}

fn vertical_bar_chart(data: &[u64]) -> BarChart {
    let bars: Vec<Bar> = data.iter().map(|(value)| vertical_bar(value, String::from("Memory"))).collect();
    BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .bar_width(5)
}

fn vertical_bar(data: &u64, label: String) -> Bar {
    Bar::default()
        .label(label.into())
        .value(*data)
        .style(Style::new().blue())
        .value_style(Style::new().blue().on_white())
}