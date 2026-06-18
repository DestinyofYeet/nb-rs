use crate::core::models::notebook_meta::NotebookMetaInformation;

impl NotebookMetaInformation {
    pub fn remove_note(&mut self, note_path: &str) {
        self.note_paths.retain(|elem| *elem == note_path);
    }
}
