use std::sync::RwLock;

use zcash_primitives::zip32::DiversifierIndex;

use crate::{ZcashError, ZcashResult};

#[derive(Default)]
pub struct ZcashDiversifierIndex(std::sync::RwLock<DiversifierIndex>);

impl ZcashDiversifierIndex {
    pub fn new() -> Self {
        DiversifierIndex::new().into()
    }

    pub fn from_u32(i: u32) -> Self {
        let i: DiversifierIndex = i.into();
        i.into()
    }

    pub fn from_u64(i: u64) -> Self {
        let i: DiversifierIndex = i.into();
        i.into()
    }

    pub fn increment(&self) -> Result<(), ZcashError> {
        self.0
            .write()
            .unwrap()
            .increment()
            .or(Err("overflow error".to_string().into()))
    }

    pub fn to_u32(&self) -> ZcashResult<u32> {
        (*self.0.read().unwrap()).try_into().map_err(|_| {
            "failed to convert u32 into diversifier index"
                .to_string()
                .into()
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.read().unwrap().0.into()
    }
}

impl From<DiversifierIndex> for ZcashDiversifierIndex {
    fn from(inner: DiversifierIndex) -> Self {
        Self(RwLock::new(inner))
    }
}

impl From<&ZcashDiversifierIndex> for DiversifierIndex {
    fn from(value: &ZcashDiversifierIndex) -> Self {
        *value.0.read().unwrap()
    }
}
