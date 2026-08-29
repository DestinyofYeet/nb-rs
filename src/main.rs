use clap::{CommandFactory, FromArgMatches, error::ErrorKind};
use colored::Colorize;
use inquire::{Confirm, CustomType};
use itertools::Itertools;
use nb_rs::{
    core::{
        Nb, models::note_path::NoteFilename, storage_strategy::SearchNoteBy,
        sync_strategy::SyncStrategy,
    },
    default_strategies::{
        storage::file_storage::FileStorage,
        sync::{
            AvailableDefaultSyncStrategies,
            git::{GitSync, meta::GitSyncMeta},
        },
    },
};
use tracing::{debug, trace};
use tracing_subscriber::EnvFilter;

use crate::app::{
    args::{ActionArgs, Args, ModifyArgs, SyncArgs},
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

    let level = format!(
        "{},mio=warn",
        match args.verbose {
            0 => "error",
            1 => "info",
            2 => "debug",
            _ => "trace",
        }
    );

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
    trace!("config: {config:?}");

    let nb = Nb::new(FileStorage::new(config.data_dir)?);

    match args.action {
        ActionArgs::Create { notebook, note } => {
            let notebook = match notebook {
                Some(nb) => nb,
                None => return Err(anyhow::format_err!("A notebook has to be provided.")),
            };

            match note {
                Some(note) => {
                    let note = NoteFilename::new(note);

                    let mut notebook = match nb.get_notebook(&notebook)? {
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
                        note.get_filename().blue(),
                        title.blue(),
                        notebook.get_name().blue()
                    ))
                    .prompt()?
                    {
                        println!("{}", "Cancelled".red());
                        return Ok(());
                    }

                    nb.create_note(&mut notebook, title.clone(), &note, config.sync)?;

                    println!(
                        "Created note {} with title {} in notebook {}.",
                        note.get_filename().blue(),
                        title.blue(),
                        notebook.get_name().blue()
                    );

                    nb.interactive_open_note_for_edit(
                        &notebook,
                        &note,
                        &config.editor_cmd,
                        config.sync,
                    )?;
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
            let notebook = match nb.get_notebook(&notebook)? {
                Some(value) => value,
                None => {
                    return Err(anyhow::format_err!(
                        "Failed to find notebook {}",
                        notebook.blue()
                    ));
                }
            };

            nb.interactive_open_note_for_edit(
                &notebook,
                &NoteFilename::new(note),
                &config.editor_cmd,
                config.sync,
            )?;
        }

        ActionArgs::List { notebook } => match notebook {
            None => {
                let notebooks = nb.list_notebooks()?;
                match notebooks.len() {
                    0 => {
                        println!("No notebooks found.")
                    }

                    _ => {
                        println!(
                            "Following notebooks:\n{}",
                            notebooks
                                .iter()
                                .map(|book| book.get_name().blue())
                                .join("\n")
                        )
                    }
                }
            }

            Some(notebook) => {
                let notebook = match nb.get_notebook(&notebook)? {
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
                            notes.iter().map(|note| format!("- {note}")).join("\n")
                        );
                    }
                }
            }
        },

        ActionArgs::Delete { notebook, note } => {
            let mut notebook = match nb.get_notebook(&notebook)? {
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
                    let note = NoteFilename::new(note);

                    let note = match nb.get_note(&notebook, &note)? {
                        Some(note) => note,
                        None => {
                            return Err(anyhow::format_err!(
                                "No note found with name {} in notebook {}",
                                note.get_filename().blue(),
                                notebook.get_name().blue()
                            ));
                        }
                    };

                    let path = note.get_path().clone();

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

                    nb.delete_note(&mut notebook, path.get_filename(), config.sync)?;

                    println!("{}", "Deleted".green());
                }
            }
        }

        ActionArgs::Sync { notebook, action } => {
            let mut notebook = match nb.get_notebook(&notebook)? {
                Some(value) => value,
                None => {
                    println!("Notebook {} not found.", notebook.blue());
                    return Ok(());
                }
            };

            match action {
                SyncArgs::Setup { kind } => {
                    if !Confirm::new(&format!(
                        "Setup {} tracking on notebook {}?",
                        kind.to_string().blue(),
                        notebook.get_name().blue()
                    ))
                    .prompt()?
                    {
                        println!("{}", "Cancelled".red());
                        return Ok(());
                    }

                    let meta = match kind {
                        AvailableDefaultSyncStrategies::Git => {
                            let repo_url = CustomType::<String>::new("Repo url:").prompt()?;
                            let branch = CustomType::<String>::new("Branch:").prompt()?;

                            if !Confirm::new(&format!(
                                "Setup git tracking on {} at branch {}?",
                                repo_url.blue(),
                                branch.blue()
                            ))
                            .prompt()?
                            {
                                println!("{}", "Cancelled".red());
                                return Ok(());
                            }

                            let git_meta = GitSyncMeta::new(repo_url, branch);

                            let git_sync = GitSync::new(git_meta);

                            match git_sync.setup_sync(&notebook, nb.get_storage()) {
                                Ok(meta) => meta,
                                Err(e) => {
                                    debug!("Failed to setup git tracking. Deleting .git folder");
                                    git_sync.remove_sync(&notebook, nb.get_storage())?;
                                    return Err(e.into());
                                }
                            }
                        }
                    };

                    nb.save_sync_setup(&mut notebook, meta)?;
                    println!("{}", "Success".green());
                }

                SyncArgs::Rm {} => {
                    if !Confirm::new(&format!(
                        "Remove tracking on notebook {}?",
                        notebook.get_name().blue()
                    ))
                    .prompt()?
                    {
                        println!("{}", "Cancelled".red());
                        return Ok(());
                    }

                    nb.remove_sync(&mut notebook)?;

                    println!("{}", "Success".green());
                }

                SyncArgs::Full {} => {
                    println!("Fully syncing {}", notebook.get_name().blue());
                    nb.sync_manual(&notebook)?;

                    println!("{}", "Done".green());
                }

                SyncArgs::Import { kind } => {
                    if !notebook.get_meta().get_notes().is_empty() {
                        println!(
                            "Notebook {} must not contain any notes!",
                            notebook.get_name()
                        );
                        return Ok(());
                    }

                    let sync: Box<dyn SyncStrategy> = match kind {
                        AvailableDefaultSyncStrategies::Git => {
                            let repo_url =
                                CustomType::<String>::new("Import from git url:").prompt()?;
                            let branch = CustomType::<String>::new("Import branch:").prompt()?;

                            if !Confirm::new(&format!(
                                "Import git repository from url {} at branch {}?",
                                repo_url.blue(),
                                branch.blue()
                            ))
                            .prompt()?
                            {
                                println!("{}", "Cancelled".red());
                                return Ok(());
                            }

                            let sync = GitSync::new(GitSyncMeta::new(repo_url, branch));

                            Box::new(sync)
                        }
                    };

                    nb.sync_import(sync, &notebook)?;

                    println!("{}", "Done".green());
                }
            }
        }

        ActionArgs::Modify { notebook, action } => {
            let mut notebook = match nb.get_notebook(&notebook)? {
                Some(value) => value,
                None => {
                    println!("Failed to find notebook {}", notebook.blue());
                    return Ok(());
                }
            };

            match action {
                ModifyArgs::Title { note } => match note {
                    Some(note) => {
                        let note = NoteFilename::new(note);
                        match nb.get_note(&notebook, &note)? {
                            Some(note) => {
                                let new_title = CustomType::<String>::new(&format!(
                                    "Current note title: {} | New title:",
                                    note.get_title().blue()
                                ))
                                .prompt()?;

                                if !Confirm::new(&format!("Change title to {}?", new_title.blue()))
                                    .prompt()?
                                {
                                    println!("{}", "Cancelled".red());
                                    return Ok(());
                                }

                                let file_name = note.get_path().clone();
                                drop(note);

                                nb.rename_note_title(
                                    &mut notebook,
                                    file_name.get_filename(),
                                    &new_title,
                                    config.sync,
                                )?;

                                println!("{}", "Done".green());
                            }
                            None => {
                                println!(
                                    "Failed to find note {} in notebook {}.",
                                    note.get_filename().blue(),
                                    notebook.get_name().blue()
                                );
                                return Ok(());
                            }
                        }
                    }
                    None => {
                        let new_name = CustomType::<String>::new(&format!(
                            "Current notebook name: {} | New name:",
                            notebook.get_name().blue(),
                        ))
                        .prompt()?;

                        if !Confirm::new(&format!(
                            "Change notebook name from {} to {}?",
                            notebook.get_name().blue(),
                            new_name.blue()
                        ))
                        .prompt()?
                        {
                            println!("{}", "Cancelled".red());
                            return Ok(());
                        }

                        nb.rename_notebook(notebook, &new_name)?;

                        println!("{}", "Done".green());
                    }
                },
                ModifyArgs::Tags { note } => {
                    let mut note = match note {
                        Some(note) => {
                            let note = NoteFilename::new(note);
                            match nb.get_note(&notebook, &note)? {
                                Some(value) => value,
                                None => {
                                    return Err(anyhow::format_err!(
                                        "Failed to find note {} in notebook {}",
                                        note.get_filename().blue(),
                                        notebook.get_name().blue()
                                    ));
                                }
                            }
                        }

                        None => {
                            return Err(anyhow::format_err!("A note has to be provided!"));
                        }
                    };
                    let mut tags: Vec<String> = note.get_metadata().get_tags().to_vec();

                    {
                        let mut new_tags: Vec<String> = Vec::new();

                        println!("Current tags: {}", tags.iter().map(|e| e.blue()).join(", "));

                        while let tag =
                            CustomType::<String>::new("Tag to add (empty to quit):").prompt()?
                            && !tag.is_empty()
                        {
                            new_tags.push(tag);
                        }

                        if !new_tags.is_empty() {
                            if !Confirm::new(&format!(
                                "Add the following tags?: {}",
                                new_tags.iter().map(|e| e.blue()).join(", ")
                            ))
                            .prompt()?
                            {
                                println!("{}", "Cancelled".red());
                                return Ok(());
                            }

                            for tag in new_tags {
                                tags.push(tag);
                            }
                        }
                    }

                    {
                        let mut remove_tags: Vec<String> = Vec::new();

                        println!("Current tags: {}", tags.iter().map(|e| e.blue()).join(", "));

                        while let tag =
                            CustomType::<String>::new("Tag to remove (empty to quit):").prompt()?
                            && !tag.is_empty()
                        {
                            remove_tags.push(tag);
                        }

                        if !remove_tags.is_empty() {
                            if !Confirm::new(&format!(
                                "Remove the following tags?: {}",
                                remove_tags.iter().map(|e| e.blue()).join(", ")
                            ))
                            .prompt()?
                            {
                                println!("{}", "Cancelled".red());
                                return Ok(());
                            }

                            for tag in remove_tags {
                                if let Some(tag_index) = tags.iter().position(|e| e == &tag) {
                                    tags.remove(tag_index);
                                }
                            }
                        }
                    }

                    println!("Final tags: {}", tags.iter().map(|e| e.blue()).join(","));
                    if !Confirm::new("Save above tags? y/N: ").prompt()? {
                        println!("{}", "Cancelled".red());
                        return Ok(());
                    }

                    note.get_metadata_mut().set_tags(tags);

                    nb.save_note(&note, config.sync)?;
                }
            }
        }
        ActionArgs::Search {
            notebook,
            title,
            filename,
            tags,
            content,
        } => {
            let notebook = match nb.get_notebook(&notebook)? {
                Some(value) => value,
                None => {
                    return Err(anyhow::format_err!(
                        "Failed to find a notebook with name {}",
                        notebook.blue()
                    ));
                }
            };

            let mut search = Vec::new();

            if let Some(filename) = filename {
                search.push(SearchNoteBy::Filename(filename));
            }

            if let Some(title) = title {
                search.push(SearchNoteBy::Title(title));
            }

            if !tags.is_empty() {
                search.push(SearchNoteBy::Tags(tags));
            }

            if !content.is_empty() {
                search.push(SearchNoteBy::Content(content));
            }

            let files = nb.search_notes(&notebook, &search)?;

            let search_criteria_string =
                search.iter().map(|search| search.to_string()).join(" and ");

            if files.is_empty() {
                println!("No notes found matching {search_criteria_string}");
                return Ok(());
            }

            println!(
                "Found the following notes matching {search_criteria_string}:\n{}",
                files.iter().map(|note| format!("- {note}")).join("\n")
            )
        }
    }

    Ok(())
}
