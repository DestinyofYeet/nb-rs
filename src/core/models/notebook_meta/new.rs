use crate::core::models::notebook_meta::NotebookMetaInformation;

impl NotebookMetaInformation {
    pub fn new() -> Self {
        Self {
            note_paths: Vec::new(),
        }
    }
}
