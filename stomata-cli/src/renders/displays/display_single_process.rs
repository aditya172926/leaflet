use crate::{renders::render_widgets::render_paragraph::paragraph_widget, structs::UIState};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
};
use stomata_core::collectors::structs::SingleProcessData;
use sysinfo::Process;

pub trait SingleProcessDisplay {
    fn display_process_metrics(&self, frame: &mut Frame, area: Rect) -> anyhow::Result<()>;
}

impl SingleProcessDisplay for SingleProcessData<'_> {
    fn display_process_metrics(&self, frame: &mut Frame, area: Rect) -> anyhow::Result<()> {
        let mut constraints: Vec<Constraint> = Vec::new();
        let p_info = format!("\n\nPID: {}", self.basic_process_data.pid);
        let title = &format!("Process {}", self.basic_process_data.pid);
        let tasks = &self.tasks;
        let paragraph = paragraph_widget(&p_info, title);
        frame.render_widget(
            paragraph.alignment(ratatui::layout::Alignment::Center),
            area,
        );
        Ok(())
    }
}
