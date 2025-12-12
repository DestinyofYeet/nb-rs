use std::{io::Write, path::PathBuf};

use colored::Colorize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RemoveNoteError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("the note {0} does not exist!")]
    NoteDoesNotExist(String),
}

pub fn note_remove(root: &PathBuf, note: &String) -> Result<(), RemoveNoteError> {
    let mut path = PathBuf::new();
    path.push(root);
    path.push(note);

    if !path.exists() {
        return Err(RemoveNoteError::NoteDoesNotExist(note.clone()));
    }

    let mut input = String::new();

    print!(
        "Are you sure you want to delete '{}'? {}/{} ",
        note.blue(),
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

    println!("Removed {}", note.blue());

    Ok(())
}
