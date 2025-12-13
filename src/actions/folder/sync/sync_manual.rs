use colored::Colorize;
use std::io::Write;

use thiserror::Error;

use crate::{
    actions::folder::{model::Folder, sync::setup::SetupSyncError},
    config::model::Config,
};

#[derive(Error, Debug)]
pub enum SyncManualError {
    #[error("no git repository is configured!")]
    NoGitRepository,

    #[error(transparent)]
    GitFailure(#[from] SetupSyncError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

type Error = SyncManualError;

impl Folder {
    pub fn sync_manual(&self, config: &Config) -> Result<(), SyncManualError> {
        if !self.sync_exists(config) {
            return Err(Error::NoGitRepository);
        }

        print!("Pulling the latest changes for {}... ", self.name.blue());
        std::io::stdout().flush()?;

        self.sync_run_git_command(&["pull"])?;

        println!("{}", "Done".green());

        print!("Pushing up all local changes for {}... ", self.name.blue());
        std::io::stdout().flush()?;

        self.sync_run_git_command(&["add", "-A"])?;
        self.sync_run_git_command_conf(&["commit", "-m", "[nb-rs] Manual Sync"], true)?;
        self.sync_run_git_command(&["push"])?;

        println!("{}", "Done".green());

        Ok(())
    }
}
