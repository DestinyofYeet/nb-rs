use std::path::PathBuf;

use crate::args::actions::ActionArgs;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'D', help = "The path to the data directory.")]
    pub data_dir: Option<PathBuf>,

    // #[arg(
    //     short = 'C',
    //     help = "The path to the config file. If not set, will try to use the xdg config dirs."
    // )]
    // pub config_file: Option<PathBuf>,
    #[arg(short='v', action = clap::ArgAction::Count, help="Sets the verbose level. More v's more output", default_value="0")]
    pub verbose: u8,

    #[command(subcommand)]
    pub action: ActionArgs,

    #[arg(long = "version", help = "Shows the version and exits")]
    pub version: bool,
}
