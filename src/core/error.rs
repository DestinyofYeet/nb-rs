use thiserror::Error;

use crate::core::{storage_strategy::StorageError, sync_strategy::SyncError};

#[derive(Error, Debug)]
pub enum NbError {
    #[error("NbError: {0}")]
    Nb(String),

    #[error(transparent)]
    Storage(#[from] StorageError),

    #[error(transparent)]
    Sync(#[from] SyncError),
}
