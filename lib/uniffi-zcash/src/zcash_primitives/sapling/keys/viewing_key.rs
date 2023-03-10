use std::sync::Arc;

use zcash_primitives::sapling::keys::ViewingKey;

use crate::{ZcashDiversifier, ZcashPaymentAddress, ZcashSaplingIvk};

pub struct ZcashViewingKey(ViewingKey);

impl From<ViewingKey> for ZcashViewingKey {
    fn from(key: ViewingKey) -> Self {
        ZcashViewingKey(key)
    }
}

impl ZcashViewingKey {
    pub fn ivk(&self) -> Arc<ZcashSaplingIvk> {
        Arc::new(self.0.ivk().into())
    }

    pub fn to_payment_address(
        &self,
        diversifier: Arc<ZcashDiversifier>,
    ) -> Option<Arc<ZcashPaymentAddress>> {
        self.0
            .to_payment_address(diversifier.as_ref().into())
            .map(From::from)
            .map(Arc::new)
    }
}
