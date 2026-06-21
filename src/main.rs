use std::{fs::File, process::Command};

use clap::{CommandFactory, FromArgMatches, error::ErrorKind};
use colored::Colorize;
use inquire::{Confirm, CustomType, Select};
use itertools::Itertools;
use nb_rs::{
    core::{Nb, nb_wrapper::NbWrapper},
    default_strategies::{storage::file_storage::FileStorage, sync::git::GitSync},
};
use tracing::{debug, trace};
use tracing_subscriber::EnvFilter;

use crate::app::{
    args::{ActionArgs, Args},
    config::Config,
};

mod app;

pub static GIT_REV: &str = env!("GIT_REV");

fn print_version() {
    println!("Compiled at git rev {}", GIT_REV.blue());
}

pub fn main() -> anyhow::Result<()> {
    let arg_matches = match Args::command().try_get_matches() {
        Ok(value) => value,
        Err(e) => {
            if e.kind() == ErrorKind::MissingSubcommand
                && std::env::args().any(|a| a == "--version")
            {
                print_version();
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

    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_env_filter(EnvFilter::new(level))
        .init();

    debug!("Debug log enabled.");
    trace!("Trace log enabled.");

    if args.version {
        print_version();
        return Ok(());
    }

    let config = Config::new(&args)?;

    debug!("data_dir: {:?}", config.data_dir);

    let nb: Box<dyn NbWrapper> = Box::new(Nb::new(FileStorage::new(config.data_dir)?, GitSync {}));
    // let nb = Box::leak(nb);

    match args.action {
        ActionArgs::Create { notebook, note } => {
            let notebook = match notebook {
                Some(nb) => nb,
                None => return Err(anyhow::format_err!("A notebook has to be provided.")),
            };

            match note {
                Some(note) => {
                    let mut notebook = match nb.get_notebook(notebook.clone())? {
                        Some(value) => value,
                        None => {
                            return Err(anyhow::format_err!(
                                "The notebook {notebook} does not exist!"
                            ));
                        }
                    };

                    let title = CustomType::<String>::new("Title for the new note:").prompt()?;

                    if !Confirm::new(&format!(
                        "Create note {} with title {} in notebook {}?",
                        note.blue(),
                        title.blue(),
                        notebook.get_name().blue()
                    ))
                    .prompt()?
                    {
                        println!("{}", "Cancelled".red());
                        return Ok(());
                    }

                    nb.create_note(&mut notebook, title.clone(), &note)?;

                    println!(
                        "Created note {} with title {} in notebook {}.",
                        note.blue(),
                        title.blue(),
                        notebook.get_name().blue()
                    );
                }

                None => {
                    if !Confirm::new(&format!("Create notebook {}?", notebook.blue())).prompt()? {
                        println!("{}", "Cancelled".red());
                        return Ok(());
                    }

                    nb.create_notebook(notebook.clone())?;
                    println!("Created notebook {}.", notebook.blue());
                }
            }
        }

        ActionArgs::Open { notebook, note } => {
            let notebook = match nb.get_notebook(notebook.clone())? {
                Some(value) => value,
                None => {
                    return Err(anyhow::format_err!(
                        "Failed to find notebook {}",
                        notebook.blue()
                    ));
                }
            };

            let note = match nb.get_note(&notebook, &note)? {
                Some(note) => note,
                None => {
                    let mut notes = nb.list_notes(&notebook)?;
                    notes.retain(|e| e.get_title().to_lowercase().contains(&note.to_lowercase()));

                    match notes.len() {
                        0 => {
                            return Err(anyhow::format_err!("No notes found with {}", note.blue()));
                        }

                        1 => notes.pop().unwrap(),

                        _ => {
                            let notes_string = notes.iter().map(|e| e.get_title()).collect_vec();

                            let note_selection =
                                Select::new("Select a note:", notes_string).prompt()?;

                            let note = match notes.iter().find(|e| e.get_title() == note_selection)
                            {
                                Some(note) => note,
                                None => {
                                    return Err(anyhow::format_err!(
                                        "Failed to find note {}",
                                        note_selection.blue()
                                    ));
                                }
                            };

                            note.clone()
                        }
                    }
                }
            };

            let note_path = nb.get_note_path_for_editor(&note)?;

            let old_modified = {
                let file = File::open(&note_path)?;
                let modified = file.metadata()?.modified()?;
                drop(file);
                modified
            };

            let mut editor_process = Command::new(config.editor_cmd);
            editor_process.arg(&note_path);

            debug!(
                "Executing {:?} with args {:?}",
                editor_process.get_program(),
                editor_process.get_args()
            );

            editor_process.status()?;

            let new_modified = {
                let file = File::open(&note_path)?;
                let modified = file.metadata()?.modified()?;
                drop(file);
                modified
            };

            if new_modified != old_modified {
                nb.save_note(&notebook, &note)?;
            }
        }

        ActionArgs::List { notebook } => {
            let notebook = match nb.get_notebook(notebook.clone())? {
                Some(value) => value,
                None => {
                    return Err(anyhow::format_err!(
                        "No notebook found with name {}",
                        notebook.blue()
                    ));
                }
            };

            let notes = nb.list_notes(&notebook)?;

            match notes.len() {
                0 => {
                    println!(
                        "There are no notes in the notebook {}",
                        notebook.get_name().blue()
                    );
                }

                _ => {
                    println!(
                        "Following notes are in the notebook {}:\n{}",
                        notebook.get_name().blue(),
                        notes
                            .iter()
                            .map(|note| format!(
                                "- {} {}",
                                note.get_title().blue(),
                                format!("({})", note.get_file_name()).white()
                            ))
                            .join("\n")
                    );
                }
            }
        }

        ActionArgs::Delete { notebook, note } => {
            let mut notebook = match nb.get_notebook(notebook.clone())? {
                Some(value) => value,
                None => {
                    return Err(anyhow::format_err!(
                        "No notebook found with name {}",
                        notebook.blue()
                    ));
                }
            };

            match note {
                None => {
                    if !Confirm::new(&format!("Delete notebook {}?", notebook.get_name().blue()))
                        .prompt()?
                    {
                        println!("{}", "Cancelled".red());
                        return Ok(());
                    }

                    nb.delete_notebook(&notebook)?;

                    println!("{}", "Deleted".green());
                }

                Some(note) => {
                    let note = match nb.get_note(&notebook, &note)? {
                        Some(note) => note,
                        None => {
                            return Err(anyhow::format_err!(
                                "No note found with name {} in notebook {}",
                                note.blue(),
                                notebook.get_name().blue()
                            ));
                        }
                    };

                    let path = note.get_path().to_string();

                    if !Confirm::new(&format!(
                        "Delete note {} in notebook {}",
                        note.get_title().blue(),
                        notebook.get_name().blue()
                    ))
                    .prompt()?
                    {
                        println!("{}", "Cancelled".red());
                        return Ok(());
                    }

                    drop(note);

                    nb.delete_note(&mut notebook, &path)?;

                    println!("{}", "Deleted".green());
                }
            }
        }
    }

    Ok(())
}
