use std::sync::Arc;

use orchard::keys::IncomingViewingKey;

use crate::{ZcashOrchardAddress, ZcashOrchardDiversifier};

/// A key that provides the capability to detect and decrypt incoming notes from the block
/// chain, without being able to spend the notes or detect when they are spent.
///
/// This key is useful in situations where you only need the capability to detect inbound
/// payments, such as merchant terminals.
pub struct ZcashOrchardIncomingViewingKey(IncomingViewingKey);

impl ZcashOrchardIncomingViewingKey {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    pub fn address(&self, diversifier: Arc<ZcashOrchardDiversifier>) -> Arc<ZcashOrchardAddress> {
        Arc::new(self.0.address(diversifier.0).into())
    }
}

impl From<IncomingViewingKey> for ZcashOrchardIncomingViewingKey {
    fn from(key: IncomingViewingKey) -> Self {
        Self(key)
    }
}
