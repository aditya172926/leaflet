use std::{thread::sleep, time::Duration};

use crate::{renders::render_app::App, structs::Cli};
use clap::Parser;

mod constants;
mod renders;
mod structs;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut app = App::new();
    let mut terminal = ratatui::init();

    // get the refresh interval from the cli arg. Default 1000 ms
    let refresh_interval = cli.interval;

    // main render loop
    while app.render {
        // draw
        terminal.draw(|frame| app.render(frame))?;

        // handle events
        app.handle_events()?;

        // sleep for refresh interval
        sleep(Duration::from_millis(refresh_interval));
    }

    Ok(())
}
