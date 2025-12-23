use std::path::PathBuf;

use thiserror::Error;
use tracing::debug;

use crate::{
    actions::{
        folder::{
            model::{Folder, FolderError},
            sync::setup::SetupSyncError,
        },
        note::model::Note,
    },
    config::model::Config,
};

#[derive(Error, Debug)]
pub enum SyncError {
    #[error(transparent)]
    GitFailure(#[from] SetupSyncError),

    #[error(transparent)]
    Folder(#[from] FolderError),

    #[error("failed to find git folder. Reached root dir")]
    GitNotFound,
}

type Error = SyncError;

impl Folder {
    pub fn sync_note(&self, note: &Note, config: &Config) -> Result<(), Error> {
        if !self.sync_exists(config) {
            return Ok(());
        }
        let folder_path = self.get_path();

        let mut stripped_path = Vec::new();

        let mut git_root = folder_path;

        let root_data_dir = PathBuf::from(&config.data_dir);

        loop {
            if git_root == root_data_dir {
                return Err(SyncError::GitNotFound);
            }
            let mut tmp = git_root.clone();
            tmp.push(".git");

            if tmp.exists() {
                break;
            }

            let base = git_root.file_name().unwrap().to_str().unwrap().to_string();
            stripped_path.push(base);
            git_root.pop();
        }

        debug!("git root found: {}", git_root.to_str().unwrap());
        debug!("stripped_path: {:#?}", stripped_path);

        let note_name = match stripped_path.is_empty() {
            false => format!("{}/{}", stripped_path.join("/"), note.name),
            true => note.name.clone(),
        };

        let git_root_folder = Folder::from_pathbuf(&git_root, ".")?;

        debug!("Syncing {}", note_name);
        git_root_folder.sync_run_git_command(&["add", &note_name])?;
        git_root_folder.sync_run_git_command_conf(
            &["commit", "-m", &format!("[nb-rs] Edit: {}", note.name)],
            true,
        )?;
        git_root_folder.sync_run_git_command(&["push"])?;
        Ok(())
    }
}
