use std::path::{Path, PathBuf};

use tracing::{debug, warn};

use crate::core::{
    models::{
        note::Note, note_meta::NoteMetaInformation, notebook::Notebook,
        notebook_meta::NotebookMetaInformation,
    },
    storage_strategy::{StorageError, StorageStrategy},
};

pub struct FileStorage {
    data_dir: PathBuf,
}

impl FileStorage {
    const BOOK_METADATA_PATH: &str = ".notebook_meta.json";

    fn note_metadata_path(path: &Path) -> PathBuf {
        let file_name = path.file_name().map(|e| e.to_str().unwrap()).unwrap();

        path.with_file_name(format!("{file_name}_meta.json"))
    }

    pub fn new(data_dir: PathBuf) -> Result<Self, StorageError> {
        if !data_dir.exists() {
            std::fs::create_dir(&data_dir).map_err(|e| {
                StorageError::StorageError(format!("Failed to create base directory: {e}"))
            })?;
        }

        Ok(Self { data_dir })
    }
}

impl StorageStrategy for FileStorage {
    type StoragePathType<'a> = PathBuf;

    fn list_notebooks(&self) -> Result<Vec<Notebook>, StorageError> {
        let path = self.data_dir.clone();

        let dir = std::fs::read_dir(&path)
            .map_err(|e| StorageError::GetNotebook(format!("Failed to read directory: {e}")))?;

        let mut notebooks: Vec<Notebook> = Vec::new();

        for entry in dir {
            let entry = entry
                .map_err(|e| StorageError::ListNotebooks(format!("Failed to read entry: {e}")))?;

            if entry
                .file_type()
                .map_err(|e| StorageError::ListNotebooks(format!("Failed to get file type: {e}")))?
                .is_dir()
            {
                match self.get_notebook(entry.file_name().to_string_lossy().to_string())? {
                    Some(value) => {
                        notebooks.push(value);
                    }
                    None => {
                        warn!("Directory {:?} is not a notebook.", entry.path())
                    }
                };
            }
        }

        Ok(notebooks)
    }

    fn create_notebook(
        &self,
        name: String,
    ) -> Result<(), crate::core::storage_strategy::StorageError> {
        let new_path = self.data_dir.join(&name);

        if new_path.exists() {
            return Err(StorageError::CreateNotebook(format!(
                "The notebook '{name}' already exists."
            )));
        }

        std::fs::create_dir(&new_path).map_err(|e| {
            StorageError::CreateNotebook(format!("Failed to create directory: {e}"))
        })?;

        let meta_information = NotebookMetaInformation::new();

        self.save_notebook_meta(&new_path, &meta_information)?;

        Ok(())
    }

    fn delete_notebook(
        &self,
        notebook: &Notebook,
    ) -> Result<(), crate::core::storage_strategy::StorageError> {
        let path = PathBuf::from(notebook.get_path());

        debug!("Removing directory {path:?} and everything beneath it");
        std::fs::remove_dir_all(&path)
            .map_err(|e| StorageError::DeleteNotebook(format!("Failed to delete notebook: {e}")))?;

        Ok(())
    }

    fn get_notebook(&self, name: String) -> Result<Option<Notebook>, StorageError> {
        let maybe_path = self.data_dir.join(&name);

        if maybe_path.is_dir() {
            let meta = self.read_notebook_meta(&maybe_path)?;
            return Ok(Some(Notebook::new(
                name,
                maybe_path.to_str().unwrap().to_string(),
                meta,
            )));
        }

        Ok(None)
    }

