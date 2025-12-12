use std::{fs, path::PathBuf};

use thiserror::Error;
use tracing::debug;

#[derive(Error, Debug)]
pub enum NoteCreationError {
    #[error("the note {0} already exists!")]
    NoteExists(String),

    #[error("failed to create note: {0}")]
    Create(String),
}

pub fn note_create(root_dir: &PathBuf, name: &String) -> Result<(), NoteCreationError> {
    let mut path = PathBuf::new();

    path.push(root_dir);
    path.push(name);

    if path.exists() {
        return Err(NoteCreationError::NoteExists(name.clone()));
    }

    fs::File::create_new(path.clone()).map_err(|e| NoteCreationError::Create(e.to_string()))?;

    debug!("Created note '{}'", path.to_str().unwrap());
    Ok(())
}
