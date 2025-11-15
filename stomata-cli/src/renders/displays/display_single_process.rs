use crate::{renders::render_widgets::render_paragraph::paragraph_widget, structs::UIState};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};
use stomata_core::collectors::structs::SingleProcessData;
use sysinfo::Process;

pub trait SingleProcessDisplay {
    fn display_process_metrics(&self, frame: &mut Frame, area: Rect) -> anyhow::Result<()>;
}

impl SingleProcessDisplay for SingleProcessData<'_> {
    fn display_process_metrics(&self, frame: &mut Frame, area: Rect) -> anyhow::Result<()> {
        let layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let tasks = &self.tasks;
        if tasks.len() > 0 {
            let mut tasks_para = String::new();
            for task in tasks {
                let pid = format!("\nTask pid: {}", task.pid().as_u32().to_string());
                tasks_para.push_str(&pid);
            }

            let task_para = paragraph_widget(&tasks_para, "Task pids");
            frame.render_widget(task_para, layout[1]);
        }

        let mut constraints: Vec<Constraint> = Vec::new();
        let p_info = format!("\n\nPID: {}", self.basic_process_data.pid);
        let title = &format!("Process {}", self.basic_process_data.pid);

        let paragraph = paragraph_widget(&p_info, "Basic Task info");
        frame.render_widget(
            paragraph.alignment(ratatui::layout::Alignment::Center),
            layout[0],
        );
        Ok(())
    }
}
