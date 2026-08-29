use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use itertools::Itertools;
use tracing::debug;

use crate::{
    core::{
        models::notebook::Notebook,
        storage_strategy::StorageStrategy,
        sync_strategy::{SyncError, SyncStrategy, meta::SyncMetaInformation, sync_kind::SyncKind},
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
    pub fn new(cwd: impl Into<Option<&'a str>>, args: &'a [&'a str]) -> Self {
        Self {
            can_fail: false,
            cwd: cwd.into(),
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

    fn collect_all_files_in_notebook(
        &self,
        notebook: &Notebook,
        storage: &dyn StorageStrategy,
    ) -> Result<Vec<String>, SyncError> {
        let files: Vec<String> = {
            let mut vec = Vec::new();

            for note in storage
                .list_notes(notebook)
                .map_err(|e| SyncError::Sync(format!("Failed to list files: {e}")))?
            {
                vec.push(note.get_file_name().to_string());

                vec.push(
                    storage
                        .get_note_metadata_file(&note)
                        .map_err(|e| SyncError::Sync(format!("Failed to get note metadata: {e}")))?
                        .file_name()
                        .expect("to have filename")
                        .to_string_lossy()
                        .to_string(),
                );

                for attachment in &note.get_metadata().attachments {
                    vec.push(attachment.get_path().to_string());
                }
            }

            let notebook_meta = storage
                .get_notebook_metadata_file(notebook)
                .map_err(|e| SyncError::Sync(format!("Failed to get notebook meta: {e}")))?;

            vec.push(
                notebook_meta
                    .file_name()
                    .expect("to get filename")
                    .to_string_lossy()
                    .to_string(),
            );

            vec
        };

        Ok(files)
    }
}

impl SyncStrategy for GitSync {
    fn setup_sync(
        &self,
        notebook: &crate::core::models::notebook::Notebook,
        storage: &dyn StorageStrategy,
    ) -> Result<SyncMetaInformation, crate::core::sync_strategy::SyncError> {
        let path = notebook.get_path();
        self.run_git_command(GitCommand::new(path, &["init", "-b", &self.meta.branch]))?;
        self.run_git_command(GitCommand::new(
            path,
            &["remote", "add", "origin", &self.meta.repo_url],
        ))?;

        self.run_git_command(GitCommand::new(path, &["switch", "-c", &self.meta.branch]))?;

        let args = {
            let mut vec: Vec<String> = Vec::new();
            let mut files = self.collect_all_files_in_notebook(notebook, storage)?;
            vec.push("add".to_string());
            vec.append(&mut files);

            vec
        };

        self.run_git_command(GitCommand::new(
            path,
            &args.iter().map(|e| e.as_str()).collect_vec(),
        ))?;

        self.run_git_command(
            GitCommand::new(path, &["commit", "-m", "[nb-rs] Init"]).set_failable(true),
        )?;

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
        _storage: &dyn StorageStrategy,
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
        kind: SyncKind,
    ) -> Result<(), crate::core::sync_strategy::SyncError> {
        let notebook_path = PathBuf::from(note.get_notebook().get_path());
        let notebook_path = notebook_path.to_str().expect("to get path");
        let note_name = note.get_file_name();

        let note_meta = storage
            .get_note_metadata_file(note)
            .map_err(|e| SyncError::Sync(format!("Failed to find metadata: {e}")))?;

        let note_meta_file = note_meta.file_name().expect("to have a base name");
        let note_meta_file_path = note_meta_file.to_string_lossy();

        let notebook_meta = storage
            .get_notebook_metadata_file(note.get_notebook())
            .map_err(|e| SyncError::Sync(format!("Failed to get notebook meta: {e}")))?;

        let notebook_meta_path = notebook_meta
            .file_name()
            .expect("to have a filename")
            .to_string_lossy();

        let mut files = note
            .get_metadata()
            .attachments
            .iter()
            .map(|e| e.get_path())
            .collect_vec();

        let commit_msg = files.iter().join(", ");

        files.push(note_name);
        files.push(&note_meta_file_path);

        if matches!(kind, SyncKind::Create) {
            files.push(&notebook_meta_path);
        }

        let commit_string = match kind {
            SyncKind::Create => "Create",
            SyncKind::Edit => "Edit",
            SyncKind::Delete => "Delete",
        };

        match kind {
            SyncKind::Create | SyncKind::Edit => {
                let args: Vec<&str> = {
                    let mut vec = Vec::new();
                    vec.push("add");

                    vec.append(&mut files);
                    vec
                };

                self.run_git_command(GitCommand::new(notebook_path, &args))?;
            }

            SyncKind::Delete => {
                let args: Vec<&str> = {
                    let mut vec = Vec::new();
                    vec.push("rm");
                    vec.push("--cached");

                    vec.append(&mut files);
                    vec
                };

                self.run_git_command(GitCommand::new(notebook_path, &args))?;
            }
        }

        self.run_git_command(
            GitCommand::new(
                notebook_path,
                &[
                    "commit",
                    "-m",
                    &format!(
                        "[nb-rs] {commit_string}: {}{}{}",
                        note.get_title(),
                        if !commit_msg.is_empty() { " | " } else { "" },
                        commit_msg
                    ),
                ],
            )
            .set_failable(true),
        )?;
        self.run_git_command(GitCommand::new(notebook_path, &["push"]))?;

        Ok(())
    }

    fn from_metadata(
        metadata: &crate::core::sync_strategy::meta::SyncMetaInformation,
        _storage: &dyn StorageStrategy,
    ) -> Self {
        let meta: GitSyncMeta =
            serde_json::from_value(metadata.data.clone()).expect("to read back meta");

        Self { meta }
    }

    fn get_name() -> &'static str {
        "git"
    }

    fn sync_full(
        &self,
        notebook: &crate::core::models::notebook::Notebook,
        storage: &dyn StorageStrategy,
        hint: Option<String>,
    ) -> Result<(), SyncError> {
        let path = notebook.get_path();

        let files = self.collect_all_files_in_notebook(notebook, storage)?;

        debug!("files: {files:?}");

        let args = {
            let mut vec = Vec::new();
            vec.push("add");

            for file in files.iter() {
                vec.push(file);
            }

            vec
        };

        let msg = format!("[nb-rs] {}", hint.unwrap_or("Full sync".to_string()));

        self.run_git_command(GitCommand::new(path, &["pull"]))?;
        self.run_git_command(GitCommand::new(path, &args))?;
        self.run_git_command(GitCommand::new(path, &["commit", "-m", &msg]).set_failable(true))?;
        self.run_git_command(GitCommand::new(path, &["push"]))?;

        Ok(())
    }

    fn sync_import(
        &self,
        notebook_path: &str,
        _storage: &dyn StorageStrategy,
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
