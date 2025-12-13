use std::fs;

use colored::Colorize;
use itertools::Itertools;
use thiserror::Error;
use tracing::debug;

use crate::actions::{
    folder::model::{Folder, FolderError},
    note::model::{Note, NoteError},
};

#[derive(Error, Debug)]
pub enum ListFolderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    NoteError(#[from] NoteError),

    #[error(transparent)]
    Folder(#[from] FolderError),
}

#[derive(Default)]
pub struct FolderSearchResult {
    pub folders: Vec<Folder>,
    pub notes: Vec<Note>,
}

impl FolderSearchResult {
    pub fn print(&self) {
        let folders_string = self
            .folders
            .iter()
            .map(|folder| format!("- {}: {}", "D".green(), folder.name.blue()))
            .join("\n");
        let notes_string = self
            .notes
            .iter()
            .map(|note| {
                let name = {
                    let mut name = String::new();
                    if let Some(pretty) = note.get_pretty_name().unwrap() {
                        name.push_str(&format!("{} | ", pretty.blue()));
                    }
                    name.push_str(&format!("{}", note.name.blue()));

                    name
                };
                format!("- {}: {}", "F".green(), name)
            })
            .join("\n");

        let entries = self.folders.len() + self.notes.len();

        println!(
            "Found {} entries!\n{}\n{}",
            entries, folders_string, notes_string
        );
    }
}

impl Folder {
    pub fn list(&self) -> Result<FolderSearchResult, ListFolderError> {
        let path = self.get_path();

        debug!("Iterating folder {:?}", self);

        let mut folders: Vec<Folder> = Vec::new();
        let mut notes: Vec<Note> = Vec::new();

        for entry in fs::read_dir(path.clone())? {
            let entry = entry?;
            let entry_type = entry.file_type()?;
            let entry_name = entry.file_name();

            if entry_type.is_dir() {
                let folder = Folder::from_pathbuf(&path, entry_name.to_str().unwrap())?;
                if folder.name.starts_with(".") {
                    continue;
                }

                folders.push(folder);
            } else if entry_type.is_file() {
                let note = Note::new(
                    self.get_path().to_str().unwrap(),
                    entry_name.to_str().unwrap(),
                )?;
                notes.push(note);
            }
        }

        Ok(FolderSearchResult { folders, notes })
    }
}
