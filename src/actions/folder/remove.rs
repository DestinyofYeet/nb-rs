use std::{fs::remove_dir, path::PathBuf};

use colored::Colorize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RemoveFolderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn folder_remove(root: &PathBuf, name: &String) -> Result<(), RemoveFolderError> {
    let mut path = PathBuf::new();
    path.push(root);
    path.push(name);

    remove_dir(path)?;

    println!("Successfully removed folder '{}'", name.blue());

    Ok(())
}
