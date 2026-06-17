use crate::core::models::{attachment::Attachment, notebook::Notebook};

pub struct Note<'a> {
    /// The title of the Note.
    /// This will be used to reference it and should be unique.
    title: String,

    /// The path in the notebook.
    path: String,

    notebook: &'a Notebook,

    /// A list of attachments this note references.
    references: Vec<Attachment>,
}
