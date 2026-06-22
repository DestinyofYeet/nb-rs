use std::path::PathBuf;

use crate::core::models::notebook_meta::NotebookMetaInformation;

#[derive(Debug, Clone)]
pub struct Notebook {
    pub(super) name: String,
    pub(super) path: String,

    pub(super) meta: NotebookMetaInformation,
}

impl Notebook {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub(crate) fn get_meta_mut(&mut self) -> &mut NotebookMetaInformation {
        &mut self.meta
    }

    pub fn get_meta(&self) -> &NotebookMetaInformation {
        &self.meta
    }

    pub(crate) fn new(name: String, path: String, meta: NotebookMetaInformation) -> Self {
        Self { name, path, meta }
    }
}
