use std::{fs::File, process::Command};

use colored::Colorize;
use thiserror::Error;
use tracing::debug;

use crate::actions::{
    folder::{model::Folder, sync::sync_note::SyncError},
    note::model::Note,
};

#[derive(Error, Debug)]
pub enum OpenNoteError {
    #[error("failed to run editor command: {0}")]
    FailedToRun(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    GitFailure(#[from] SyncError),
}

impl Note {
    pub fn open(&self, editor: &String) -> Result<(), OpenNoteError> {
        let path = self.get_path();

        let file = File::open(&path)?;
        let modified = file.metadata()?.modified()?;
        drop(file);

        let mut process = Command::new(editor);
        process.arg(&path);

        debug!(
            "Executing {:?} with '{:?}'",
            process.get_program(),
            process.get_args()
        );
        process
            .status()
            .map_err(|e| OpenNoteError::FailedToRun(e.to_string()))?;

        let file = File::open(&path)?;
        let new_modified = file.metadata()?.modified()?;

        if modified != new_modified {
            let folder = Folder::from_note(self);
            if folder.sync_exists() {
                folder.sync_note(self)?;
            }
            println!("Updated {}", format!("{}/{}", self.path, self.name).blue());
        }

        Ok(())
    }
}
