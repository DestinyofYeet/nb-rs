use std::path::PathBuf;

use crate::core::{
    models::notebook::Notebook,
    storage_strategy::{StorageError, StorageStrategy},
};

pub struct FileStorage {
    data_dir: PathBuf,
}

impl FileStorage {
    pub fn new(data_dir: PathBuf) -> Result<Self, StorageError> {
        if !data_dir.exists() {
            std::fs::create_dir(&data_dir).map_err(|e| {
                StorageError::StorageError(format!("Failed to create base directory: {e}"))
            })?;
        }

        Ok(Self { data_dir })
    }
}

impl<'a> StorageStrategy<'a> for FileStorage {
    fn list_notebooks(
        &self,
    ) -> Result<
        Vec<crate::core::models::notebook::Notebook>,
        crate::core::storage_strategy::StorageError,
    > {
        todo!()
    }

    fn create_notebook(
        &self,
        name: String,
    ) -> Result<(), crate::core::storage_strategy::StorageError> {
        let new_path = self.data_dir.join(&name);

        if new_path.exists() {
            return Err(StorageError::CreateNotebook(format!(
                "The notebook '{name}' already exists."
            )));
        }

        std::fs::create_dir(new_path).map_err(|e| {
            StorageError::CreateNotebook(format!("Failed to create directory: {e}"))
        })?;

        Ok(())
    }

    fn delete_notebook(
        &self,
        notebook: &'a crate::core::models::notebook::Notebook,
    ) -> Result<(), crate::core::storage_strategy::StorageError> {
        todo!()
    }

    fn get_notebook(&self, name: String) -> Result<Option<Notebook>, StorageError> {
        let maybe_path = self.data_dir.join(&name);

        if maybe_path.is_dir() {
            return Ok(Some(Notebook::new(name, maybe_path)));
        }

        Ok(None)
    }

    fn list_notes(
        &self,
        notebook: &'a crate::core::models::notebook::Notebook,
    ) -> Result<Vec<crate::core::models::note::Note<'a>>, crate::core::storage_strategy::StorageError>
    {
        todo!()
    }

    fn search_notes(
        &self,
        notebook: &'a crate::core::models::notebook::Notebook,
        search_term: String,
    ) -> Result<Vec<crate::core::models::note::Note<'a>>, crate::core::storage_strategy::StorageError>
    {
        todo!()
    }

    fn create_note(
        &self,
        notebook: &'a Notebook,
        title: String,
        path: String,
    ) -> Result<(), crate::core::storage_strategy::StorageError> {
        todo!()
    }

    fn delete_note(
        &self,
        note: &crate::core::models::note::Note,
    ) -> Result<(), crate::core::storage_strategy::StorageError> {
        todo!()
    }

    fn save_note(
        &self,
        note: &crate::core::models::note::Note,
    ) -> Result<(), crate::core::storage_strategy::StorageError> {
        todo!()
    }

    fn get_note_path_for_editor(
        &self,
        note: &crate::core::models::note::Note,
    ) -> Result<std::path::PathBuf, crate::core::storage_strategy::StorageError> {
        todo!()
    }

    fn save_note_meta(
        &self,
        note_path: &str,
        meta: &crate::core::models::notemeta::NoteMetaInformation,
    ) -> Result<(), StorageError> {
        todo!()
    }

    fn read_note_meta(
        &self,
        note_path: &str,
    ) -> Result<crate::core::models::notemeta::NoteMetaInformation, StorageError> {
        todo!()
    }
}
