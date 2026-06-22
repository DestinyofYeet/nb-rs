use crate::core::models::{note_meta::NoteMetaInformation, notebook::Notebook};

#[derive(Debug, Clone)]
pub struct Note<'a> {
    /// The path in the notebook.
    pub(super) path: String,

    pub(super) notebook: &'a Notebook,

    pub(super) meta: NoteMetaInformation,
}
