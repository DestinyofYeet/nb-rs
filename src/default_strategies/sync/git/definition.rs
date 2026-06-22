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

impl GitSync {
    pub fn new(meta: GitSyncMeta) -> Self {
        Self { meta }
    }

    pub fn run_git_command(&self, cwd: &str, commands: &[&str]) -> Result<(), SyncError> {
        let mut command = Command::new("git");
        command
            .args(commands)
            .current_dir(cwd)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        debug!("Running git command: {:?}", command.get_args());

        let output = command
            .output()
            .map_err(|e| SyncError::Sync(format!("Failed to get process output: {e}")))?;

        if !output.status.success() {
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

        self.run_git_command(path, &["init", "-b", &self.meta.branch])?;
        self.run_git_command(path, &["remote", "add", "origin", &self.meta.repo_url])?;
        self.run_git_command(path, &["switch", "-c", &self.meta.branch])?;
        self.run_git_command(path, &["add", "-A"])?;
        self.run_git_command(
            path,
            &["commit", "-m", &format!("[{}] Init", notebook.get_name())],
        )?;
        self.run_git_command(
            path,
            &["push", "--set-upstream", "origin", &self.meta.branch],
        )?;

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

        let files: Vec<&str> = {
            let mut vec = Vec::new();
            vec.push("add");

            vec.append(&mut files);
            vec
        };

        self.run_git_command(notebook_path, &files)?;
        self.run_git_command(notebook_path, &["push"])?;

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
}
