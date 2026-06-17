#[derive(Debug, Clone)]
pub struct Attachment {
    /// The name of the attachment.
    /// This should be unique
    name: String,
    /// The path of the attachment in the Notebook
    path: String,
}
