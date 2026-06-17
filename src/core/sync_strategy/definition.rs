use crate::core::models::{note::Note, notebook::Notebook};

pub trait SyncStragegy {
    fn setup_sync(&self, notebook: &Notebook);
    fn remove_sync(&self, notebook: &Notebook);

    fn sync_note(&self, note: &Note);
}
