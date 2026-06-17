use crate::core::models::note_meta::NoteMetaInformation;

impl NoteMetaInformation {
    pub fn new(title: String) -> Self {
        Self {
            title,
            attachments: Vec::new(),
            tags: Vec::new(),
        }
    }
}
