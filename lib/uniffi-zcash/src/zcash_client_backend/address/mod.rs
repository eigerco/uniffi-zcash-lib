use std::sync::Arc;
use zcash_client_backend::address::AddressMetadata;

use crate::{ZcashAccountId, ZcashDiversifierIndex};

mod recipient_address;
pub use self::recipient_address::*;

mod unified_address;
pub use self::unified_address::*;

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

impl From<AddressMetadata> for ZcashAddressMetadata {
    fn from(inner: AddressMetadata) -> Self {
        ZcashAddressMetadata(inner)
    }
}

impl From<ZcashAddressMetadata> for AddressMetadata {
    fn from(outer: ZcashAddressMetadata) -> Self {
        outer.0
    }
}
