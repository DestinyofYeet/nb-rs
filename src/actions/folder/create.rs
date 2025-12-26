use colored::Colorize;
use std::fs;

use thiserror::Error;
use tracing::debug;

use crate::actions::folder::model::Folder;

#[derive(Error, Debug)]
pub enum FolderCreationError {
    #[error("the folder {0} already exists!")]
    FolderExists(String),

    #[error("failed to create folder: {0}")]
    Create(String),
}

impl Folder {
    pub fn create(&self) -> Result<(), FolderCreationError> {
        let path = self.get_path();

        if path.exists() {
            return Err(FolderCreationError::FolderExists(self.name.clone()));
        }

        fs::create_dir(path.clone()).map_err(|e| FolderCreationError::Create(e.to_string()))?;
        debug!("Created directory '{}'", path.to_str().unwrap());

        println!("Created folder '{}'", self.name.blue());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{actions::folder::model::Folder, tests::test::Test};

    #[test]
    fn folder_create() {
        let test = Test::setup("folder_create");

        let folder = Folder::from_pathbuf(&test.dir, "test_folder").unwrap();

        folder.create().unwrap();

        assert!(folder.get_path().exists())
    }
}
