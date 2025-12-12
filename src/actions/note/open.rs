use std::{path::PathBuf, process::Command};

use thiserror::Error;
use tracing::debug;

#[derive(Error, Debug)]
pub enum OpenNoteError {
    #[error("note '{0}' not found!")]
    NotFound(String),

    #[error("failed to run editor command: {0}")]
    FailedToRun(String),
}

pub fn note_open(root: &PathBuf, name: &String, editor: &String) -> Result<(), OpenNoteError> {
    let mut path = PathBuf::new();

    path.push(root);
    path.push(name);

    if !path.exists() {
        return Err(OpenNoteError::NotFound(path.to_str().unwrap().to_string()));
    }

    let mut process = Command::new(editor);
    process.arg(path);

    debug!(
        "Executing {:?} with '{:?}'",
        process.get_program(),
        process.get_args()
    );
    process
        .status()
        .map_err(|e| OpenNoteError::FailedToRun(e.to_string()))?;

    Ok(())
}
