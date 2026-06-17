use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookMetaInformation {
    pub(super) note_paths: Vec<String>,
}
