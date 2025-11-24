use std::collections::HashMap;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};
use stomata_core::NetworkMetrics;

use crate::{
    renders::{
        displays::traits::Display,
        render_widgets::{render_paragraph::paragraph_widget, render_sparkline::render_sparkline},
    },
    structs::{NetworkInterfaceData, UIState},
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
        let layout1 = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]).split(area);
        let layout = Layout::vertical(&constraints).split(layout1[0]);

        if let Some(ui_state) = ui_state {
            let map = ui_state.networks_state.get_or_insert(HashMap::new());

            for (index, interface) in self.interfaces.iter().enumerate() {
                let iface = map
                    .entry(interface.name.clone())
                    .or_insert_with(NetworkInterfaceData::default);

                iface.update_network_history(interface);

                let sparkline_title = format!(
                    "Interface name: {} - Bytes received: {} - Bytes transmitted: {} - Total Bytes Received: {} - Total Bytes Transmitted: {} - Packets Received: {} - Packet transmitted: {}",
                    interface.name,
                    interface.bytes_received,
                    interface.bytes_transmitted,
                    interface.total_bytes_received,
                    interface.total_bytes_transmitted,
                    interface.packets_received,
                    interface.packets_transmitted
                );
                let sparkline_widget =
                    render_sparkline(iface.received_bytes.make_contiguous(), &sparkline_title);

                
                // let network_para_widget = paragraph_widget(&network_para_info, &interface.name);
                frame.render_widget(sparkline_widget, layout[index]);
            }
        }
        Ok(())
    }
}
