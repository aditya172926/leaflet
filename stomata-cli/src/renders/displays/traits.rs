use ratatui::{Frame, layout::Rect};

use crate::structs::UIState;

pub trait Display {
    fn display(
        &self,
        frame: &mut Frame,
        area: Rect,
        ui_state: Option<&mut UIState>,
    ) -> anyhow::Result<()>;
}
