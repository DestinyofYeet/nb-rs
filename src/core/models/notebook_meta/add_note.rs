use crate::core::models::notebook_meta::NotebookMetaInformation;

impl NotebookMetaInformation {
    pub(crate) fn add_note(&mut self, path: String) {
        self.note_paths.push(path);
    }
}
