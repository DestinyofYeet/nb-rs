use crate::core::{Nb, NbError, models::notebook::Notebook};

impl Nb {
    pub fn list_notebooks(&self) -> Result<Vec<Notebook>, NbError> {
        Ok(self.storage.list_notebooks()?)
    }

    pub fn create_notebook(&self, name: String) -> Result<(), NbError> {
        Ok(self.storage.create_notebook(name)?)
    }

    pub fn get_notebook(&self, name: &str) -> Result<Option<Notebook>, NbError> {
        Ok(self.storage.get_notebook(name)?)
    }

    pub fn delete_notebook(&self, notebook: &Notebook) -> Result<(), NbError> {
        Ok(self.storage.delete_notebook(notebook)?)
    }

    pub fn rename_notebook(&self, notebook: Notebook, new_name: &str) -> Result<Notebook, NbError> {
        Ok(self.storage.rename_notebook(notebook, new_name)?)
    }
}
