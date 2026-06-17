use std::marker::PhantomData;

use crate::core::{storage_strategy::StorageStrategy, sync_strategy::SyncStrategy};

pub struct Nb<'a, ST, SY>
where
    ST: StorageStrategy<'a>,
    SY: SyncStrategy,
{
    pub(super) storage: ST,
    pub(super) sync: SY,
    _m: PhantomData<&'a ST>,
}

impl<'a, ST, SY> Nb<'a, ST, SY>
where
    ST: StorageStrategy<'a>,
    SY: SyncStrategy,
{
    pub fn new(storage: ST, sync: SY) -> Self {
        Self {
            storage,
            sync,
            _m: PhantomData,
        }
    }
}
