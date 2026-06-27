use std::path::PathBuf;

use crate::core::{
    models::{
        note::Note,
        note_meta::NoteMetaInformation,
        note_path::{NoteFilename, NotePath},
        notebook::Notebook,
        notebook_meta::NotebookMetaInformation,
    },
    storage_strategy::StorageError,
};

pub trait StorageStrategy {
    /// This function saves notebook metainformation
    fn save_notebook_meta(
        &self,
        notebook_path: &str,
        meta: &NotebookMetaInformation,
    ) -> Result<(), StorageError>;

    /// This function reads notebook metainformation
    fn read_notebook_meta(
        &self,
        notebook_path: &str,
    ) -> Result<NotebookMetaInformation, StorageError>;

    /// This function lists all available notebooks
    fn list_notebooks(&self) -> Result<Vec<Notebook>, StorageError>;

    /// This function creates a notebook
    fn create_notebook(&self, name: String) -> Result<(), StorageError>;

    /// This function deletes a notebook
    fn delete_notebook(&self, notebook: &Notebook) -> Result<(), StorageError>;

    /// This function gets a notebook
    fn get_notebook(&self, name: &str) -> Result<Option<Notebook>, StorageError>;

    fn rename_notebook(&self, notebook: Notebook, new_name: &str)
    -> Result<Notebook, StorageError>;

    /// * `note_path`: The path in the notebook
    fn rename_note_title(
        &self,
        notebook: &mut Notebook,
        note_path: &NoteFilename,
        new_title: &str,
    ) -> Result<(), StorageError>;

    /// This function lists all notes in a notebook
    fn list_notes<'a>(&self, notebook: &'a Notebook) -> Result<Vec<Note<'a>>, StorageError>;

    /// This function searches for notes in a notebook
    fn search_notes<'a>(
        &self,
        notebook: &'a Notebook,
        search_term: String,
    ) -> Result<Vec<Note<'a>>, StorageError>;

    /// This function gets a note in a notebook
    fn get_note_by_path<'a>(
        &self,
        notebook: &'a Notebook,
        note_path: &NoteFilename,
    ) -> Result<Option<Note<'a>>, StorageError>;

    /// This function saves note metainformation
    fn save_note_meta(
        &self,
        note_path: &NotePath,
        meta: &NoteMetaInformation,
    ) -> Result<(), StorageError>;

    /// This function reads note metainformation
    fn read_note_meta(&self, note_path: &NotePath) -> Result<NoteMetaInformation, StorageError>;

    /// This function creates a note
    fn create_note(
        &self,
        notebook: &mut Notebook,
        title: String,
        path: &NoteFilename,
    ) -> Result<(), StorageError>;

    /// This function deletes a note
    fn delete_note(
        &self,
        notebook: &mut Notebook,
        note_path: &NoteFilename,
    ) -> Result<(), StorageError>;

    /// This function saves a note, after it was modified by the editor
    fn save_note(&self, notebook: &Notebook, note: &Note) -> Result<(), StorageError>;

    /// This function returns a path on the local filesystem
    fn get_path_on_fs(
        &self,
        notebook: &Notebook,
        path: &NoteFilename,
    ) -> Result<PathBuf, StorageError>;

    /// This function gets all filenames in a notebook
    fn list_files(&self, notebook: &Notebook) -> Result<Vec<NoteFilename>, StorageError>;

    /// Returns the path of the metadata file in the notebook
    fn get_notebook_metadata_file(&self, notebook: &Notebook) -> Result<PathBuf, StorageError>;

    /// Returns the path of the metadata file in the note
    fn get_note_metadata_file(&self, note: &Note) -> Result<PathBuf, StorageError>;
}
