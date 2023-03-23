use std::sync::Arc;

use zcash_primitives::sapling::PaymentAddress;
use zcash_primitives::zip32::DiversifierIndex;
use zcash_primitives::zip32::Scope;

mod consensus;
pub use self::consensus::*;

mod legacy;
pub use self::legacy::*;

mod memo;
pub use self::memo::*;

mod sapling;
pub use self::sapling::*;

mod transaction;
pub use self::transaction::*;

mod zip32;
pub use self::zip32::*;

pub struct ZcashDiversifierIndexAndPaymentAddress {
    pub diversifier_index: Arc<ZcashDiversifierIndex>,
    pub address: Arc<ZcashPaymentAddress>,
}

impl From<(DiversifierIndex, PaymentAddress)> for ZcashDiversifierIndexAndPaymentAddress {
    fn from(elems: (DiversifierIndex, PaymentAddress)) -> Self {
        ZcashDiversifierIndexAndPaymentAddress {
            diversifier_index: Arc::new(elems.0.into()),
            address: Arc::new(elems.1.into()),
        }
    }
}

pub struct ZcashDiversifierIndexAndScope {
    pub diversifier_index: Arc<ZcashDiversifierIndex>,
    pub scope: ZcashScope,
}

impl From<(DiversifierIndex, Scope)> for ZcashDiversifierIndexAndScope {
    fn from(elems: (DiversifierIndex, Scope)) -> Self {
        ZcashDiversifierIndexAndScope {
            diversifier_index: Arc::new(elems.0.into()),
            scope: elems.1.into(),
        }
    }
}
