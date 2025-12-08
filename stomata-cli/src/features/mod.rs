use std::io::Stdout;

use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::structs::{Cli, Feature};

#[cfg(feature = "core")]
pub mod core;

#[cfg(feature = "web3")]
pub mod web3;

pub fn run_feature(
    feature: Feature,
    cli: &Cli,
    terminal: Option<&mut Terminal<CrosstermBackend<Stdout>>>,
) -> anyhow::Result<bool> {
    match feature {
        #[cfg(feature = "core")]
        Feature::Core => core::core_feature::run(cli, terminal),
        #[cfg(feature = "web3")]
        Feature::Web3 => web3::web3_feature::run(cli, terminal),
    }
}
