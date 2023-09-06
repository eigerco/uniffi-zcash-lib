use std::sync::Arc;

use zcash_client_backend::address::UnifiedAddress;
use zcash_primitives::zip32::DiversifierIndex;

use crate::ZcashDiversifierIndex;

mod address;
pub use self::address::*;

mod encoding;
pub use self::encoding::*;

mod keys;
pub use self::keys::*;

mod fees;
pub use self::fees::*;

mod wallet;
pub use self::wallet::*;

mod zip321;
pub use self::zip321::*;

pub struct ZcashUnifiedAddressAndDiversifierIndex {
    pub address: Arc<ZcashUnifiedAddress>,
    pub diversifier_index: Arc<ZcashDiversifierIndex>,
}

impl From<(UnifiedAddress, DiversifierIndex)> for ZcashUnifiedAddressAndDiversifierIndex {
    fn from(value: (UnifiedAddress, DiversifierIndex)) -> Self {
        ZcashUnifiedAddressAndDiversifierIndex {
            address: Arc::new(value.0.into()),
            diversifier_index: Arc::new(value.1.into()),
        }
    }
}
