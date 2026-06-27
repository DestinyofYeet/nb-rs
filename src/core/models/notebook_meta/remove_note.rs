use crate::core::models::{note_path::NoteFilename, notebook_meta::NotebookMetaInformation};

impl NotebookMetaInformation {
    pub fn remove_note(&mut self, note_path: &NoteFilename) -> bool {
        let len = self.note_paths.len();
        self.note_paths
            .retain(|elem| *elem != note_path.get_filename());

        len > self.note_paths.len()
    }
}
