use std::path::PathBuf;

use crate::core::{
    models::{
        note::Note, note_meta::NoteMetaInformation, notebook::Notebook,
        notebook_meta::NotebookMetaInformation,
    },
    storage_strategy::StorageError,
};

pub trait StorageStrategy {
    /// This function saves notebook metainformation
    fn save_notebook_meta<'a>(
        &self,
        notebook_path: &str,
        meta: &NotebookMetaInformation,
    ) -> Result<(), StorageError>;

    /// This function reads notebook metainformation
    fn read_notebook_meta<'a>(
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
    fn get_notebook(&self, name: String) -> Result<Option<Notebook>, StorageError>;

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
        note_path: &str,
    ) -> Result<Option<Note<'a>>, StorageError>;

    /// This function saves note metainformation
    fn save_note_meta<'a>(
        &self,
        note_path: &str,
        meta: &NoteMetaInformation,
    ) -> Result<(), StorageError>;

    /// This function reads note metainformation
    fn read_note_meta<'a>(&self, note_path: &str) -> Result<NoteMetaInformation, StorageError>;

    /// This function creates a note
    fn create_note<'a>(
        &self,
        notebook: &'a mut Notebook,
        title: String,
        path: &str,
    ) -> Result<(), StorageError>;

    /// This function deletes a note
    fn delete_note<'a>(
        &self,
        notebook: &'a mut Notebook,
        note_path: &str,
    ) -> Result<(), StorageError>;

    /// This function saves a note, after it was modified by the editor
    fn save_note(&self, notebook: &Notebook, note: &Note) -> Result<(), StorageError>;

    /// This function returns a path on the local filesystem
    fn get_path_on_fs<'a>(&self, notebook: &Notebook, path: &str) -> Result<PathBuf, StorageError>;

    /// This function returns a path to the root of the data dir
    fn get_root_path_on_fs(&self) -> Result<PathBuf, StorageError>;

    /// This function gets all filenames in a notebook
    fn list_files(&self, notebook: &Notebook) -> Result<Vec<String>, StorageError>;
}
