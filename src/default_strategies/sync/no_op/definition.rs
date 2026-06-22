use crate::core::sync_strategy::{SyncStrategy, meta::SyncMetaInformation};

pub struct NoopSync {}

impl SyncStrategy for NoopSync {
    fn setup_sync(
        &self,
        _notebook: &crate::core::models::notebook::Notebook,
    ) -> Result<SyncMetaInformation, crate::core::sync_strategy::SyncError> {
        Ok(SyncMetaInformation::new())
    }

    fn remove_sync(
        &self,
        _notebook: &crate::core::models::notebook::Notebook,
    ) -> Result<(), crate::core::sync_strategy::SyncError> {
        Ok(())
    }

    fn sync_note(
        &self,
        _note: &crate::core::models::note::Note,
    ) -> Result<(), crate::core::sync_strategy::SyncError> {
        Ok(())
    }

    fn from_metadata(_metadata: &crate::core::sync_strategy::meta::SyncMetaInformation) -> Self
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

    fn sync_manual(
        &self,
        _notebook: &crate::core::models::notebook::Notebook,
    ) -> Result<(), crate::core::sync_strategy::SyncError> {
        Ok(())
    }

    fn sync_import(
        &self,
        _notebook_path: &str,
    ) -> Result<(), crate::core::sync_strategy::SyncError> {
        Ok(())
    }
}
