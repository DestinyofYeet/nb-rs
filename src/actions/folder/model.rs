use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::actions::note::model::Note;

#[derive(Error, Debug)]
pub enum FolderError {
    #[error("failed to convert PathBuf while parsing Folder")]
    PathBufConversionError,
}

#[derive(Debug)]
pub struct Folder {
    pub path: String,
    pub name: String,
}

impl Folder {
    pub fn new(path: impl ToString, name: impl ToString) -> Self {
        let mut name = name.to_string();

        if name == "." {
            name = String::new();
        }
        Self {
            path: path.to_string(),
            name,
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

    pub fn from_note(note: &Note) -> Self {
        let path = note.get_path();
        let dir = path
            .parent()
            .map(|path| path.file_name().unwrap().to_str().unwrap())
            .unwrap_or(".");
        let parent_dir = path
            .parent()
            .map(|path| {
                path.parent()
                    .map(|path| path.to_str().unwrap())
                    .unwrap_or(".")
            })
            .unwrap_or(".");

        Self::new(parent_dir, dir)
    }
}
