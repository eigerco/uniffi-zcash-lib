use zcash_primitives::sapling::keys::OutgoingViewingKey;

use crate::{utils, ZcashResult};

/// An outgoing viewing key
 #[derive(Clone)]
pub struct ZcashOutgoingViewingKey(OutgoingViewingKey);

impl ZcashOutgoingViewingKey {
    pub fn from_bytes(b: &[u8]) -> ZcashResult<Self> {
        let array = utils::cast_slice(b)?;
        Ok(ZcashOutgoingViewingKey(OutgoingViewingKey(array)))
    }

    // TODO there is a mistake here?
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0 .0.to_vec()
    }
}

impl From<OutgoingViewingKey> for ZcashOutgoingViewingKey {
    fn from(key: OutgoingViewingKey) -> Self {
        ZcashOutgoingViewingKey(key)
    }
}
