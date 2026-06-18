use crate::core::models::{note::Note, note_meta::NoteMetaInformation, notebook::Notebook};

impl<'a> Note<'a> {
    pub fn new(path: String, notebook: &'a Notebook, meta: NoteMetaInformation) -> Self {
        Self {
            path,
            notebook,
            meta,
        }
    }
}
