use std::sync::Arc;

use orchard::keys::FullViewingKey;

use crate::{ZcashOrchardIncomingViewingKey, ZcashOrchardOutgoingViewingKey, ZcashOrchardScope};

/// A key that provides the capability to view incoming and outgoing transactions.
///
/// This key is useful anywhere you need to maintain accurate balance, but do not want the
/// ability to spend funds (such as a view-only wallet).
pub struct ZcashOrchardFullViewingKey(FullViewingKey);

impl ZcashOrchardFullViewingKey {
    pub fn to_ivk(&self, scope: ZcashOrchardScope) -> Arc<ZcashOrchardIncomingViewingKey> {
        Arc::new(self.0.to_ivk(scope.into()).into())
    }
    pub fn to_ovk(&self, scope: ZcashOrchardScope) -> Arc<ZcashOrchardOutgoingViewingKey> {
        Arc::new(self.0.to_ovk(scope.into()).into())
    }
}

impl From<FullViewingKey> for ZcashOrchardFullViewingKey {
    fn from(key: FullViewingKey) -> Self {
        Self(key)
    }
}
