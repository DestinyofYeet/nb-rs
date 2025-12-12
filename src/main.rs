use std::fs::{self};

use anyhow::Result;
use clap::Parser;
use tracing::{debug, error};
use tracing_subscriber::EnvFilter;

use crate::{
    actions::{
        folder::{create::folder_create, list::folder_list, remove::folder_remove},
        note::{create::note_create, open::note_open, remove::note_remove},
    },
    args::top::Args,
    config::model::Config,
};

mod actions;
mod args;
mod config;

fn main() -> Result<()> {
    let args = Args::parse();
    let level = match args.verbose {
        0 => "error",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    let config = Config::new(&args)?;

    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_env_filter(EnvFilter::new(level))
        .init();

    if !config.data_dir.exists() {
        match fs::create_dir(config.data_dir.clone()) {
            Ok(_) => {
                debug!("Created directory: {}", config.data_dir.to_str().unwrap());
            }

            Err(e) => {
                error!(
                    "Failed to create directory '{}': {e}",
                    config.data_dir.to_str().unwrap()
                )
            }
        }
    } else {
        debug!(
            "Not creating '{}' because it exists",
            config.data_dir.to_str().unwrap()
        )
    }

    dbg!(&config);

    match args.action {
        args::actions::ActionArgs::Create { folder, note } => {
            if let Some(folder) = folder {
                folder_create(&config.data_dir, &folder)?;
            }

            if let Some(note) = note {
                note_create(&config.data_dir, &note)?;
                note_open(&config.data_dir, &note, &config.editor)?;
            }
        }
        args::actions::ActionArgs::Open { note } => {
            note_open(&config.data_dir, &note, &config.editor)?;
        }
        args::actions::ActionArgs::Ls { folder } => {
            folder_list(&config.data_dir, &folder)?;
        }
        args::actions::ActionArgs::Rm { folder, note } => {
            if let Some(note) = note {
                note_remove(&config.data_dir, &note)?;
            }

            if let Some(folder) = folder {
                folder_remove(&config.data_dir, &folder)?;
            }
        }
    }

    Ok(())
}
