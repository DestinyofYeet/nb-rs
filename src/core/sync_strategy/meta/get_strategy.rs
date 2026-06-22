use crate::{
    core::sync_strategy::{SyncError, SyncStrategy, meta::SyncMetaInformation},
    default_strategies::sync::{git::GitSync, no_op::NoopSync},
};

impl SyncMetaInformation {
    pub fn get_strategy(&self) -> Result<Box<dyn SyncStrategy>, SyncError> {
        if self.strategy_name.as_str() == NoopSync::get_name() {
            return Ok(Box::new(NoopSync::from_metadata(self)));
        };

        if self.strategy_name.as_str() == GitSync::get_name() {
            return Ok(Box::new(GitSync::from_metadata(self)));
        }

        return Err(SyncError::SyncError(format!(
            "Unknown strategy {}",
            self.strategy_name
        )));
    }
}
