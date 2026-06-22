use crate::core::models::attachment::Attachment;

impl Attachment {
    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
