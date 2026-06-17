use std::path::PathBuf;

use crate::core::{
    models::{note::Note, notebook::Notebook, notemeta::NoteMetaInformation},
    storage_strategy::StorageError,
};

pub trait StorageStrategy<'a> {
    /// This function lists all available notebooks
    fn list_notebooks(&self) -> Result<Vec<Notebook>, StorageError>;

    /// This function creates a notebook
    fn create_notebook(&self, name: String) -> Result<(), StorageError>;

    /// This function deletes a notebook
    fn delete_notebook(&self, notebook: &'a Notebook) -> Result<(), StorageError>;

    /// This function gets a notebook
    fn get_notebook(&self, name: String) -> Result<Option<Notebook>, StorageError>;

    /// This function lists all notes in a notebook
    fn list_notes(&self, notebook: &'a Notebook) -> Result<Vec<Note<'a>>, StorageError>;

    /// This function searches for notes in a notebook
    fn search_notes(
        &self,
        notebook: &'a Notebook,
        search_term: String,
    ) -> Result<Vec<Note<'a>>, StorageError>;

    /// This function saves note metainformation
    fn save_note_meta(
        &self,
        note_path: &str,
        meta: &NoteMetaInformation,
    ) -> Result<(), StorageError>;

    /// This function reads note metainformation
    fn read_note_meta(&self, note_path: &str) -> Result<NoteMetaInformation, StorageError>;

    /// This function creates a note
    fn create_note(
        &self,
        notebook: &'a Notebook,
        title: String,
        path: String,
    ) -> Result<(), StorageError>;

    /// This function deletes a note
    fn delete_note(&self, note: &Note) -> Result<(), StorageError>;

    /// This function saves a note, after it was modified by the editor
    fn save_note(&self, note: &Note) -> Result<(), StorageError>;

    /// This function returns the path for the note on the local filesystems.
    /// This will be used to open the editor.
    fn get_note_path_for_editor(&self, note: &Note) -> Result<PathBuf, StorageError>;
}
