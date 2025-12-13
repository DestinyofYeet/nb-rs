use colored::Colorize;
use std::{
    fs::{self},
    io::{self, Write},
};

use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use tracing::{debug, error};
use tracing_subscriber::EnvFilter;

use crate::{
    actions::{
        folder::{model::Folder, search_notes::SearchNotesError},
        note::model::{Note, NoteError},
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

    match args.action {
        args::actions::ActionArgs::Create { folder, note } => {
            if let Some(folder) = folder {
                let folder = Folder::from_pathbuf(&config.data_dir, &folder)?;
                folder.create()?;
            }

            if let Some(note) = note {
                let note = Note::new_create(config.data_dir.to_str().unwrap(), note)?;
                note.open(&config.editor)?;
            }
        }
        args::actions::ActionArgs::Open { note: note_string } => {
            let note = Note::from_pathbuf(&config.data_dir, note_string.clone());
            let note = match note {
                Ok(note) => note,
                Err(e) => match e {
                    NoteError::NoteDoesNotExist(_) => {
                        let folder = Folder::from_pathbuf(&config.data_dir, ".")?;
                        let notes = folder.get_notes_by_name(&note_string.to_lowercase())?;
                        match notes.len() {
                            1 => notes.get(0).unwrap(),

                            _ => {
                                let string = notes
                                    .iter()
                                    .zip(1..usize::MAX)
                                    .map(|(note, index)| {
                                        format!(
                                            "{}) {}",
                                            index.to_string().green(),
                                            note.get_name().unwrap().blue()
                                        )
                                    })
                                    .join("\n");
                                print!("Please pick a note to edit!\n{}\nInput: ", string);
                                io::stdout().flush()?;

                                let mut input = String::new();

                                std::io::stdin().read_line(&mut input)?;

                                let index = match input.trim().parse::<usize>() {
                                    Ok(index) => index,
                                    Err(e) => {
                                        return Err(SearchNotesError::Convert {
                                            input,
                                            err: e.to_string(),
                                        }
                                        .into());
                                    }
                                };

                                if index == 0 || index > notes.len() {
                                    return Err(SearchNotesError::Index {
                                        input: index,
                                        min: 1,
                                        max: notes.len(),
                                    }
                                    .into());
                                }

                                notes.get(index - 1).unwrap()
                            }
                        }
                        .clone()
                    }

                    _ => return Err(e.into()),
                },
            };

            note.open(&config.editor)?;
        }
        args::actions::ActionArgs::Ls { folder } => {
            let folder = Folder::from_pathbuf(&config.data_dir, folder)?;
            let result = folder.list()?;
            result.print();
        }
        args::actions::ActionArgs::Rm { folder, note } => {
            if let Some(note) = note {
                let note = Note::from_pathbuf(&config.data_dir, note)?;
                note.remove()?;
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
            debug!("searching for content={term}, folder={folder}");
            let folder = Folder::from_pathbuf(&config.data_dir, folder)?;
            let found_notes = folder.search_notes_content(&term)?;

            match found_notes.is_empty() {
                true => {
                    println!("'{}' {}", term.blue(), "was not found in any notes!".red())
                }
                false => {
                    println!(
                        "Found '{term}' in the following notes:\n{}",
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
                            .join("\n")
                    )
                }
            }
        }
    }

    Ok(())
}
