use orchard::keys::OutgoingViewingKey;

/// A key that provides the capability to recover outgoing transaction information from
/// the block chain.
pub struct ZcashOrchardOutgoingViewingKey(OutgoingViewingKey);

impl From<OutgoingViewingKey> for ZcashOrchardOutgoingViewingKey {
    fn from(key: OutgoingViewingKey) -> Self {
        Self(key)
    }
}
