use std::path::PathBuf;

use crate::core::{
    NbError,
    models::{note::Note, notebook::Notebook},
};

pub trait NbWrapper {
    fn list_notebooks(&self) -> Result<Vec<Notebook>, NbError>;
    fn create_notebook(&self, name: String) -> Result<(), NbError>;
    fn get_notebook(&self, name: String) -> Result<Option<Notebook>, NbError>;
    fn delete_notebook(&self, notebook: &Notebook) -> Result<(), NbError>;

    fn create_note<'a>(
        &self,
        notebook: &'a mut Notebook,
        title: String,
        path: &'a str,
    ) -> Result<(), NbError>;

    fn get_note<'a>(
        &self,
        notebook: &'a Notebook,
        note_path: &'a str,
    ) -> Result<Option<Note<'a>>, NbError>;

    fn get_note_path_for_editor<'a>(&self, note: &Note<'a>) -> Result<PathBuf, NbError>;

    fn save_note(&self, notebook: &Notebook, note: &Note) -> Result<(), NbError>;

    fn list_notes<'a>(&self, notebook: &'a Notebook) -> Result<Vec<Note<'a>>, NbError>;

    fn delete_note<'a>(
        &self,
        notebook: &'a mut Notebook,
        note_path: &'a str,
    ) -> Result<(), NbError>;
}
