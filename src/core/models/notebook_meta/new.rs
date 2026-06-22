use crate::core::{
    models::notebook_meta::NotebookMetaInformation, sync_strategy::meta::SyncMetaInformation,
};

impl NotebookMetaInformation {
    pub fn new() -> Self {
        Self {
            note_paths: Vec::new(),
            sync_meta: SyncMetaInformation::new(),
        }
    }
}

impl Default for NotebookMetaInformation {
    fn default() -> Self {
        Self::new()
    }
}
