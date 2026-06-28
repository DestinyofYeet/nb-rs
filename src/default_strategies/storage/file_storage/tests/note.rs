use crate::{
    core::models::note_path::NoteFilename,
    default_strategies::storage::file_storage::{
        FileStorage,
        tests::{get_temp, notebook::test_notebook},
    },
};

#[test]
fn create() -> Result<(), anyhow::Error> {
    let temp = get_temp();

    let nb = test_notebook(temp.clone(), "test_notebook")?;

    let mut notebook = nb
        .get_notebook("test_notebook")?
        .expect("to have a notebook");

    let test_notefilename = NoteFilename::new("testing.md".to_string());

    nb.create_note(
        &mut notebook,
        "Test Note".to_string(),
        &test_notefilename,
        false,
    )?;

    let note_path = temp.join("test_notebook").join("testing.md");

    assert!(note_path.exists() && note_path.is_file());

    let note_meta_path = FileStorage::note_metadata_path(&note_path);

    assert!(note_meta_path.exists() && note_meta_path.is_file());

    let note = nb
        .get_note(&notebook, &test_notefilename)?
        .expect("note to exist");

    let _meta = nb.get_storage().read_note_meta(note.get_path())?;

    Ok(())
}

#[test]
fn delete() -> Result<(), anyhow::Error> {
    let temp = get_temp();

    let nb = test_notebook(temp.clone(), "test_notebook")?;

    let mut notebook = nb.get_notebook("test_notebook")?.expect("to have notebook");

    let test_filename = NoteFilename::new("testing.md".to_string());

    nb.create_note(
        &mut notebook,
        "Test note".to_string(),
        &test_filename,
        false,
    )?;

    let note_path = temp.join("test_notebook").join("testing.md");

    assert!(note_path.exists());

    let note_meta_path = FileStorage::note_metadata_path(&note_path);

    assert!(note_meta_path.exists() && note_meta_path.is_file());

    nb.delete_note(&mut notebook, &test_filename, false)?;

    assert!(!note_path.exists());
    assert!(!note_meta_path.exists());

    Ok(())
}
