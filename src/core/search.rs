use crate::core::{
    Nb, NbError,
    models::{note::Note, notebook::Notebook},
    storage_strategy::{SearchNoteBy, StorageError, StorageStrategy},
};

impl Nb {
    pub fn search_notes<'a>(
        &self,
        notebook: &'a Notebook,
        search_by: &SearchNoteBy,
        tags: &[String],
    ) -> Result<Vec<Note<'a>>, NbError> {
        Ok(self.storage.search_notes(notebook, search_by, tags)?)
    }
}
