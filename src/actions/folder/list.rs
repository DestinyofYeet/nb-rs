use std::path::PathBuf;

use colored::Colorize;
use itertools::Itertools;
use thiserror::Error;
use tracing::debug;

#[derive(Error, Debug)]
pub enum ListFolderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn folder_list(root: &PathBuf, folder: &String) -> Result<(), ListFolderError> {
    let mut path = PathBuf::new();
    path.push(root);
    path.push(folder);

    debug!("Searching for folders in {}", path.to_str().unwrap());

    let mut found_folders = Vec::new();
    let mut found_notes = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            found_folders.push(entry)
        } else if file_type.is_file() {
            found_notes.push(entry)
        }
    }

    if found_folders.is_empty() && found_notes.is_empty() {
        println!("{}", "The folder is empty!".red());
        return Ok(());
    }

    let mut entries = found_folders;

    entries.append(&mut found_notes);

    println!(
        "Found {} entries!\n{}",
        entries.len(),
        entries
            .into_iter()
            .map(|elem| {
                let file_type = elem.file_type().unwrap();

                if file_type.is_dir() {
                    format!("- D {}/", elem.file_name().to_str().unwrap().blue())
                } else {
                    format!("- F {}", elem.file_name().to_str().unwrap().blue())
                }
            })
            .join("\n")
    );

    Ok(())
}
