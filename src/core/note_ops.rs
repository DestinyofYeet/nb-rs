use crate::core::{
    Nb, NbError,
    models::{note::Note, notebook::Notebook},
    sync_strategy::sync_kind::SyncKind,
};

use roxygen::roxygen;

impl Nb {
    #[roxygen]
    /// Create a note
    pub fn create_note<'a>(
        &self,
        /// Notebook to create the note in
        notebook: &'a mut Notebook,

        /// Title to give the note
        title: String,

        /// The path of the note (usually the filename) in the notebook
        path: &'a str,

        /// Sync?
        do_sync: bool,
    ) -> Result<(), NbError> {
        self.storage.create_note(notebook, title, path)?;

        let note = self
            .storage
            .get_note_by_path(notebook, path)?
            .expect("to get note");

        let sync = notebook
            .get_meta()
            .get_sync_information()
            .get_strategy(&*self.storage)?;

        if !do_sync {
            sync.sync_note(&note, &*self.storage, SyncKind::Create)?;
        }

        Ok(())
    }

    #[roxygen]
    /// Get a note
    pub fn get_note<'a>(
        &self,
        /// Notebook to look in
        notebook: &'a Notebook,
        /// Path of the note in the notebook
        note_path: &'a str,
    ) -> Result<Option<Note<'a>>, NbError> {
        Ok(self.storage.get_note_by_path(notebook, note_path)?)
    }

    #[roxygen]
    /// Save a note
    pub fn save_note(
        &self,
        /// Note to save
        note: &Note,
        /// Should the change be synced?
        do_sync: bool,
    ) -> Result<(), NbError> {
        let notebook = note.get_notebook();
        self.storage.save_note(notebook, note)?;

        let meta = notebook.get_meta().get_sync_information();

        let sync = meta.get_strategy(&*self.storage)?;

        if !do_sync {
            sync.sync_note(note, &*self.storage, SyncKind::Edit)?;
        }

        Ok(())
    }

    pub fn list_notes<'a>(&self, notebook: &'a Notebook) -> Result<Vec<Note<'a>>, NbError> {
        Ok(self.storage.list_notes(notebook)?)
    }

    #[roxygen]
    /// Deletes a note
    pub fn delete_note<'a>(
        &self,
        /// The notebook containing the note
        notebook: &'a mut Notebook,
        /// The path to the note in the notebook
        note_path: &'a str,

        no_sync: bool,
    ) -> Result<(), NbError> {
        let sync = notebook
            .get_meta()
            .get_sync_information()
            .get_strategy(&*self.storage)?;

        if !no_sync {
            let note = self
                .storage
                .get_note_by_path(notebook, note_path)?
                .expect("to find note");

            sync.sync_note(&note, &*self.storage, SyncKind::Delete)?;
        }

        self.storage.delete_note(notebook, note_path)?;

        if !no_sync {
            sync.sync_full(notebook, &*self.storage)?;
        }

        Ok(())
    }

    pub fn rename_note_title(
        &self,
        notebook: &mut Notebook,
        note_path: &str,
        new_title: &str,
    ) -> Result<(), NbError> {
        Ok(self
            .storage
            .rename_note_title(notebook, note_path, new_title)?)
    }
}
