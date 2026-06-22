use std::path::PathBuf;

use crate::core::{
    Nb, NbError,
    models::{note::Note, notebook::Notebook, notebook_meta::NotebookMetaInformation},
    nb_wrapper::NbWrapper,
    storage_strategy::{StorageError, StorageStrategy},
    sync_strategy::SyncStrategy,
};

impl<'n, ST, SY> NbWrapper for Nb<'n, ST, SY>
where
    ST: StorageStrategy,
    SY: SyncStrategy,
{
    fn list_notebooks(&self) -> Result<Vec<Notebook>, NbError> {
        Ok(self.storage.list_notebooks()?)
    }

    fn create_notebook(&self, name: String) -> Result<(), NbError> {
        Ok(self.storage.create_notebook(name)?)
    }

    fn get_notebook(&self, name: String) -> Result<Option<Notebook>, NbError> {
        Ok(self.storage.get_notebook(name)?)
    }

    fn create_note<'a>(
        &self,
        notebook: &'a mut Notebook,
        title: String,
        path: &'a str,
    ) -> Result<(), NbError> {
        Ok(self.storage.create_note(notebook, title, &path.into())?)
    }

    fn get_note<'a>(
        &self,
        notebook: &'a Notebook,
        note_path: &'a str,
    ) -> Result<Option<Note<'a>>, NbError> {
        Ok(self.storage.get_note_by_path(notebook, &note_path.into())?)
    }

    fn get_note_path_for_editor<'a>(&self, note: &Note<'a>) -> Result<PathBuf, NbError> {
        Ok(self.storage.get_note_path_for_editor(note)?)
    }

    fn save_note(&self, notebook: &Notebook, note: &Note) -> Result<(), NbError> {
        self.storage.save_note(notebook, note)?;

        self.sync.sync_note(note);

        Ok(())
    }

    fn list_notes<'a>(&self, notebook: &'a Notebook) -> Result<Vec<Note<'a>>, NbError> {
        Ok(self.storage.list_notes(notebook)?)
    }

    fn delete_note<'a>(
        &self,
        notebook: &'a mut Notebook,
        note_path: &'a str,
    ) -> Result<(), NbError> {
        Ok(self.storage.delete_note(notebook, &note_path.into())?)
    }

    fn delete_notebook(&self, notebook: &Notebook) -> Result<(), NbError> {
        Ok(self.storage.delete_notebook(notebook)?)
    }

    fn setup_sync(&self, notebook: &Notebook) {
        self.sync.setup_sync(notebook);
    }

    fn remove_sync(&self, notebook: &Notebook) {
        self.sync.remove_sync(notebook);
    }

    fn sync_note(&self, note: &Note) {
        self.sync.sync_note(note);
    }
}
