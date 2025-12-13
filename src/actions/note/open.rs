use std::{path::PathBuf, process::Command};

use colored::Colorize;
use thiserror::Error;
use tracing::debug;

use crate::actions::note::model::Note;

#[derive(Error, Debug)]
pub enum OpenNoteError {
    #[error("failed to run editor command: {0}")]
    FailedToRun(String),
}

impl Note {
    pub fn open(&self, editor: &String) -> Result<(), OpenNoteError> {
        let path = self.get_path();

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

        println!("Upated {}", format!("{}/{}", self.path, self.name).blue());
        Ok(())
    }
}
