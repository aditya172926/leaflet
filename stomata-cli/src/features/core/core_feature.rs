use std::{
    io::Stdout,
    time::{Duration, Instant},
};

use ratatui::{
    Terminal,
    crossterm::event::{self, Event},
    prelude::CrosstermBackend,
};

use crate::{renders::core_displays::display_app::App, structs::Cli};

pub fn run(
    cli: &Cli,
    terminal: Option<&mut Terminal<CrosstermBackend<Stdout>>>,
) -> anyhow::Result<bool> {
    match terminal {
        Some(terminal) => {
            let store_metrics_data = cli.store;
            let mut app = App::new(store_metrics_data);

            // get the refresh interval from the cli arg. Default 1000 ms
            let refresh_interval = Duration::from_millis(cli.interval);
            let mut last_tick = Instant::now();

            // main render loop
            while app.render {
                let timeout = refresh_interval
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(Duration::from_secs(0));

                // poll for inputs only until timeout
                if event::poll(timeout)? {
                    if let Event::Key(key) = event::read()? {
                        // handle events
                        app.handle_events(key)?;
                        // redraw immediately after an event
                        terminal.draw(|frame| app.render(frame))?;
                    }
                }

                if last_tick.elapsed() >= refresh_interval {
                    // draw
                    terminal.draw(|frame| app.render(frame))?;
                    last_tick = Instant::now();
                }
            }
            Ok(app.render)
        }
        None => Ok(false),
    }
}
