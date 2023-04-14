use orchard::keys::OutgoingViewingKey;

/// A key that provides the capability to recover outgoing transaction information from
/// the block chain.
pub struct ZcashOrchardOutgoingViewingKey(OutgoingViewingKey);

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
