use std::collections::HashMap;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::Sparkline,
};
use stomata_core::{NetworkMetrics, collectors::network::metrics::NetworkInterfaces};

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
        let layout = Layout::horizontal(&constraints).split(area);

        if let Some(ui_state) = ui_state {
            let map = ui_state.networks_state.get_or_insert(HashMap::new());

            for (index, interface) in self.interfaces.iter().enumerate() {
                let iface = map
                    .entry(interface.name.clone())
                    .or_insert_with(NetworkInterfaceData::default);

                iface.update_network_history(interface);

                let received_bytes_sparkline_title = format!(
                    "Interface name: {} - Bytes received: {}",
                    interface.name, interface.bytes_received
                );

                let transmitted_bytes_sparkline_title = format!(
                    "Interface name: {} - Bytes transmitted: {}",
                    interface.name, interface.bytes_transmitted
                );

                let packets_received_sparkline_title = format!(
                    "Interface name: {} - Packets received: {}",
                    interface.name, interface.packets_received
                );

                let packets_transmitted_sparkline_title = format!(
                    "Interface name: {} - Packets transmitted: {}",
                    interface.name, interface.packets_transmitted
                );

                let errors_received_sparkline_title = format!(
                    "Interface name: {} - Errors Received: {}",
                    interface.name, interface.errors_on_received
                );

                let errors_transmitted_sparkline_title = format!(
                    "Interface name: {} - Errors Transmitted: {}",
                    interface.name, interface.errors_on_transmitted
                );

                //-- widgets --
                let sparkline_widgets = vec![
                    render_sparkline(
                        iface.received_bytes.make_contiguous(),
                        &received_bytes_sparkline_title,
                    ),
                    render_sparkline(
                        iface.transmitted_bytes.make_contiguous(),
                        &transmitted_bytes_sparkline_title,
                    ),
                    render_sparkline(
                        iface.packets_received.make_contiguous(),
                        &packets_received_sparkline_title,
                    ),
                    render_sparkline(
                        iface.packets_transmitted.make_contiguous(),
                        &packets_transmitted_sparkline_title,
                    ),
                    render_sparkline(
                        iface.errors_received.make_contiguous(),
                        &errors_received_sparkline_title,
                    ),
                    render_sparkline(
                        iface.errors_transmitted.make_contiguous(),
                        &errors_transmitted_sparkline_title,
                    ),
                ];

                let secondart_constraints =
                    vec![
                        Constraint::Percentage(100 / sparkline_widgets.len() as u16);
                        sparkline_widgets.len()
                    ];
                let secondary_layout =
                    Layout::vertical(&secondart_constraints).split(layout[index]);

                for (widget_index, widget) in sparkline_widgets.iter().enumerate() {
                    frame.render_widget(widget, secondary_layout[widget_index]);
                }
            }
        }
        Ok(())
    }
}
