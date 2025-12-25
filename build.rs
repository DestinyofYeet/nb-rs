use std::{
    io::Read,
    process::{Command, Stdio, exit},
};

fn get_git_rev() -> String {
    let mut command = Command::new("git");
    command.args(["rev-parse", "--short", "HEAD"]);
    command.stdin(Stdio::piped());
    command.stdout(Stdio::piped());
    let result = command.spawn();

    let mut child = match result {
        Ok(child) => child,
        Err(e) => {
            eprintln!("cargo::error=Failed to spawn git command: {}", e);
            exit(1);
        }
    };

    let result = match child.wait() {
        Ok(value) => value,
        Err(e) => {
            eprintln!("cargo::error=Failed to wait for process: {}", e);
            exit(1)
        }
    };

    let mut stdout = String::new();
    let mut stderr = String::new();

    if let Some(mut child_stdout) = child.stdout {
        _ = child_stdout.read_to_string(&mut stdout);
    }

    if let Some(mut child_stderr) = child.stderr {
        _ = child_stderr.read_to_string(&mut stderr);
    }

    if !result.success() {
        eprintln!(
            "cargo::error=Getting git rev failed!\nStdout:\n{}\nStderr:\n{}",
            stdout, stderr
        );
        exit(1);
    }

    stdout.trim().to_string()
}

static GIT_REV: Option<&str> = option_env!("GIT_REV");

fn main() {
    let git_rev = GIT_REV.map(|e| e.to_string()).unwrap_or_else(get_git_rev);

    println!("cargo::rustc-env=GIT_REV={}", git_rev)
}
