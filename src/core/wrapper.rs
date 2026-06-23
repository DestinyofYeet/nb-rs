use std::path::PathBuf;

use crate::{
    core::{
        Nb, NbError,
        models::{
            note::Note, note_meta::NoteMetaInformation, notebook::Notebook,
            notebook_meta::NotebookMetaInformation,
        },
        storage_strategy::StorageStrategy,
        sync_strategy::{SyncStrategy, meta::SyncMetaInformation},
    },
    default_strategies::sync::no_op::NoopSync,
};

impl Nb {
    pub fn list_notebooks(&self) -> Result<Vec<Notebook>, NbError> {
        Ok(self.storage.list_notebooks()?)
    }

    pub fn create_notebook(&self, name: String) -> Result<(), NbError> {
        Ok(self.storage.create_notebook(name)?)
    }

    pub fn get_notebook(&self, name: String) -> Result<Option<Notebook>, NbError> {
        Ok(self.storage.get_notebook(name)?)
    }

    pub fn create_note<'a>(
        &self,
        notebook: &'a mut Notebook,
        title: String,
        path: &'a str,
    ) -> Result<(), NbError> {
        Ok(self.storage.create_note(notebook, title, path)?)
    }

    pub fn get_note<'a>(
        &self,
        notebook: &'a Notebook,
        note_path: &'a str,
    ) -> Result<Option<Note<'a>>, NbError> {
        Ok(self.storage.get_note_by_path(notebook, note_path)?)
    }

    pub fn save_note(&self, notebook: &Notebook, note: &Note) -> Result<(), NbError> {
        self.storage.save_note(notebook, note)?;

        let meta = notebook.get_meta().get_sync_information();

        let sync = meta.get_strategy(&*self.storage)?;

        sync.sync_note(note, &*self.storage)?;

        Ok(())
    }

    pub fn list_notes<'a>(&self, notebook: &'a Notebook) -> Result<Vec<Note<'a>>, NbError> {
        Ok(self.storage.list_notes(notebook)?)
    }

    pub fn delete_note<'a>(
        &self,
        notebook: &'a mut Notebook,
        note_path: &'a str,
    ) -> Result<(), NbError> {
        Ok(self.storage.delete_note(notebook, note_path)?)
    }

    pub fn delete_notebook(&self, notebook: &Notebook) -> Result<(), NbError> {
        Ok(self.storage.delete_notebook(notebook)?)
    }

    pub fn get_path_on_fs<'a>(&self, notebook: &Notebook, path: &str) -> Result<PathBuf, NbError> {
        Ok(self.storage.get_path_on_fs(notebook, path)?)
    }

    pub fn save_sync_setup(
        &self,
        notebook: &mut Notebook,
        strategy: SyncMetaInformation,
    ) -> Result<(), NbError> {
        {
            notebook.get_meta_mut().set_sync_meta(strategy);
        }

        self.storage
            .save_notebook_meta(&notebook.get_path(), notebook.get_meta())?;

        let meta = notebook
            .get_meta()
            .get_sync_information()
            .get_strategy(&*self.storage)?;

        meta.sync_manual(notebook, &*self.storage)?;

        Ok(())
    }

    pub fn remove_sync(&self, notebook: &mut Notebook) -> Result<(), NbError> {
        let old_strat = notebook
            .get_meta()
            .get_sync_information()
            .get_strategy(&*self.storage)?;
        old_strat.remove_sync(notebook, &*self.storage)?;

        let strat = NoopSync {}.setup_sync(notebook, &*self.storage)?;
        notebook.get_meta_mut().set_sync_meta(strat);

        self.storage
            .save_notebook_meta(&notebook.get_path(), notebook.get_meta())?;
        Ok(())
    }

    pub fn sync_note(&self, note: &Note) -> Result<(), NbError> {
        let meta = note.get_notebook().get_meta();
        let sync = meta.get_sync_information().get_strategy(&*self.storage)?;
        sync.sync_note(note, &*self.storage)?;
        Ok(())
    }

    pub fn sync_manual(&self, notebook: &Notebook) -> Result<(), NbError> {
        let sync = notebook
            .get_meta()
            .get_sync_information()
            .get_strategy(&*self.storage)?;
        sync.sync_manual(notebook, &*self.storage)?;

        Ok(())
    }

    pub fn sync_import(
        &self,
        sync: Box<dyn SyncStrategy>,
        notebook: &Notebook,
    ) -> Result<(), NbError> {
        let path = notebook.get_path();

        let meta = sync.sync_import(path, &*self.storage)?;

        let mut book_meta = NotebookMetaInformation::new();
        book_meta.set_sync_meta(meta);

        for file in self.storage.list_files(notebook)? {
            book_meta.add_note(file.clone());

            let mut note_path = PathBuf::from(notebook.get_path());
            note_path.push(file.clone());

            self.storage.save_note_meta(
                note_path.to_str().expect("to have a valid path"),
                &NoteMetaInformation::new(file),
            )?;
        }

        self.storage
            .save_notebook_meta(notebook.get_path(), &book_meta)?;

        Ok(())
    }

    pub fn get_storage(&self) -> &dyn StorageStrategy {
        &*self.storage
    }
}
