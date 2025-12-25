use std::{fs::File, io::Write, process::Command};

use colored::Colorize;
use thiserror::Error;
use tracing::debug;

use crate::{
    actions::{
        folder::{model::Folder, sync::sync_note::SyncError},
        note::model::Note,
    },
    config::model::Config,
};

#[derive(Error, Debug)]
pub enum OpenNoteError {
    #[error("failed to run editor command: {0}")]
    FailedToRun(String),

    #[error(": {0}")]
    Io(
        #[source]
        #[from]
        std::io::Error,
    ),

    #[error(transparent)]
    GitFailure(#[from] SyncError),
}

impl Note {
    pub fn open(&self, config: &Config) -> Result<(), OpenNoteError> {
        let path = self.get_path();

        let file = File::open(&path)?;
        let old_modified = file.metadata()?.modified()?;
        drop(file);

        let mut process = Command::new(&config.editor);
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

        if old_modified != new_modified {
            let folder = Folder::from_note(self);
            println!();
            print!(
                "Updating {}... ",
                format!("{}/{}", self.path, self.name).blue()
            );
            std::io::stdout().flush()?;
            folder.sync_note(self, config)?;
            println!("{}", "Done".green());
        }

        Ok(())
    }
}
