use crate::core::{
    models::{note::Note, notebook::Notebook},
    storage_strategy::StorageStrategy,
    sync_strategy::{SyncError, meta::SyncMetaInformation},
};

pub trait SyncStrategy {
    fn get_name() -> &'static str
    where
        Self: Sized;

    fn setup_sync(
        &self,
        notebook: &Notebook,
        storage: &dyn StorageStrategy,
    ) -> Result<SyncMetaInformation, SyncError>;

    fn remove_sync(
        &self,
        notebook: &Notebook,
        storage: &dyn StorageStrategy,
    ) -> Result<(), SyncError>;

    fn sync_note(&self, note: &Note, storage: &dyn StorageStrategy) -> Result<(), SyncError>;

    fn sync_manual(
        &self,
        notebook: &Notebook,
        storage: &dyn StorageStrategy,
    ) -> Result<(), SyncError>;

    fn from_metadata(metadata: &SyncMetaInformation, storage: &dyn StorageStrategy) -> Self
    where
        Self: Sized;

    fn sync_import(
        &self,
        notebook_path: &str,
        storage: &dyn StorageStrategy,
    ) -> Result<SyncMetaInformation, SyncError>;
}
