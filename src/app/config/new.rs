use std::path::PathBuf;

use xdgdir::BaseDir;

use crate::app::{
    args::Args,
    config::{Config, ConfigError},
};

impl Config {
    pub fn new(args: &Args) -> Result<Self, ConfigError> {
        Ok(Self {
            data_dir: match &args.data_dir {
                Some(value) => PathBuf::clone(value),
                None => {
                    let dir =
                        BaseDir::new("nb-rs").map_err(|e| ConfigError::NoXdgDir(e.to_string()))?;
                    dir.data
                }
            },
            sync: !args.no_sync,
            editor_cmd: std::env::var("EDITOR").map_err(|_| ConfigError::NoEditor)?,
        })
    }
}
