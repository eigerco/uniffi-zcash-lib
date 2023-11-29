use zcash_primitives::sapling::keys::OutgoingViewingKey;

use crate::{utils, ZcashResult};

use derive_more::{From, Into};

/// An outgoing viewing key
#[derive(From, Into)]
pub struct ZcashOutgoingViewingKey(OutgoingViewingKey);

impl ZcashOutgoingViewingKey {
    pub fn from_bytes(b: &[u8]) -> ZcashResult<Self> {
        let array = utils::cast_slice(b)?;
        Ok(ZcashOutgoingViewingKey(OutgoingViewingKey(array)))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0 .0.to_vec()
    }
}

impl From<&ZcashOutgoingViewingKey> for OutgoingViewingKey {
    fn from(value: &ZcashOutgoingViewingKey) -> Self {
        value.0
    }
}
