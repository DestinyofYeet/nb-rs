use std::path::PathBuf;

use crate::{
    core::{
        Nb, NbError,
        models::{
            note::Note, note_meta::NoteMetaInformation, notebook::Notebook,
            notebook_meta::NotebookMetaInformation,
        },
        sync_strategy::{SyncStrategy, meta::SyncMetaInformation, sync_kind::SyncKind},
    },
    default_strategies::sync::no_op::NoopSync,
};

impl Nb {
    pub fn save_sync_setup(
        &self,
        notebook: &mut Notebook,
        strategy: SyncMetaInformation,
    ) -> Result<(), NbError> {
        {
            notebook.get_meta_mut().set_sync_meta(strategy);
        }

        self.storage
            .save_notebook_meta(notebook.get_path(), notebook.get_meta())?;

        let meta = notebook
            .get_meta()
            .get_sync_information()
            .get_strategy(&*self.storage)?;

        meta.sync_full(notebook, &*self.storage)?;

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
            .save_notebook_meta(notebook.get_path(), notebook.get_meta())?;
        Ok(())
    }

    pub fn sync_note(&self, note: &Note) -> Result<(), NbError> {
        let meta = note.get_notebook().get_meta();
        let sync = meta.get_sync_information().get_strategy(&*self.storage)?;
        sync.sync_note(note, &*self.storage, SyncKind::Edit)?;
        Ok(())
    }

    pub fn sync_manual(&self, notebook: &Notebook) -> Result<(), NbError> {
        let sync = notebook
            .get_meta()
            .get_sync_information()
            .get_strategy(&*self.storage)?;
        sync.sync_full(notebook, &*self.storage)?;

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
}
