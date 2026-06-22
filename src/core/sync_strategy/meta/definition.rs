use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncMetaInformation {
    pub(crate) strategy_name: String,
    pub(crate) data: Value,
}
