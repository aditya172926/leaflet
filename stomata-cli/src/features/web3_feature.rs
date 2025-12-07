use std::io::Stdout;

use ratatui::{Terminal, prelude::CrosstermBackend};

pub fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>
) -> anyhow::Result<()> {
    Ok(())
}