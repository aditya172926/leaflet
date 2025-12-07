use std::time::{Duration, Instant};

use crate::{
    features::run_feature,
    renders::core_displays::display_app::App,
    structs::{AppState, Cli, StomataState},
};
use clap::Parser;
use ratatui::crossterm::event::{self, Event};

mod constants;
mod features;
mod renders;
mod stomata_state;
mod structs;
mod utils;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut app = StomataState::new();

    if app.available_features.is_empty() {
        eprintln!("Error: No features enabled. Build with at least one feature:");
        return Ok(());
    }

    let mut terminal = ratatui::init();

    loop {
        match app.state {
            AppState::FeatureSelection => {
                terminal.draw(|frame| app.render_feature_selection(frame))?;
                if let Event::Key(key) = event::read()? {
                    if !app.handle_feature_selection(key) {
                        break; // User quit
                    }
                }
            }
            AppState::RunningFeature(feature) => {
                // Run the selected feature
                match run_feature(feature, &cli, &mut terminal) {
                    Ok(render) => {
                        if !render {
                            app.state = AppState::FeatureSelection;
                        }
                    }
                    Err(_) => {
                        eprint!("Error in rendering feature");
                        app.state = AppState::FeatureSelection;
                    }
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}
