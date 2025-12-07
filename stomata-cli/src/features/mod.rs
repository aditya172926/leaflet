use std::io::Stdout;

use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::structs::{Cli, Feature};

#[cfg(feature = "core")]
pub mod core_feature;

#[cfg(feature = "web3")]
pub mod web3_feature;

pub fn run_feature(
    feature: Feature,
    cli: &Cli,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> anyhow::Result<()> {
    match feature {
        #[cfg(feature = "core")]
        Feature::Core => core_feature::run(cli, terminal),
        #[cfg(feature = "web3")]
        Feature::Web3 => web3_feature::run(cli, terminal),
    }
}
