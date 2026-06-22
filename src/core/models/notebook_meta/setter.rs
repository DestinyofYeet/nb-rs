use crate::core::{
    models::notebook_meta::NotebookMetaInformation, sync_strategy::meta::SyncMetaInformation,
};

impl NotebookMetaInformation {
    pub(crate) fn set_sync_meta(&mut self, meta: SyncMetaInformation) {
        self.sync_meta = meta;
    }
}
