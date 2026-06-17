use thiserror::Error;

#[rustfmt::skip]
#[derive(Debug, Error)]
pub enum StorageError {

    #[error("Storage error: {0}")]
    StorageError(String),

    
    #[error("Failed to list notebooks: {0}")]
    ListNotebooks(String),

    #[error("Failed to create notebook: {0}")]
    CreateNotebook(String),

    #[error("Failed to delete notebook: {0}")]
    DeleteNotebook(String),

    #[error("Failed to get notebook: {0}")]
    GetNotebook(String),


    #[error("Failed to list notes: {0}")]
    ListNotes(String),

    #[error("Failed to create note: {0}")]
    CreateNote(String),

    #[error("Failed to delete note: {0}")]
    DeleteNote(String),

    #[error("Failed to get note path: {0}")]
    GetNotePathForEditor(String),

    #[error("Failed to save note: {0}")]
    SaveNote(String)
}
