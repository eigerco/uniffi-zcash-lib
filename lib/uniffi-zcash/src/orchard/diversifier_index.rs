use orchard::keys::DiversifierIndex;

use crate::{utils, ZcashResult};

pub struct ZcashOrchardDiversifierIndex(DiversifierIndex);

impl ZcashOrchardDiversifierIndex {
    pub fn from_bytes(b: Vec<u8>) -> ZcashResult<Self> {
        let array = utils::cast_slice(&b)?;
        Ok(ZcashOrchardDiversifierIndex(array.into()))
    }

    pub fn from_u32(i: u32) -> Self {
        let i: DiversifierIndex = i.into();
        i.into()
    }

    pub fn from_u64(i: u64) -> Self {
        let i: DiversifierIndex = i.into();
        i.into()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

impl From<DiversifierIndex> for ZcashOrchardDiversifierIndex {
    fn from(inner: DiversifierIndex) -> Self {
        Self(inner)
    }
}

impl From<&ZcashOrchardDiversifierIndex> for DiversifierIndex {
    fn from(value: &ZcashOrchardDiversifierIndex) -> Self {
        value.0
    }
}
