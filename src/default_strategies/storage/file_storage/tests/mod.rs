use std::path::PathBuf;

use tempfile::tempdir;

mod note;
mod notebook;

pub fn get_temp() -> PathBuf {
    tempdir().unwrap().keep()
}
