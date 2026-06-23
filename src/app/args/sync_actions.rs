use clap::Subcommand;
use nb_rs::default_strategies::sync::AvailableDefaultSyncStrategies;

#[derive(Debug, Subcommand)]
pub enum SyncArgs {
    #[command(about = "Setup sync")]
    Setup {
        #[arg(help = "The kind of tracking")]
        kind: AvailableDefaultSyncStrategies,
    },

    #[command(about = "Remove sync")]
    Rm {},

    #[command(about = "Full sync")]
    Full {},

    #[command(about = "Import remote notebook")]
    Import {
        #[arg(help = "The kind of tracking")]
        kind: AvailableDefaultSyncStrategies,
    },
}
