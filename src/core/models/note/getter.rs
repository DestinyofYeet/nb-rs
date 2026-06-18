use crate::core::models::{note::Note, note_meta::NoteMetaInformation};

impl<'a> Note<'a> {
    pub(crate) fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_title(&self) -> &str {
        &self.meta.title
    }

    pub fn get_metadata(&self) -> &NoteMetaInformation {
        &self.meta
    }
}
