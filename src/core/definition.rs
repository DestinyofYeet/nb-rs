use std::marker::PhantomData;

use crate::core::storage_strategy::StorageStrategy;

pub struct Nb {
    pub(super) storage: Box<dyn StorageStrategy>,
}

impl Nb {
    pub fn new<ST>(storage: ST) -> Self
    where
        ST: StorageStrategy + 'static,
    {
        Self {
            storage: Box::new(storage),
        }
    }
}
