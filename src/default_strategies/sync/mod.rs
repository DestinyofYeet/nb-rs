pub mod git;
pub mod no_op;

use std::fmt::Display;

use clap::ValueEnum;

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum AvailableDefaultSyncStrategies {
    Git,
}

impl Display for AvailableDefaultSyncStrategies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AvailableDefaultSyncStrategies::Git => "git",
        })
    }
}
