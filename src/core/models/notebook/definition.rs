use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Notebook {
    name: String,
    path: PathBuf,
}

impl Notebook {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }
}
