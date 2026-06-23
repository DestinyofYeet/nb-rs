use serde_json::Value;

use crate::{
    core::sync_strategy::{SyncStrategy, meta::SyncMetaInformation},
    default_strategies::sync::no_op::NoopSync,
};

impl SyncMetaInformation {
    pub fn new() -> Self {
        Self {
            strategy_name: NoopSync::get_name().to_string(),
            data: Value::Null,
        }
    }
}

impl Default for SyncMetaInformation {
    fn default() -> Self {
        Self::new()
    }
}
