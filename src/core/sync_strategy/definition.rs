use crate::core::{
    models::{note::Note, notebook::Notebook},
    sync_strategy::{SyncError, meta::SyncMetaInformation},
};

pub trait SyncStrategy {
    fn get_name() -> &'static str
    where
        Self: Sized;

    fn setup_sync(&self, notebook: &Notebook) -> Result<SyncMetaInformation, SyncError>;

    fn remove_sync(&self, notebook: &Notebook) -> Result<(), SyncError>;

    fn sync_note(&self, note: &Note) -> Result<(), SyncError>;

    fn from_metadata(metadata: &SyncMetaInformation) -> Self
    where
        Self: Sized;
}
