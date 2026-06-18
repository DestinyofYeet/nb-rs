use std::path::PathBuf;

use crate::core::{
    Nb, NbError,
    models::{note::Note, notebook::Notebook, notebook_meta::NotebookMetaInformation},
    storage_strategy::{StorageError, StorageStrategy},
    sync_strategy::SyncStrategy,
};

impl<'a, ST, SY> Nb<'a, ST, SY>
where
    ST: StorageStrategy<'a>,
    SY: SyncStrategy,
{
    pub fn list_notebooks(&self) -> Result<Vec<Notebook>, NbError> {
        Ok(self.storage.list_notebooks()?)
    }

    pub fn create_notebook(&self, name: String) -> Result<(), NbError> {
        Ok(self.storage.create_notebook(name)?)
    }

    pub fn get_notebook(&self, name: String) -> Result<Option<Notebook>, NbError> {
        Ok(self.storage.get_notebook(name)?)
    }

    pub fn create_note(
        &self,
        notebook: &'a mut Notebook,
        title: String,
        path: &'a str,
    ) -> Result<(), NbError> {
        Ok(self.storage.create_note(notebook, title, &path.into())?)
    }

    pub fn get_note(
        &self,
        notebook: &'a Notebook,
        note_path: &'a str,
    ) -> Result<Option<Note<'a>>, NbError> {
        Ok(self.storage.get_note_by_path(notebook, &note_path.into())?)
    }

    pub fn get_note_path_for_editor(
        &self,
        note: &super::models::note::Note<'_>,
    ) -> Result<PathBuf, NbError> {
        Ok(self.storage.get_note_path_for_editor(note)?)
    }

    pub fn save_note(&self, notebook: &'a Notebook, note: &Note) -> Result<(), NbError> {
        self.storage.save_note(notebook, note)?;

        Ok(())
    }

    pub fn list_notes(&self, notebook: &'a Notebook) -> Result<Vec<Note<'a>>, NbError> {
        Ok(self.storage.list_notes(notebook)?)
    }
}
