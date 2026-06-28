use std::path::PathBuf;

use crate::{
    core::Nb,
    default_strategies::storage::file_storage::{FileStorage, tests::get_temp},
};

pub fn test_notebook(dir: PathBuf, name: &str) -> Result<Nb, anyhow::Error> {
    let nb = Nb::new(FileStorage::new(dir.clone())?);

    nb.create_notebook(name.to_string())?;

    let notebook_path = dir.join(name);

    assert!(notebook_path.exists());

    let metadata_path = dir.join(name).join(FileStorage::BOOK_METADATA_PATH);

    assert!(metadata_path.exists() && metadata_path.is_file());

    let storage = nb.get_storage();

    let _meta = storage.read_notebook_meta(&notebook_path.to_string_lossy())?;

    Ok(nb)
}

#[test]
fn create() -> Result<(), anyhow::Error> {
    let temp_dir = get_temp();

    let _nb = test_notebook(temp_dir, "test_notebook")?;

    Ok(())
}

#[test]
fn create_space() -> Result<(), anyhow::Error> {
    let temp_dir = get_temp();

    test_notebook(temp_dir, "test notebook")?;

    Ok(())
}

#[test]
fn delete() -> Result<(), anyhow::Error> {
    let temp_dir = get_temp();

    let nb = test_notebook(temp_dir.clone(), "test")?;

    nb.delete_notebook(&nb.get_notebook("test")?.unwrap())?;

    assert!(!temp_dir.join("test").exists());

    Ok(())
}
