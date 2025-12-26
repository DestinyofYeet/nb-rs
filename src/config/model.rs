use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub data_dir: PathBuf,
    pub editor: String,

    pub offline: bool,
}
