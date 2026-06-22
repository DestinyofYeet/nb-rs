use std::path::PathBuf;

use clap::Parser;

use crate::app::args::actions::ActionArgs;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'D', help = "The path to the data directory")]
    pub data_dir: Option<PathBuf>,

    #[arg(long = "version", help = "Shows the version and exits")]
    pub version: bool,

    #[arg(short='v', action = clap::ArgAction::Count, help="Sets the verbose level. More v's more output", default_value="0")]
    pub verbose: u8,

    #[arg(short, long, help = "Skip syncing")]
    pub no_sync: bool,

    #[command(subcommand)]
    pub action: ActionArgs,
}
