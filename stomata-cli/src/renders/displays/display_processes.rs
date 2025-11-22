use ratatui::{Frame, layout::Rect};
use stomata_core::collectors::process::metrics::ProcessData;

use crate::{
    renders::{displays::traits::Display, render_widgets::render_table::render_table},
    structs::UIState,
};

impl Display for Vec<ProcessData> {
    fn display(
        &self,
        frame: &mut Frame,
        area: Rect,
        ui_state: Option<&mut UIState>,
    ) -> anyhow::Result<()> {
        // self.update_metrics(MetricsCategory::ProcessesWithoutTasks); // update processes only

        // let processes = match self.get_latest_metric() {
        //     Some(metrics) => metrics.processes.clone(),
        //     None => Vec::new(),
        // };
        let headers = vec!["PID", "Name", "CPU", "Memory", "Status"];

        let table_widget = render_table(headers, &self, "Processes");
        if let Some(ui_state) = ui_state {
            frame.render_stateful_widget(table_widget, area, &mut ui_state.process_list);
        }
        Ok(())
    }
}
