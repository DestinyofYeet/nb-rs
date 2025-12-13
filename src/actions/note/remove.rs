use std::io::Write;

use colored::Colorize;
use thiserror::Error;

use crate::actions::note::model::Note;

#[derive(Error, Debug)]
pub enum RemoveNoteError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("the note {0} does not exist!")]
    NoteDoesNotExist(String),
}

impl Note {
    pub fn remove(&self) -> Result<(), RemoveNoteError> {
        let path = self.get_path();

        if !path.exists() {
            return Err(RemoveNoteError::NoteDoesNotExist(self.name.clone()));
        }

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

        std::fs::remove_file(path)?;

        println!("Removed {}", self.name.blue());

        Ok(())
    }
}
