use crate::core::models::note::Note;

impl<'a> Note<'a> {
    pub fn get_title(&self) -> &str {
        &self.meta.title
    }
}
