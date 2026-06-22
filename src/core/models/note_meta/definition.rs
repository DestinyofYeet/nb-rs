use serde::{Deserialize, Serialize};

use crate::core::models::attachment::Attachment;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteMetaInformation {
    pub(crate) attachments: Vec<Attachment>,
    pub(crate) tags: Vec<String>,
    pub(crate) title: String,
}
