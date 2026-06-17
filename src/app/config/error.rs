use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("No EDITOR configured.")]
    NoEditor,

    #[error("Failed to find xdg data dir: {0}")]
    NoXdgDir(String),
}
