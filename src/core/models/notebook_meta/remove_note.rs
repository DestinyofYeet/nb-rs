use crate::core::models::notebook_meta::NotebookMetaInformation;

impl NotebookMetaInformation {
    pub fn remove_note(&mut self, note_path: &str) -> bool {
        let len = self.note_paths.len();
        self.note_paths.retain(|elem| *elem != note_path);

        len > self.note_paths.len()
    }
}
