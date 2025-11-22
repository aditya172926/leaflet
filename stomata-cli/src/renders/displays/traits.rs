use ratatui::{Frame, layout::Rect};

pub trait Display {
    fn display(&self, frame: &mut Frame, area: Rect) -> anyhow::Result<()>;
}
