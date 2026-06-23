use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use itertools::Itertools;
use tracing::debug;

use crate::{
    core::{
        storage_strategy::StorageStrategy,
        sync_strategy::{SyncError, SyncStrategy, meta::SyncMetaInformation},
    },
    default_strategies::sync::git::meta::GitSyncMeta,
};

pub struct GitSync {
    pub(super) meta: GitSyncMeta,
}

struct GitCommand<'a> {
    can_fail: bool,
    cwd: Option<&'a str>,
    args: &'a [&'a str],
}

impl<'a> GitCommand<'a> {
    pub fn new(cwd: Option<&'a str>, args: &'a [&'a str]) -> Self {
        Self {
            can_fail: false,
            cwd,
            args,
        }
    }

    pub fn set_failable(mut self, can_fail: bool) -> Self {
        self.can_fail = can_fail;
        self
    }
}

impl GitSync {
    pub fn new(meta: GitSyncMeta) -> Self {
        Self { meta }
    }

    fn run_git_command(&self, git_command: GitCommand) -> Result<(), SyncError> {
        let mut command = Command::new("git");

        command
            .args(git_command.args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if let Some(cwd) = git_command.cwd.as_ref() {
            command.current_dir(cwd);
        }

        debug!("Running git command: {:?}", command.get_args());

        let output = command
            .output()
            .map_err(|e| SyncError::Sync(format!("Failed to get process output: {e}")))?;

        if !output.status.success() && !git_command.can_fail {
            let stdout = String::from_utf8(output.stdout).expect("to parse stdout");
            let stderr = String::from_utf8(output.stderr).expect("to parse stderr");

            return Err(SyncError::Sync(format!(
                "Failed to run git command!\nStdout:\n{stdout}\n\nStderr:\n{stderr}\n\n"
            )));
        }

        Ok(())
    }
}

impl SyncStrategy for GitSync {
    fn setup_sync(
        &self,
        notebook: &crate::core::models::notebook::Notebook,
        storage: &dyn StorageStrategy,
    ) -> Result<SyncMetaInformation, crate::core::sync_strategy::SyncError> {
        let path = notebook.get_path();
        self.run_git_command(GitCommand::new(
            path.into(),
            &["init", "-b", &self.meta.branch],
        ))?;
        self.run_git_command(GitCommand::new(
            path.into(),
            &["remote", "add", "origin", &self.meta.repo_url],
        ))?;

        self.run_git_command(GitCommand::new(
            path.into(),
            &["switch", "-c", &self.meta.branch],
        ))?;

        self.run_git_command(GitCommand::new(path.into(), &["add", "-A"]))?;
        self.run_git_command(
            GitCommand::new(path.into(), &["commit", "-m", "Init"]).set_failable(true),
        )?;

        // maybe can fail
        self.run_git_command(GitCommand::new(
            path.into(),
            &["push", "--set-upstream", "origin", &self.meta.branch],
        ))?;

        let meta = SyncMetaInformation {
            strategy_name: Self::get_name().to_string(),
            data: serde_json::to_value(&self.meta)
                .map_err(|e| SyncError::Sync(format!("Failed to serialize meta: {e}")))?,
        };

        Ok(meta)
    }

    fn remove_sync(
        &self,
        notebook: &crate::core::models::notebook::Notebook,
        storage: &dyn StorageStrategy,
    ) -> Result<(), crate::core::sync_strategy::SyncError> {
        let mut path = PathBuf::from(notebook.get_path());
        path.push(".git");
        debug!("Removing .git folder at {path:?}");

        std::fs::remove_dir_all(path)
            .map_err(|e| SyncError::RemoveSync(format!("Failed to delete .git folder: {e}")))?;

        Ok(())
    }

    fn sync_note(
        &self,
        note: &crate::core::models::note::Note,
        storage: &dyn StorageStrategy,
    ) -> Result<(), crate::core::sync_strategy::SyncError> {
        let notebook_path = PathBuf::from(note.get_notebook().get_path());
        let notebook_path = notebook_path.to_str().expect("to get path");
        let note_name = note.get_file_name();

        let mut files = note
            .get_metadata()
            .attachments
            .iter()
            .map(|e| e.get_path())
            .collect_vec();

        let files_string = note
            .get_metadata()
            .attachments
            .iter()
            .map(|e| e.get_name())
            .join(", ");

        let files: Vec<&str> = {
            let mut vec = Vec::new();
            vec.push("add");
            vec.push(&note_name);

            vec.append(&mut files);
            vec
        };

        self.run_git_command(GitCommand::new(notebook_path.into(), &files))?;
        self.run_git_command(
            GitCommand::new(
                notebook_path.into(),
                &[
                    "commit",
                    "-m",
                    &format!(
                        "Edit: {}{}{}",
                        note.get_title(),
                        if !files_string.is_empty() { " | " } else { "" },
                        files_string
                    ),
                ],
            )
            .set_failable(true),
        )?;
        self.run_git_command(GitCommand::new(notebook_path.into(), &["push"]))?;

        Ok(())
    }

    fn from_metadata(
        metadata: &crate::core::sync_strategy::meta::SyncMetaInformation,
        storage: &dyn StorageStrategy,
    ) -> Self {
        let meta: GitSyncMeta =
            serde_json::from_value(metadata.data.clone()).expect("to read back meta");

        Self { meta }
    }

    fn get_name() -> &'static str {
        "git"
    }

    fn sync_manual(
        &self,
        notebook: &crate::core::models::notebook::Notebook,
        storage: &dyn StorageStrategy,
    ) -> Result<(), SyncError> {
        let path = notebook.get_path();

        self.run_git_command(GitCommand::new(path.into(), &["pull"]))?;
        self.run_git_command(GitCommand::new(path.into(), &["add", "-A"]))?;
        self.run_git_command(
            GitCommand::new(path.into(), &["commit", "-m", "Manual sync"]).set_failable(true),
        )?;
        self.run_git_command(GitCommand::new(path.into(), &["push"]))?;

        Ok(())
    }

    fn sync_import(
        &self,
        notebook_path: &str,
        storage: &dyn StorageStrategy,
    ) -> Result<SyncMetaInformation, SyncError> {
        std::fs::remove_dir_all(notebook_path)
            .map_err(|e| SyncError::Import(format!("Failed to remove notebook: {e}")))?;

        self.run_git_command(GitCommand::new(
            None,
            &[
                "clone",
                &self.meta.repo_url,
                "-b",
                &self.meta.branch,
                notebook_path,
            ],
        ))?;

        let meta = SyncMetaInformation {
            strategy_name: Self::get_name().to_string(),
            data: serde_json::to_value(&self.meta)
                .map_err(|e| SyncError::Sync(format!("Failed to serialize meta: {e}")))?,
        };

        Ok(meta)
    }
}
