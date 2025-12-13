use std::fs::remove_dir;

use colored::Colorize;
use thiserror::Error;

use crate::actions::folder::model::Folder;

#[derive(Error, Debug)]
pub enum RemoveFolderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Folder {
    pub fn remove(&self) -> Result<(), RemoveFolderError> {
        let path = self.get_path();

        remove_dir(path)?;

        println!("Successfully removed folder '{}'", self.name.blue());

        Ok(())
    }
}
