use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use itertools::Itertools;
use tracing::debug;

use crate::{
    core::sync_strategy::{SyncError, SyncStrategy, meta::SyncMetaInformation},
    default_strategies::sync::git::meta::GitSyncMeta,
};

pub struct GitSync {
    pub(super) meta: GitSyncMeta,
}

struct GitCommand<'a> {
    can_fail: bool,
    cwd: &'a str,
    args: &'a [&'a str],
}

impl<'a> GitCommand<'a> {
    pub fn new(cwd: &'a str, args: &'a [&'a str]) -> Self {
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
            .current_dir(git_command.cwd)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

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
    ) -> Result<SyncMetaInformation, crate::core::sync_strategy::SyncError> {
        let path = notebook.get_path();
        self.run_git_command(GitCommand::new(path, &["init", "-b", &self.meta.branch]))?;
        self.run_git_command(GitCommand::new(
            path,
            &["remote", "add", "origin", &self.meta.repo_url],
        ))?;

        self.run_git_command(GitCommand::new(path, &["switch", "-c", &self.meta.branch]))?;
        self.run_git_command(GitCommand::new(path, &["add", "-A"]))?;
        self.run_git_command(GitCommand::new(path, &["commit", "-m", "Init"]).set_failable(true))?;

        // maybe can fail
        self.run_git_command(GitCommand::new(
            path,
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
    ) -> Result<(), crate::core::sync_strategy::SyncError> {
        let path = PathBuf::from(notebook.get_path());
        std::fs::remove_dir_all(path.join(".git"))
            .map_err(|e| SyncError::RemoveSync(format!("Failed to delete .git folder: {e}")))?;

        Ok(())
    }

    fn sync_note(
        &self,
        note: &crate::core::models::note::Note,
    ) -> Result<(), crate::core::sync_strategy::SyncError> {
        let notebook_path = PathBuf::from(note.get_notebook().get_path());
        let notebook_path = notebook_path.to_str().expect("to get path");

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

            vec.append(&mut files);
            vec
        };

        self.run_git_command(GitCommand::new(notebook_path, &files))?;
        self.run_git_command(GitCommand::new(
            notebook_path,
            &[
                "commit",
                "-m",
                &format!("Edit: {} | {}", note.get_title(), files_string),
            ],
        ))?;
        self.run_git_command(GitCommand::new(notebook_path, &["push"]))?;

        Ok(())
    }

    fn from_metadata(metadata: &crate::core::sync_strategy::meta::SyncMetaInformation) -> Self {
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
    ) -> Result<(), SyncError> {
        let path = notebook.get_path();

        self.run_git_command(GitCommand::new(path, &["pull"]))?;
        self.run_git_command(GitCommand::new(path, &["add", "-A"]))?;
        self.run_git_command(
            GitCommand::new(path, &["commit", "-m", "Manual sync"]).set_failable(true),
        )?;
        self.run_git_command(GitCommand::new(path, &["push"]))?;

        Ok(())
    }

    fn sync_import(&self, notebook_path: &str) -> Result<(), SyncError> {
        todo!()
    }
}
