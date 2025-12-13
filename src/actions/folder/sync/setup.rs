use std::{
    io::Read,
    process::{Command, ExitStatus, Stdio},
};

use itertools::Itertools;
use thiserror::Error;
use tracing::debug;

use crate::{actions::folder::model::Folder, config::model::Config};

#[derive(Error, Debug)]
pub enum SetupSyncError {
    #[error("sync is already configured!")]
    SyncExists,

    #[error("failed to run git command '{}': {}", .command, .err)]
    Git { command: String, err: String },

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

type Error = SetupSyncError;

impl Folder {
    pub fn sync_exists(&self, config: &Config) -> bool {
        let mut path = self.get_path();
        while path != config.data_dir {
            let mut tmp = path.clone();
            tmp.push(".git");
            if tmp.exists() {
                return true;
            }

            path.pop();
        }

        false
    }
    pub fn sync_run_git_command(&self, args: &[&str]) -> Result<ExitStatus, Error> {
        self.sync_run_git_command_conf(args, false)
    }

    pub fn sync_run_git_command_conf(
        &self,
        args: &[&str],
        ignore_errors: bool,
    ) -> Result<ExitStatus, Error> {
        let mut command = Command::new("git");
        command
            .args(args)
            .current_dir(self.get_path())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = command.spawn()?;
        let status = child.wait()?;

        let mut stderr = String::new();
        child.stderr.unwrap().read_to_string(&mut stderr)?;

        let mut stdout = String::new();
        child.stdout.unwrap().read_to_string(&mut stdout)?;

        if !status.success() && !ignore_errors {
            return Err(Error::Git {
                command: args.join(" "),
                err: format!("Stdout: {}\n\nStderr: {}", stdout, stderr),
            });
        }

        debug!(
            "Ran 'git {}'\nStdout:{}\nStderr:{}\n",
            args.iter()
                .map(|arg| {
                    if arg.contains(" ") {
                        format!("\"{}\"", arg)
                    } else {
                        arg.to_string()
                    }
                })
                .join(" "),
            stdout,
            stderr
        );

        Ok(status)
    }

    pub fn sync_setup(&self, config: &Config, repo: &str, branch: &str) -> Result<(), Error> {
        if self.sync_exists(config) {
            return Err(Error::SyncExists);
        }

        self.sync_run_git_command(&["init", "-b", branch])?;
        self.sync_run_git_command(&["remote", "add", "origin", repo])?;
        self.sync_run_git_command(&["fetch", "origin"])?;
        self.sync_run_git_command(&["checkout", "-b", branch, &format!("origin/{}", branch)])?;

        println!("Set up git tracking!");

        Ok(())
    }
}
