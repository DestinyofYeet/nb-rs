use colored::Colorize;
use std::fs;

use thiserror::Error;
use tracing::debug;

use crate::actions::note::model::{Note, NoteError};

#[derive(Error, Debug)]
pub enum NoteCreationError {
    #[error("the note {0} already exists!")]
    NoteExists(String),

    #[error("failed to create note: {0}")]
    Create(String),

    #[error(transparent)]
    Note(#[from] NoteError),
}

impl Note {
    pub(super) fn create(&self) -> Result<(), NoteCreationError> {
        let path = self.get_path();
        if path.exists() {
            return Err(NoteCreationError::NoteExists(self.name.clone()));
        }

        fs::File::create_new(path.clone()).map_err(|e| NoteCreationError::Create(e.to_string()))?;

        debug!("Created note '{}'", path.to_str().unwrap());

        println!("Created note '{}'", self.name.blue());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{actions::note::model::Note, tests::test::Test};

    const TEST_NAME: &str = "create_note";

    #[test]
    fn create_note() {
        let test = Test::setup(TEST_NAME);

        let note = Note::new_create(test.dir.to_str().unwrap(), "test.md").unwrap();

        assert!(note.get_path().exists());
    }
}