    fn list_notes<'a>(
        &self,
        notebook: &'a crate::core::models::notebook::Notebook,
    ) -> Result<Vec<crate::core::models::note::Note<'a>>, crate::core::storage_strategy::StorageError>
    {
        let notes = notebook.get_meta().get_notes();

        let mut results = Vec::new();

        for note in notes {
            results.push(
                match self.get_note_by_path(notebook, &PathBuf::from(note))? {
                    Some(value) => value,
                    None => {
                        return Err(StorageError::ListNotes(format!(
                            "Note {note} should exist, but it doesnt."
                        )));
                    }
                },
            );
        }

        Ok(results)
    }

    fn search_notes<'a>(
        &self,
        notebook: &'a crate::core::models::notebook::Notebook,
        search_term: String,
    ) -> Result<Vec<crate::core::models::note::Note<'a>>, crate::core::storage_strategy::StorageError>
    {
        todo!()
    }

    fn create_note<'a>(
        &self,
        notebook: &'a mut Notebook,
        title: String,
        path: &Self::StoragePathType<'a>,
    ) -> Result<(), crate::core::storage_strategy::StorageError> {
        let mut note_path = PathBuf::from(notebook.get_path());
        note_path.push(path);

        std::fs::write(&note_path, [])
            .map_err(|e| StorageError::CreateNote(format!("Failed to create note: {e}")))?;

        let note_meta = NoteMetaInformation::new(title);

        self.save_note_meta(&note_path, &note_meta)?;

        notebook
            .get_meta_mut()
            .add_note(path.to_str().unwrap().to_string());

        let meta = notebook.get_meta();
        let path = notebook.get_path();

        self.save_notebook_meta(&PathBuf::from(path), meta)?;

        Ok(())
    }

    fn delete_note<'a>(
        &self,
        notebook: &'a mut Notebook,
        note_path: &Self::StoragePathType<'a>,
    ) -> Result<(), crate::core::storage_strategy::StorageError> {
        std::fs::remove_file(note_path)
            .map_err(|e| StorageError::DeleteNote(format!("Failed to delete note: {e}")))?;

        std::fs::remove_file(FileStorage::note_metadata_path(note_path))
            .map_err(|e| StorageError::DeleteNote(format!("Failed to delete metadata: {e}")))?;

        if !notebook.get_meta_mut().remove_note(
            &note_path
                .file_name()
                .expect("to have a filename")
                .to_string_lossy(),
        ) {
            return Err(StorageError::DeleteNote(format!(
                "Path {note_path:?} didn't get removed from metadata"
            )));
        };

        self.save_notebook_meta(&PathBuf::from(notebook.get_path()), notebook.get_meta())?;

        Ok(())
    }

    fn save_note(
        &self,
        _notebook: &Notebook,
        note: &crate::core::models::note::Note,
    ) -> Result<(), crate::core::storage_strategy::StorageError> {
        self.save_note_meta(&PathBuf::from(note.get_path()), note.get_metadata())?;
        Ok(())
    }

    fn get_note_by_path<'a>(
        &self,
        notebook: &'a Notebook,
        note_path: &Self::StoragePathType<'a>,
    ) -> Result<Option<Note<'a>>, StorageError> {
        let mut maybe_path = PathBuf::from(notebook.get_path());
        maybe_path.push(note_path);

        if !maybe_path.exists() {
            return Ok(None);
        }

        let meta = self.read_note_meta(&maybe_path)?;
        let note = Note::new(maybe_path.to_str().unwrap().to_string(), notebook, meta);

        Ok(Some(note))
    }

    fn save_note_meta<'a>(
        &self,
        note_path: &Self::StoragePathType<'a>,
        meta: &crate::core::models::note_meta::NoteMetaInformation,
    ) -> Result<(), StorageError> {
        let path = Self::note_metadata_path(note_path);

        let json = serde_json::to_string_pretty(&meta).map_err(|e| {
            StorageError::SaveNoteMeta(format!("Failed to serialize metadata: {e}"))
        })?;

        std::fs::write(path, json)
            .map_err(|e| StorageError::SaveNoteMeta(format!("Failed to write metadata: {e}")))?;

        Ok(())
    }

    fn read_note_meta<'a>(
        &self,
        note_path: &Self::StoragePathType<'a>,
    ) -> Result<crate::core::models::note_meta::NoteMetaInformation, StorageError> {
        let path = Self::note_metadata_path(note_path);

        let content = std::fs::read_to_string(path)
            .map_err(|e| StorageError::ReadNoteMeta(format!("Failed to read metadata: {e}")))?;

        serde_json::from_str(&content)
            .map_err(|e| StorageError::ReadNoteMeta(format!("Failed to deserialize metadata: {e}")))
    }

    fn save_notebook_meta<'a>(
        &self,
        notebook_path: &Self::StoragePathType<'a>,
        meta: &NotebookMetaInformation,
    ) -> Result<(), StorageError> {
        let mut path = PathBuf::from(notebook_path);

        path.push(FileStorage::BOOK_METADATA_PATH);

        let json = serde_json::to_string_pretty(&meta).map_err(|e| {
            StorageError::SaveNotebookMeta(format!("Failed to serialize metadata: {e}"))
        })?;

        std::fs::write(path, json).map_err(|e| {
            StorageError::SaveNotebookMeta(format!("Failed to write metadata: {e}"))
        })?;

        Ok(())
    }

    fn read_notebook_meta<'a>(
        &self,
        notebook_path: &Self::StoragePathType<'a>,
    ) -> Result<NotebookMetaInformation, StorageError> {
        let mut path = PathBuf::from(notebook_path);

        path.push(FileStorage::BOOK_METADATA_PATH);

        let contents = std::fs::read_to_string(&path).map_err(|e| {
            StorageError::ReadNotebookMeta(format!("Failed to read meta information: {e}"))
        })?;

        let meta: NotebookMetaInformation = serde_json::from_str(&contents).map_err(|e| {
            StorageError::ReadNotebookMeta(format!("Failed to deserialize meta information: {e}"))
        })?;

        Ok(meta)
    }

    fn get_path_on_fs<'a>(
        &self,
        notebook: &Notebook,
        path: &Self::StoragePathType<'a>,
    ) -> Result<PathBuf, StorageError> {
        let mut book_path = PathBuf::from(notebook.get_path());
        book_path.push(path);

        if !book_path.exists() {
            return Err(StorageError::PathOnFs(format!(
                "Path {book_path:?} does not exist!"
            )));
        }

        Ok(book_path)
    }

    fn get_root_path_on_fs<'a>(&self) -> Result<PathBuf, StorageError> {
        Ok(self.data_dir.clone())
    }

    fn list_files(&self, notebook: &Notebook) -> Result<Vec<String>, StorageError> {
        let mut files = Vec::new();

        let dir = std::fs::read_dir(notebook.get_path())
            .map_err(|e| StorageError::ListFiles(format!("Failed to read directory: {e}")))?;

        for entry in dir {
            let entry =
                entry.map_err(|e| StorageError::ListFiles(format!("Failed to get entry: {e}")))?;

            if entry
                .file_type()
                .map_err(|e| StorageError::ListFiles(format!("Failed to get file type: {e}")))?
                .is_file()
            {
                files.push(entry.file_name().to_string_lossy().to_string());
            }
        }

        Ok(files)
    }
}
