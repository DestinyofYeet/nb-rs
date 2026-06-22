use crate::default_strategies::sync::git::meta::GitSyncMeta;

impl GitSyncMeta {
    pub fn new(repo_url: String, branch: String) -> Self {
        Self { repo_url, branch }
    }
}
