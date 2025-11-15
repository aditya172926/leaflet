use crate::{
    renders::render_widgets::{
        render_gauge::render_gauge, render_paragraph::paragraph_widget, render_table::render_table,
    },
    structs::SingleProcessUI,
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

pub trait SingleProcessDisplay {
    fn display_process_metrics(&self, frame: &mut Frame, area: Rect) -> anyhow::Result<()>;
}

impl SingleProcessDisplay for SingleProcessUI<'_> {
    fn display_process_metrics(&self, frame: &mut Frame, area: Rect) -> anyhow::Result<()> {
        let constraints: Vec<Constraint>;

        let tasks = &self.data.tasks;
        if tasks.len() > 0 {
            constraints = vec![
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ];
        } else {
            constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
        }

        let primary_layout = Layout::horizontal(&constraints).split(area);
        let secondary_layout =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(primary_layout[0]);

        let p_info = format!(
            "PID: {}\nName: {}\nStatus: {}",
            self.data.basic_process_data.pid,
            self.data.basic_process_data.name,
            self.data.basic_process_data.status
        );

        let basic_info_paragraph = paragraph_widget(&p_info, "Basic Task info");
        let extra_info = format!(
            "Start time: {}\nRunning time: {}\nCWD: {}\nTotal written Bytes: {}",
            self.data.start_time,
            self.data.running_time,
            self.data
                .current_working_dir
                .clone()
                .unwrap_or(String::new()),
            self.data.disk_usage.total_read_bytes
        );
        let extra_info_paragraph = paragraph_widget(&extra_info, "More info");
        let cpu_gauge = render_gauge(
            self.data.basic_process_data.cpu_usage.into(),
            100.0,
            "CPU",
            "%",
        );
        // let memory_gauge = render_gauge(self.data.basic_process_data.memory, , label, unit)

        frame.render_widget(
            basic_info_paragraph.alignment(ratatui::layout::Alignment::Left),
            secondary_layout[0],
        );
        frame.render_widget(cpu_gauge, secondary_layout[1]);
        frame.render_widget(extra_info_paragraph, primary_layout[1]);
        if tasks.len() > 0 {
            let task_headers = vec!["PID", "Name", "CPU", "Memory", "Status"];
            let task_widget = render_table(task_headers, &self.data.tasks, "Tasks");
            frame.render_widget(task_widget, primary_layout[2]);
        }
        Ok(())
    }
}
