use crate::core::models::{
    note_meta::NoteMetaInformation, note_path::NotePath, notebook::Notebook,
};

#[derive(Debug, Clone)]
pub struct Note<'a> {
    /// The path in the notebook.
    pub(super) path: NotePath,

    pub(super) notebook: &'a Notebook,

    pub(super) meta: NoteMetaInformation,
}
