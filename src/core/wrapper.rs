use std::{fs::File, path::PathBuf, process::Command};

use colored::Colorize;
use inquire::Select;
use itertools::Itertools;
use tracing::debug;

use crate::core::{
    Nb, NbError,
    models::{note_path::NoteFilename, notebook::Notebook},
    storage_strategy::StorageStrategy,
};

impl Nb {
    pub fn get_path_on_fs(
        &self,
        notebook: &Notebook,
        path: &NoteFilename,
    ) -> Result<PathBuf, NbError> {
        Ok(self.storage.get_path_on_fs(notebook, path)?)
    }

    pub fn get_storage(&self) -> &dyn StorageStrategy {
        &*self.storage
    }

    pub fn interactive_open_note_for_edit(
        &self,
        notebook: &Notebook,
        note_search: &NoteFilename,
        editor: &str,
        do_sync: bool,
    ) -> Result<(), anyhow::Error> {
        let note = match self.get_note(notebook, note_search)? {
            Some(note) => note,
            None => {
                let mut notes = self.list_notes(notebook)?;
                notes.retain(|e| {
                    e.get_title()
                        .to_lowercase()
                        .contains(&note_search.get_filename().to_lowercase())
                });

                match notes.len() {
                    0 => {
                        return Err(anyhow::format_err!(
                            "No notes found with {}",
                            note_search.get_filename().blue()
                        ));
                    }

                    1 => notes.pop().unwrap(),

                    _ => {
                        let notes_string = notes.iter().map(|e| e.get_title()).collect_vec();

                        let note_selection =
                            Select::new("Select a note:", notes_string).prompt()?;

                        let note = match notes.iter().find(|e| e.get_title() == note_selection) {
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

        let note_path = self.get_path_on_fs(notebook, note.get_path().get_filename())?;

        let old_modified = {
            let file = File::open(&note_path)?;
            let modified = file.metadata()?.modified()?;
            drop(file);
            modified
        };

        let mut editor_process = Command::new(editor);
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
            self.save_note(&note, do_sync)?;
        }

        Ok(())
    }
}
