use std::io::Write;

use colored::Colorize;
use thiserror::Error;

use crate::{
    actions::{
        folder::{model::Folder, sync::sync_note::SyncError},
        note::model::Note,
    },
    config::model::Config,
};

#[derive(Error, Debug)]
pub enum RemoveNoteError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("the note {0} does not exist!")]
    NoteDoesNotExist(String),

    #[error(transparent)]
    Sync(#[from] SyncError),
}

impl Note {
    pub fn remove(&self, config: &Config) -> Result<(), RemoveNoteError> {
        let path = self.get_path();

        if !path.exists() {
            return Err(RemoveNoteError::NoteDoesNotExist(self.name.clone()));
        }

        if !config.is_test {
            let mut input = String::new();

            print!(
                "Are you sure you want to delete '{}'? {}/{} ",
                self.name.blue(),
                "y".red(),
                "N".green()
            );

            std::io::stdout().flush()?;

            std::io::stdin().read_line(&mut input)?;

            if input.trim() != "y" {
                println!("Aborting!");
                return Ok(());
            }
        }

        std::fs::remove_file(path)?;

        let folder = Folder::from_note(self);
        folder.sync_note(self, config)?;

        println!("Removed {}", self.name.blue());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{actions::note::model::Note, tests::test::Test};

    const TEST_NAME: &str = "delete_note";

    #[test]
    pub fn delete_note() {
        let test = Test::setup(TEST_NAME);

        let note = Note::new_create(test.dir.to_str().unwrap(), "test.md").unwrap();

        assert!(note.get_path().exists());

        note.remove(&test.config).unwrap();

        let exists = note.get_path().exists();

        println!("exists: {exists}");

        assert!(!exists);
    }
}
