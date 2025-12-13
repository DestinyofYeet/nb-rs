use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Lines},
    path::{Path, PathBuf},
};

use thiserror::Error;
use tracing::debug;

use crate::actions::note::create::NoteCreationError;

#[derive(Error, Debug)]
pub enum NoteError {
    #[error("failed to convert note path")]
    PathBufConversionError,

    #[error("note does not exist: {0}")]
    NoteDoesNotExist(String),

    #[error(transparent)]
    ReadError(#[from] std::io::Error),
}

#[derive(Clone, Debug)]
pub struct Note {
    pub path: String,
    pub name: String,
}

impl Note {
    pub fn new(path: impl ToString, name: impl ToString) -> Result<Self, NoteError> {
        let note = Note {
            path: path.to_string(),
            name: name.to_string(),
        };

        if !note.exists() || !note.get_path().is_file() {
            debug!("{:#?}", note);
            return Err(NoteError::NoteDoesNotExist(name.to_string()));
        }

        Ok(note)
    }

    pub fn new_create(path: impl ToString, name: impl ToString) -> Result<Self, NoteCreationError> {
        let note = Note {
            path: path.to_string(),
            name: name.to_string(),
        };

        note.create()?;

        Ok(note)
    }

    pub fn from_pathbuf(path: &Path, name: String) -> Result<Self, NoteError> {
        // let path = match path.to_str() {
        //     None => return Err(NoteError::PathBufConversionError),
        //     Some(value) => value.to_string(),
        // };

        let mut big_path = PathBuf::from(path);
        big_path.push(name);

        let file_name = match big_path
            .file_name()
            .map(|elem| elem.to_str().map(|elem| elem.to_string()))
        {
            Some(Some(some)) => some,
            _ => return Err(NoteError::PathBufConversionError),
        };

        big_path.pop();

        let directory_path = match big_path.to_str().map(|elem| elem.to_string()) {
            None => return Err(NoteError::PathBufConversionError),
            Some(value) => value,
        };

        Note::new(directory_path, file_name)
    }

    #[inline(always)]
    pub fn get_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(&self.path);
        path.push(&self.name);

        path
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn exists(&self) -> bool {
        self.get_path().exists()
    }

    pub fn get_content_by_lines(&self) -> Result<Lines<BufReader<File>>, NoteError> {
        let file = fs::File::open(self.get_path())?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn get_pretty_name(&self) -> Result<Option<String>, NoteError> {
        let lines = self.get_content_by_lines()?;

        for line in lines.map_while(Result::ok) {
            if line.starts_with("# ") {
                let line = line.strip_prefix("# ").unwrap();
                let line = line.trim();
                return Ok(Some(line.to_string()));
            }
        }

        Ok(None)
    }

    /// Returns the pretty name if available, otherwise just the file name
    pub fn get_name(&self) -> Result<String, NoteError> {
        Ok(self.get_pretty_name()?.unwrap_or(self.name.clone()))
    }
}
