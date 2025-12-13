use thiserror::Error;

use crate::actions::{
    folder::{list::ListFolderError, model::Folder},
    note::model::{Note, NoteError},
};

#[derive(Error, Debug)]
pub enum GetNotesByNameError {
    #[error(transparent)]
    ListFolder(#[from] ListFolderError),

    #[error(transparent)]
    Note(#[from] NoteError),
}

impl Folder {
    pub fn get_notes_by_name(&self, name: &str) -> Result<Vec<Note>, GetNotesByNameError> {
        let entries = self.list()?;

        let mut matching_notes = Vec::new();

        for note in entries.notes.into_iter() {
            if note.get_name()?.to_lowercase().contains(name) {
                matching_notes.push(note);
            }
        }

        for folder in entries.folders.into_iter() {
            matching_notes.append(&mut folder.get_notes_by_name(name)?);
        }

        Ok(matching_notes)
    }
}
