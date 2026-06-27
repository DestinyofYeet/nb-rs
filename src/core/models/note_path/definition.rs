#[derive(Debug, Clone)]
pub struct NotePath {
    /// This is the filename of the note
    pub(super) filename: NoteFilename,

    /// This is the path of the note without the filename
    /// For the full path, this will be prepended to the `filename` with a '/'
    pub(super) base_path: String,
}

#[derive(Debug, Clone)]
pub struct NoteFilename {
    pub(super) filename: String,
}
