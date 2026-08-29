use crate::core::{
    models::{note::Note, notebook::Notebook},
    storage_strategy::StorageStrategy,
    sync_strategy::{SyncError, meta::SyncMetaInformation, sync_kind::SyncKind},
};

pub trait SyncStrategy {
    /// Get the name of the sync strategy
    fn get_name() -> &'static str
    where
        Self: Sized;

    /// Set up the sync strategy
    fn setup_sync(
        &self,
        notebook: &Notebook,
        storage: &dyn StorageStrategy,
    ) -> Result<SyncMetaInformation, SyncError>;

    /// Remove sync strategy
    fn remove_sync(
        &self,
        notebook: &Notebook,
        storage: &dyn StorageStrategy,
    ) -> Result<(), SyncError>;

    /// Sync a single note
    fn sync_note(
        &self,
        note: &Note,
        storage: &dyn StorageStrategy,
        kind: SyncKind,
    ) -> Result<(), SyncError>;

    /// Sync a full notebook
    fn sync_full(
        &self,
        notebook: &Notebook,
        storage: &dyn StorageStrategy,
        hint: Option<String>,
    ) -> Result<(), SyncError>;

    /// Instanciate a Strategy from a metadata
    fn from_metadata(metadata: &SyncMetaInformation, storage: &dyn StorageStrategy) -> Self
    where
        Self: Sized;

    /// Import the specified remote (in self) into the specified `notebook_path`
    fn sync_import(
        &self,
        notebook_path: &str,
        storage: &dyn StorageStrategy,
    ) -> Result<SyncMetaInformation, SyncError>;
}
