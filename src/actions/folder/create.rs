use std::{fs, path::PathBuf};

use thiserror::Error;
use tracing::debug;

#[derive(Error, Debug)]
pub enum FolderCreationError {
    #[error("the folder {0} already exists!")]
    FolderExists(String),

    #[error("failed to create folder: {0}")]
    Create(String),
}

pub fn folder_create(root_dir: &PathBuf, name: &String) -> Result<(), FolderCreationError> {
    let mut path = PathBuf::new();

    path.push(root_dir);
    path.push(name);

    if path.exists() {
        return Err(FolderCreationError::FolderExists(name.clone()));
    }

    fs::create_dir(path.clone()).map_err(|e| FolderCreationError::Create(e.to_string()))?;
    debug!("Created directory '{}'", path.to_str().unwrap());
    Ok(())
}
