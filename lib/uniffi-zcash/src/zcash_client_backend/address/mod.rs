use std::sync::Arc;
use zcash_client_backend::address::AddressMetadata;

use derive_more::{From, Into};

use crate::{ZcashAccountId, ZcashDiversifierIndex};

mod recipient_address;
pub use self::recipient_address::*;

mod unified_address;
pub use self::unified_address::*;

#[derive(From, Into)]
pub struct ZcashAddressMetadata(AddressMetadata);

impl ZcashAddressMetadata {
    pub fn new(account: ZcashAccountId, diversifier_index: Arc<ZcashDiversifierIndex>) -> Self {
        Self(AddressMetadata::new(
            account.into(),
            (&(*diversifier_index)).into(),
        ))
    }

    pub fn account(&self) -> ZcashAccountId {
        self.0.account().into()
    }

    pub fn diversifier_index(&self) -> Arc<ZcashDiversifierIndex> {
        Arc::new((*self.0.diversifier_index()).into())
    }
}
