use std::path::{Path, PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FolderError {
    #[error("failed to convert PathBuf while parsing Folder")]
    PathBufConversionError,
}

pub struct Folder {
    pub path: String,
    pub name: String,
}

impl Folder {
    pub fn new(path: impl ToString, name: impl ToString) -> Self {
        Self {
            path: path.to_string(),
            name: name.to_string(),
        }
    }

    pub fn from_pathbuf(path: &Path, name: impl ToString) -> Result<Self, FolderError> {
        let path = match path.to_str() {
            None => return Err(FolderError::PathBufConversionError),
            Some(value) => value,
        };

        Ok(Self::new(path, name))
    }

    pub fn get_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(&self.path);
        path.push(&self.name);

        path
    }
}
