use thiserror::Error;

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("Sync error: {0}")]
    SyncError(String),

    #[error("Failed to setup sync: {0}")]
    SetupSync(String),

    #[error("Failed to remove sync: {0}")]
    RemoveSync(String),

    #[error("Failed to sync: {0}")]
    Sync(String),
}
