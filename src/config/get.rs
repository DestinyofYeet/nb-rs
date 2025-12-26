use std::{env, path::PathBuf};

use resolve_path::PathResolveExt;
use thiserror::Error;

use crate::{args::top::Args, config::model::Config};

#[derive(Error, Debug)]
pub enum ConfigGetError {
    // #[error("no data_dir was passed in. Either use '-D' or use the config file!")]
    // NoDataDir,
    #[error("no EDITOR was found: {0}")]
    NoEditor(String),
}

impl Config {
    pub fn new(args: &Args) -> Result<Self, ConfigGetError> {
        let data_dir: PathBuf = match args.data_dir.clone() {
            None => {
                let path = "~/.nb-rs/".resolve();
                PathBuf::from(path)
            }
            Some(value) => value,
        };

        let editor = env::var("EDITOR").map_err(|e| ConfigGetError::NoEditor(e.to_string()))?;

        Ok(Config {
            data_dir,
            editor,
            offline: args.offline,

            is_test: false,
        })
    }
}
