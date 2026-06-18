use crate::core::models::notebook_meta::NotebookMetaInformation;

impl NotebookMetaInformation {
    pub fn get_notes(&self) -> &Vec<String> {
        &self.note_paths
    }
}
