use std::path::Path;

use crate::{
    core::Nb,
    default_strategies::storage::file_storage::{FileStorage, tests::get_temp},
};

fn test_notebook(nb: &Nb, name: &Path) -> Result<(), anyhow::Error> {
    let metadata_path = name.join(FileStorage::BOOK_METADATA_PATH);

    assert!(metadata_path.exists() && metadata_path.is_file());

    let storage = nb.get_storage();

    let _meta = storage.read_notebook_meta(&name.to_string_lossy())?;

    Ok(())
}

#[test]
fn create() -> Result<(), anyhow::Error> {
    let temp_dir = get_temp();

    let nb = Nb::new(FileStorage::new(temp_dir.clone())?);

    nb.create_notebook("testNotebook".to_string())?;

    test_notebook(&nb, &temp_dir.join("testNotebook"))?;

    Ok(())
}

#[test]
fn create_space() -> Result<(), anyhow::Error> {
    let temp_dir = get_temp();

    let nb = Nb::new(FileStorage::new(temp_dir.clone())?);

    nb.create_notebook("test notebook".to_string())?;

    let nb_path = temp_dir.join("test notebook");

    test_notebook(&nb, &nb_path)?;

    Ok(())
}
