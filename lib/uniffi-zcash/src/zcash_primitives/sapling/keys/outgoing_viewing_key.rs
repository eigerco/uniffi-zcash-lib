use zcash_primitives::sapling::keys::OutgoingViewingKey;

/// An outgoing viewing key
pub struct ZcashOutgoingViewingKey(OutgoingViewingKey);

impl ZcashOutgoingViewingKey {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0 .0.to_vec()
    }
}

impl From<OutgoingViewingKey> for ZcashOutgoingViewingKey {
    fn from(key: OutgoingViewingKey) -> Self {
        ZcashOutgoingViewingKey(key)
    }
}
