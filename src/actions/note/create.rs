use std::fs;

use thiserror::Error;
use tracing::debug;

use crate::actions::note::model::Note;

#[derive(Error, Debug)]
pub enum NoteCreationError {
    #[error("the note {0} already exists!")]
    NoteExists(String),

    #[error("failed to create note: {0}")]
    Create(String),
}

impl Note {
    pub(super) fn create(&self) -> Result<(), NoteCreationError> {
        let path = self.get_path();
        if path.exists() {
            return Err(NoteCreationError::NoteExists(self.name.clone()));
        }

        fs::File::create_new(path.clone()).map_err(|e| NoteCreationError::Create(e.to_string()))?;

        debug!("Created note '{}'", path.to_str().unwrap());

        Ok(())
    }
}
