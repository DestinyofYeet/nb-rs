use crate::core::sync_strategy::SyncStrategy;

pub struct GitSync {}

impl SyncStrategy for GitSync {
    fn setup_sync(&self, notebook: &crate::core::models::notebook::Notebook) {
        todo!()
    }

    fn remove_sync(&self, notebook: &crate::core::models::notebook::Notebook) {
        todo!()
    }

    fn sync_note(&self, note: &crate::core::models::note::Note) {
        todo!()
    }
}
