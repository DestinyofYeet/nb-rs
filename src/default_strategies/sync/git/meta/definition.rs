use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitSyncMeta {
    pub(in crate::default_strategies::sync::git) repo_url: String,
    pub(in crate::default_strategies::sync::git) branch: String,
}
