use roxygen::roxygen;
use std::path::PathBuf;

use crate::core::{
    models::{
        note::Note,
        note_meta::NoteMetaInformation,
        note_path::{NoteFilename, NotePath},
        notebook::Notebook,
        notebook_meta::NotebookMetaInformation,
    },
    storage_strategy::{SearchNoteBy, StorageError},
};

pub trait StorageStrategy {
    #[roxygen]
    /// Save notebook metainformation
    fn save_notebook_meta(
        &self,
        /// The full path to the noteboook
        notebook_path: &str,
        /// The meta to save
        meta: &NotebookMetaInformation,
    ) -> Result<(), StorageError>;

    #[roxygen]
    /// Read notebook metainformation
    fn read_notebook_meta(
        &self,
        /// The full path to the notebook
        notebook_path: &str,
    ) -> Result<NotebookMetaInformation, StorageError>;

    /// List all available notebooks
    fn list_notebooks(&self) -> Result<Vec<Notebook>, StorageError>;

    /// Create a notebook
    fn create_notebook(&self, name: String) -> Result<(), StorageError>;

    /// Delete a notebook
    fn delete_notebook(&self, notebook: &Notebook) -> Result<(), StorageError>;

    /// Get a notebook
    fn get_notebook(&self, name: &str) -> Result<Option<Notebook>, StorageError>;

    /// Rename a notebook
    fn rename_notebook(&self, notebook: Notebook, new_name: &str)
    -> Result<Notebook, StorageError>;

    #[roxygen]
    fn rename_note_title(
        &self,
        /// The notebook of the note to rename
        notebook: &mut Notebook,
        /// The note filename
        note_path: &NoteFilename,
        /// The new title
        new_title: &str,
    ) -> Result<(), StorageError>;

    /// List all notes in a notebook
    fn list_notes<'a>(&self, notebook: &'a Notebook) -> Result<Vec<Note<'a>>, StorageError>;

    #[roxygen]
    /// Search for notes in a notebook
    fn search_notes<'a>(
        &self,
        /// The notebook to search in
        notebook: &'a Notebook,
        /// Search criteria
        search_by: &SearchNoteBy,
        tags: &[String],
    ) -> Result<Vec<Note<'a>>, StorageError>;

    /// Get a note in a notebook
    fn get_note_by_path<'a>(
        &self,
        notebook: &'a Notebook,
        note_path: &NoteFilename,
    ) -> Result<Option<Note<'a>>, StorageError>;

    /// Save note metainformation
    fn save_note_meta(
        &self,
        note_path: &NotePath,
        meta: &NoteMetaInformation,
    ) -> Result<(), StorageError>;

    /// Read note metainformation
    fn read_note_meta(&self, note_path: &NotePath) -> Result<NoteMetaInformation, StorageError>;

    /// Create a note
    fn create_note(
        &self,
        notebook: &mut Notebook,
        title: String,
        path: &NoteFilename,
    ) -> Result<(), StorageError>;

    /// Delete a note
    fn delete_note(
        &self,
        notebook: &mut Notebook,
        note_path: &NoteFilename,
    ) -> Result<(), StorageError>;

    /// Save a note, after it was modified by the editor
    fn save_note(&self, notebook: &Notebook, note: &Note) -> Result<(), StorageError>;

    /// Return the path on the local filesystem of a note
    fn get_path_on_fs(
        &self,
        notebook: &Notebook,
        path: &NoteFilename,
    ) -> Result<PathBuf, StorageError>;

    /// Get all filenames in a notebook
    fn list_files(&self, notebook: &Notebook) -> Result<Vec<NoteFilename>, StorageError>;

    /// Returns the path of the metadata file in the notebook
    fn get_notebook_metadata_file(&self, notebook: &Notebook) -> Result<PathBuf, StorageError>;

    /// Returns the path of the metadata file in the note
    fn get_note_metadata_file(&self, note: &Note) -> Result<PathBuf, StorageError>;
}
