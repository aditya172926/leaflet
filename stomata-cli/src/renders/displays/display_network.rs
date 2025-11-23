use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};
use stomata_core::NetworkMetrics;

use crate::{
    renders::{displays::traits::Display, render_widgets::render_paragraph::paragraph_widget},
    structs::UIState,
};

impl Display for NetworkMetrics {
    fn display(
        &self,
        frame: &mut Frame,
        area: Rect,
        ui_state: Option<&mut UIState>,
    ) -> anyhow::Result<()> {
        let number_of_interfaces: u16 = self.interfaces.len().try_into().unwrap_or(5);
        let constraints =
            vec![Constraint::Percentage((100 / number_of_interfaces)); number_of_interfaces.into()];
        let layout = Layout::vertical(&constraints).split(area);
        for (index, interface) in self.interfaces.iter().enumerate() {
            let network_para_info = format!(
                "Interface name: {}\nBytes received: {}\n, Bytes transmitted: {}\n",
                interface.name, interface.bytes_received, interface.bytes_transmitted
            );
            let network_para_widget = paragraph_widget(&network_para_info, &interface.name);
            frame.render_widget(network_para_widget, layout[index]);
        }
        Ok(())
    }
}

pub fn display_network_stats(frame: &mut Frame, area: Rect) -> anyhow::Result<()> {
    let networks_string = "Displaying networks information";
    let paragraph_widget = paragraph_widget(networks_string, "Networks info");
    frame.render_widget(paragraph_widget, area);
    Ok(())
}
