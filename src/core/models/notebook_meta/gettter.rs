use crate::core::{
    models::notebook_meta::NotebookMetaInformation, sync_strategy::meta::SyncMetaInformation,
};

impl NotebookMetaInformation {
    pub fn get_notes(&self) -> &Vec<String> {
        &self.note_paths
    }

    pub fn get_sync_information(&self) -> &SyncMetaInformation {
        &self.sync_meta
    }
}
