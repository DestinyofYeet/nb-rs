use std::path::PathBuf;

use crate::core::{
    models::{
        note::Note, note_meta::NoteMetaInformation, notebook::Notebook,
        notebook_meta::NotebookMetaInformation,
    },
    storage_strategy::StorageError,
};

pub trait StorageStrategy {
    type StoragePathType<'a>: From<&'a str>;

    /// This function saves notebook metainformation
    fn save_notebook_meta<'a>(
        &self,
        notebook_path: &Self::StoragePathType<'a>,
        meta: &NotebookMetaInformation,
    ) -> Result<(), StorageError>;

    /// This function reads notebook metainformation
    fn read_notebook_meta<'a>(
        &self,
        notebook_path: &Self::StoragePathType<'a>,
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
        note_path: &Self::StoragePathType<'a>,
    ) -> Result<Option<Note<'a>>, StorageError>;

    /// This function saves note metainformation
    fn save_note_meta<'a>(
        &self,
        note_path: &Self::StoragePathType<'a>,
        meta: &NoteMetaInformation,
    ) -> Result<(), StorageError>;

    /// This function reads note metainformation
    fn read_note_meta<'a>(
        &self,
        note_path: &Self::StoragePathType<'a>,
    ) -> Result<NoteMetaInformation, StorageError>;

    /// This function creates a note
    fn create_note<'a>(
        &self,
        notebook: &'a mut Notebook,
        title: String,
        path: &Self::StoragePathType<'a>,
    ) -> Result<(), StorageError>;

    /// This function deletes a note
    fn delete_note<'a>(
        &self,
        notebook: &'a mut Notebook,
        note_path: &Self::StoragePathType<'a>,
    ) -> Result<(), StorageError>;

    /// This function saves a note, after it was modified by the editor
    fn save_note(&self, notebook: &Notebook, note: &Note) -> Result<(), StorageError>;

    /// This function returns the path for the note on the local filesystems.
    /// This will be used to open the editor.
    fn get_note_path_for_editor(&self, note: &Note) -> Result<PathBuf, StorageError>;
}
