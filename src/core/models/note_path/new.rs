use crate::core::models::note_path::{NoteFilename, NotePath};

impl NotePath {
    pub fn new(filename: NoteFilename, base_path: String) -> Self {
        Self {
            filename,
            base_path,
        }
    }
}

impl NoteFilename {
    pub fn new(filename: String) -> Self {
        Self { filename }
    }
}
