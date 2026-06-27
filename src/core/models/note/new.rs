use crate::core::models::{
    note::Note, note_meta::NoteMetaInformation, note_path::NotePath, notebook::Notebook,
};

impl<'a> Note<'a> {
    pub fn new(path: NotePath, notebook: &'a Notebook, meta: NoteMetaInformation) -> Self {
        Self {
            path,
            notebook,
            meta,
        }
    }
}
