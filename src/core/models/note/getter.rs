use std::path::PathBuf;

use crate::core::models::{note::Note, note_meta::NoteMetaInformation};

impl<'a> Note<'a> {
    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_title(&self) -> &str {
        &self.meta.title
    }

    pub fn get_metadata(&self) -> &NoteMetaInformation {
        &self.meta
    }

    pub fn get_file_name(&self) -> String {
        PathBuf::from(&self.path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
    }
}
