use crate::core::models::{note_path::NoteFilename, notebook_meta::NotebookMetaInformation};

impl NotebookMetaInformation {
    pub(crate) fn add_note(&mut self, path: &NoteFilename) {
        self.note_paths.push(path.get_filename().to_string());
    }
}
