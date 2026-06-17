use clap::{CommandFactory, FromArgMatches, error::ErrorKind};
use colored::Colorize;
use inquire::{Confirm, CustomType, InputAction};
use nb_rs::{
    core::Nb,
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

    let nb = Nb::new(FileStorage::new(config.data_dir)?, GitSync {});

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

                    nb.create_note(&mut notebook, title.clone(), note.clone())?;

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
    }

    Ok(())
}
