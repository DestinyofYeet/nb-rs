use crate::core::models::note_path::{NoteFilename, NotePath};

impl NotePath {
    pub fn get_filename(&self) -> &NoteFilename {
        &self.filename
    }

    pub fn get_full_path(&self) -> String {
        format!("{}/{}", self.base_path, self.filename.get_filename())
    }
}

impl NoteFilename {
    pub fn get_filename(&self) -> &str {
        &self.filename
    }
}
