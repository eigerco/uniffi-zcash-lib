use zcash_primitives::sapling::Diversifier;

use crate::{utils, ZcashResult};

pub struct ZcashDiversifier(Diversifier);

impl ZcashDiversifier {
    pub fn new(bytes: Vec<u8>) -> ZcashResult<Self> {
        let array = utils::cast_slice(&bytes)?;
        Ok(Diversifier(array).into())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0 .0.to_vec()
    }
}

impl From<ZcashDiversifier> for Diversifier {
    fn from(value: ZcashDiversifier) -> Self {
        value.0
    }
}

impl From<&ZcashDiversifier> for Diversifier {
    fn from(value: &ZcashDiversifier) -> Self {
        value.0
    }
}

impl From<Diversifier> for ZcashDiversifier {
    fn from(diversifier: Diversifier) -> Self {
        ZcashDiversifier(diversifier)
    }
}
