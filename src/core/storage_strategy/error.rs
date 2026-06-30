use thiserror::Error;

#[rustfmt::skip]
#[derive(Debug, Error)]
pub enum StorageError {

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Failed to rename notebook: {0}")]
    Rename(String),
    
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
    SaveNote(String),

    #[error("Failed to write note meta: {0}")]
    SaveNoteMeta(String),

    #[error("Failed to read note meta: {0}")]
    ReadNoteMeta(String),

    #[error("Failed to save notebook meta: {0}")]
    SaveNotebookMeta(String),

    #[error("Failed to read notebook meta: {0}")]
    ReadNotebookMeta(String),

    #[error("Failed to get path on fs: {0}")]
    PathOnFs(String),

    #[error("Failed to list files: {0}")]
    ListFiles(String),

    #[error("Failed to search files: {0}")]
    Search(String),
}
