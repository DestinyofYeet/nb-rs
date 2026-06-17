use crate::core::models::{notebook::Notebook, notemeta::NoteMetaInformation};

#[derive(Debug, Clone)]
pub struct Note<'a> {
    /// The path in the notebook.
    pub(super) path: String,

    pub(super) notebook: &'a Notebook,

    pub(super) meta: NoteMetaInformation,
}
