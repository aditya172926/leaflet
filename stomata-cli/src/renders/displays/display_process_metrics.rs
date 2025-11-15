use crate::{renders::render_widgets::render_paragraph::paragraph_widget, structs::UIState};
use ratatui::{Frame, layout::Rect};
use stomata_core::collectors::structs::ProcessData;
use sysinfo::Process;

pub trait SingleProcessDisplay {
    fn display_process_metrics(&self, frame: &mut Frame, area: Rect) -> anyhow::Result<()>;
}

impl SingleProcessDisplay for Process {
    fn display_process_metrics(&self, frame: &mut Frame, area: Rect) -> anyhow::Result<()> {
        let p_info = format!("\n\nPID: {}", self.pid());
        let title = &format!("Process {}", self.pid());
        let paragraph = paragraph_widget(&p_info, title);
        frame.render_widget(
            paragraph.alignment(ratatui::layout::Alignment::Center),
            area,
        );
        Ok(())
    }
}
