use crate::core::storage_strategy::StorageStrategy;
use roxygen::roxygen;

pub struct Nb {
    pub(super) storage: Box<dyn StorageStrategy>,
}

impl Nb {
    #[roxygen]
    /// Creates a new Nb
    pub fn new<ST>(
        /// The Storage backend. Needs to implement [StorageStrategy](crate::core::storage_strategy::StorageStrategy)
        storage: ST,
    ) -> Self
    where
        ST: StorageStrategy + 'static,
    {
        Self {
            storage: Box::new(storage),
        }
    }
}
