use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub data_dir: PathBuf,
    pub no_sync: bool,
    pub editor_cmd: String,
}
