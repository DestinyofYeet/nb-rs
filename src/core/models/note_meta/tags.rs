use crate::core::models::note_meta::NoteMetaInformation;

impl NoteMetaInformation {
    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    pub fn remove_tag(&mut self, tag: String) {
        if let Some(tag_index) = self.tags.iter().position(|e| e == &tag) {
            self.tags.remove(tag_index);
        }
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
    }
}
