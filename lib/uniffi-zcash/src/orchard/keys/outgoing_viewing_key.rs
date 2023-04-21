use orchard::keys::OutgoingViewingKey;

use crate::{utils, ZcashResult};

/// A key that provides the capability to recover outgoing transaction information from
/// the block chain.
pub struct ZcashOrchardOutgoingViewingKey(pub(crate) OutgoingViewingKey);

impl From<OutgoingViewingKey> for ZcashOrchardOutgoingViewingKey {
    fn from(key: OutgoingViewingKey) -> Self {
        Self(key)
    }
}

impl From<&ZcashOrchardOutgoingViewingKey> for OutgoingViewingKey {
    fn from(value: &ZcashOrchardOutgoingViewingKey) -> Self {
        value.0.clone()
    }
}

impl ZcashOrchardOutgoingViewingKey {
    pub fn from_bytes(bytes: &[u8]) -> ZcashResult<Self> {
        let array = utils::cast_slice(bytes)?;
        Ok(OutgoingViewingKey::from(array).into())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }
}
