use std::marker::PhantomData;

use crate::core::{storage_strategy::StorageStrategy, sync_strategy::SyncStragegy};

pub struct Nb<'a, ST, SY>
where
    ST: StorageStrategy<'a>,
    SY: SyncStragegy,
{
    storage: ST,
    sync: SY,
    _m: PhantomData<&'a ST>,
}

impl<'a, ST, SY> Nb<'a, ST, SY>
where
    ST: StorageStrategy<'a>,
    SY: SyncStragegy,
{
    pub fn new(storage: ST, sync: SY) -> Self {
        Self {
            storage,
            sync,
            _m: PhantomData,
        }
    }
}
