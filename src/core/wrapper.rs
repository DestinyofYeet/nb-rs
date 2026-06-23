use std::path::PathBuf;

use crate::core::{Nb, NbError, models::notebook::Notebook, storage_strategy::StorageStrategy};

impl Nb {
    pub fn get_path_on_fs(&self, notebook: &Notebook, path: &str) -> Result<PathBuf, NbError> {
        Ok(self.storage.get_path_on_fs(notebook, path)?)
    }

    pub fn get_storage(&self) -> &dyn StorageStrategy {
        &*self.storage
    }
}
