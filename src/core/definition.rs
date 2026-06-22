use std::marker::PhantomData;

use crate::core::storage_strategy::StorageStrategy;

pub struct Nb<'a, ST>
where
    ST: StorageStrategy,
{
    pub(super) storage: ST,
    _m: PhantomData<&'a ST>,
}

impl<'a, ST> Nb<'a, ST>
where
    ST: StorageStrategy,
{
    pub fn new(storage: ST) -> Self {
        Self {
            storage,
            _m: PhantomData,
        }
    }
}
