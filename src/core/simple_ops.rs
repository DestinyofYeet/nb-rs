use crate::core::{
    Nb,
    models::notebook::Notebook,
    storage_strategy::{StorageError, StorageStrategy},
    sync_strategy::SyncStrategy,
};

impl<'a, ST, SY> Nb<'a, ST, SY>
where
    ST: StorageStrategy<'a>,
    SY: SyncStrategy,
{
    pub fn list_notebooks(&self) -> Result<Vec<Notebook>, StorageError> {
        self.storage.list_notebooks()
    }

    pub fn create_notebook(&self, name: String) -> Result<(), StorageError> {
        self.storage.create_notebook(name)
    }

    pub fn get_notebook(&self, name: String) -> Result<Option<Notebook>, StorageError> {
        self.storage.get_notebook(name)
    }

    pub fn create_note(
        &self,
        notebook: &'a Notebook,
        title: String,
        path: String,
    ) -> Result<(), StorageError> {
        self.storage.create_note(notebook, title, path)
    }
}
