use crate::core::{
    storage_strategy::StorageStrategy,
    sync_strategy::{SyncStrategy, meta::SyncMetaInformation, sync_kind::SyncKind},
};

pub struct NoopSync {}

impl SyncStrategy for NoopSync {
    fn setup_sync(
        &self,
        _notebook: &crate::core::models::notebook::Notebook,
        _storage: &dyn StorageStrategy,
    ) -> Result<SyncMetaInformation, crate::core::sync_strategy::SyncError> {
        Ok(SyncMetaInformation::new())
    }

    fn remove_sync(
        &self,
        _notebook: &crate::core::models::notebook::Notebook,
        _storage: &dyn StorageStrategy,
    ) -> Result<(), crate::core::sync_strategy::SyncError> {
        Ok(())
    }

    fn sync_note(
        &self,
        _note: &crate::core::models::note::Note,
        _storage: &dyn StorageStrategy,
        _kind: SyncKind,
    ) -> Result<(), crate::core::sync_strategy::SyncError> {
        Ok(())
    }

    fn from_metadata(
        _metadata: &crate::core::sync_strategy::meta::SyncMetaInformation,
        _storage: &dyn StorageStrategy,
    ) -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn get_name() -> &'static str
    where
        Self: Sized,
    {
        "no_op"
    }

    fn sync_full(
        &self,
        _notebook: &crate::core::models::notebook::Notebook,
        _storage: &dyn StorageStrategy,
        _hint: Option<String>,
    ) -> Result<(), crate::core::sync_strategy::SyncError> {
        Ok(())
    }

    fn sync_import(
        &self,
        _notebook_path: &str,
        _storage: &dyn StorageStrategy,
    ) -> Result<SyncMetaInformation, crate::core::sync_strategy::SyncError> {
        Ok(SyncMetaInformation::new())
    }
}
