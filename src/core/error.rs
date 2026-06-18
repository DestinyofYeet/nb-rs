use thiserror::Error;

use crate::core::storage_strategy::StorageError;

#[derive(Error, Debug)]
pub enum NbError {
    #[error(transparent)]
    StorageError(#[from] StorageError),
}
