use serde::{Deserialize, Serialize};

use crate::core::sync_strategy::meta::SyncMetaInformation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookMetaInformation {
    pub(super) note_paths: Vec<String>,
    pub(super) sync_meta: SyncMetaInformation,
}
