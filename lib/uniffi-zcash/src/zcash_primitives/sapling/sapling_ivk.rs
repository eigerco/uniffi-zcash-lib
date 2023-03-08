use std::sync::Arc;

use zcash_primitives::sapling::SaplingIvk;

use crate::{ZcashDiversifier, ZcashPaymentAddress};

pub struct ZcashSaplingIvk(SaplingIvk);

impl From<SaplingIvk> for ZcashSaplingIvk {
    fn from(ivk: SaplingIvk) -> Self {
        ZcashSaplingIvk(ivk)
    }
}

impl ZcashSaplingIvk {
    pub fn to_payment_address(
        &self,
        diversifier: Arc<ZcashDiversifier>,
    ) -> Option<Arc<ZcashPaymentAddress>> {
        self.0
            .to_payment_address(diversifier.as_ref().into())
            .map(ZcashPaymentAddress::from)
            .map(Arc::new)
    }

    pub fn to_repr(&self) -> Vec<u8> {
        self.0.to_repr().into()
    }
}
