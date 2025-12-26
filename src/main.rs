use colored::Colorize;
use fuzzy_select::FuzzySelect;
use std::{
    collections::HashMap,
    fs::{self},
};

use anyhow::Result;
use clap::{CommandFactory, FromArgMatches, error::ErrorKind};
use itertools::Itertools;
use tracing::{debug, error};
use tracing_subscriber::EnvFilter;

use crate::{
    actions::{
        folder::model::Folder,
        note::model::{Note, NoteError},
    },
    args::top::Args,
    config::model::Config,
};

mod actions;
mod args;
mod config;

pub static GIT_REV: &str = env!("GIT_REV");

fn main() -> Result<()> {
    let arg_matches = match Args::command().try_get_matches() {
        Ok(value) => value,
        Err(e) => {
            if e.kind() == ErrorKind::MissingSubcommand
                && std::env::args().any(|a| a == "--version")
            {
                println!("Compiled at {}", GIT_REV.blue());
                return Ok(());
            }

            e.exit();
        }
    };
    let args = Args::from_arg_matches(&arg_matches).unwrap();

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

    if config.offline {
        debug!("We are offline, not syncing stuff!");
    }

    match args.action {
        args::actions::ActionArgs::Create { folder, note } => {
            if let Some(folder) = folder {
                let folder = Folder::from_pathbuf(&config.data_dir, &folder)?;
                folder.create()?;
            }

            if let Some(note) = note {
                let note = Note::new_create(config.data_dir.to_str().unwrap(), note)?;
                note.open(&config)?;
            }
        }
        args::actions::ActionArgs::Open { note: note_string } => {
            let note = Note::from_pathbuf(&config.data_dir, note_string.clone());
            let note = match note {
                Ok(note) => note,
                Err(e) => match e {
                    NoteError::NoteDoesNotExist(non_existant_note) => {
                        let folder = Folder::from_pathbuf(&config.data_dir, ".")?;
                        let notes = folder.get_notes_by_name(&note_string.to_lowercase())?;
                        match notes.len() {
                            0 => {
                                return Err(NoteError::NoteDoesNotExist(non_existant_note).into());
                            }
                            1 => notes.first().unwrap().clone(),

                            _ => {
                                let mut map = HashMap::<String, Note>::new();
                                let mut options = Vec::new();

                                for note in notes {
                                    let name = note.get_name()?;
                                    options.push(name.clone());
                                    map.insert(name, note);
                                }

                                let selected = FuzzySelect::new()
                                    .with_prompt("Select a note:")
                                    .with_options(options)
                                    .select()?;

                                match map.remove(&selected) {
                                    None => {
                                        return Err(NoteError::NoteDoesNotExist(selected).into());
                                    }
                                    Some(value) => value,
                                }
                            }
                        }
                    }

                    _ => return Err(e.into()),
                },
            };

            note.open(&config)?;
        }
        args::actions::ActionArgs::Ls { folder } => {
            let folder = Folder::from_pathbuf(&config.data_dir, folder)?;
            let result = folder.list()?;
            result.print();
        }
        args::actions::ActionArgs::Rm { folder, note } => {
            if let Some(note) = note {
                let note = Note::from_pathbuf(&config.data_dir, note)?;
                note.remove(&config)?;
            }

            if let Some(folder) = folder {
                let folder = Folder::from_pathbuf(&config.data_dir, &folder)?;
                folder.remove()?;
            }
        }
        args::actions::ActionArgs::Search {
            content: term,
            folder,
        } => {
            let term = term.to_lowercase();
            debug!("searching for term={term}, folder={folder}");
            let folder = Folder::from_pathbuf(&config.data_dir, folder)?;
            let found_notes = folder.search_notes_content(&term)?;

            match found_notes.is_empty() {
                true => {
                    println!("'{}' {}", term.blue(), "was not found in any notes!".red())
                }
                false => {
                    println!(
                        "Found '{term}' in the following notes:\n\n{}",
                        found_notes
                            .into_iter()
                            .map(|result| format!(
                                "- {}\n{}",
                                result.note.name.blue(),
                                result
                                    .snippets
                                    .into_iter()
                                    .map(|snippet| snippet.to_string())
                                    .join("\n  -----\n")
                            ))
                            .join("\n\n")
                    )
                }
            }
        }
        args::actions::ActionArgs::Sync { setup, folder } => {
            let folder = Folder::from_pathbuf(&config.data_dir, folder)?;
            match setup {
                None => {
                    // just run sync
                    folder.sync_manual(&config)?;
                }
                Some(setup) => match setup {
                    args::sync::actions::SetupSyncArgs::Setup { repo, branch } => {
                        folder.sync_setup(&config, &repo, &branch)?;
                    }
                },
            }
        }
    }

    Ok(())
}
