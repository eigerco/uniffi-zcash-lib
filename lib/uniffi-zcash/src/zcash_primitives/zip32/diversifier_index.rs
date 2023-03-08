use std::sync::RwLock;

use zcash_primitives::zip32::DiversifierIndex;

use crate::ZcashError;

pub struct ZcashDiversifierIndex(std::sync::RwLock<DiversifierIndex>);

impl ZcashDiversifierIndex {
    pub fn new() -> Self {
        DiversifierIndex::new().into()
    }

    pub fn increment(&self) -> Result<(), ZcashError> {
        self.0
            .write()
            .unwrap()
            .increment()
            .or(Err("overflow error".to_string().into()))
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
