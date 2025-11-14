use crate::{renders::render_paragraph::paragraph_widget, structs::UIState};
use ratatui::{Frame, layout::Rect};
use stomata_core::collectors::structs::ProcessData;

pub trait SingleProcessDisplay {
    fn display_process_metrics(&self, frame: &mut Frame, area: Rect) -> anyhow::Result<()>;
}

impl SingleProcessDisplay for ProcessData {
    fn display_process_metrics(&self, frame: &mut Frame, area: Rect) -> anyhow::Result<()> {
        let p_info = format!(
            "\n\nPID: {}\nProcess Name: {}\nCPU usage: {}\nMemory usage: {}\nStatus: {}",
            self.pid, self.name, self.cpu_usage, self.memory, self.status
        );
        let title = &format!("Process {}", self.pid);
        let paragraph = paragraph_widget(&p_info, title);
        frame.render_widget(
            paragraph.alignment(ratatui::layout::Alignment::Center),
            area,
        );
        Ok(())
    }
}
