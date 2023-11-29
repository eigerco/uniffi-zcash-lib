use std::sync::Arc;

use zcash_primitives::sapling::keys::FullViewingKey;

use crate::{ZcashExpandedSpendingKey, ZcashOutgoingViewingKey, ZcashResult, ZcashViewingKey};
use derive_more::{From, Into};

#[derive(From, Into)]
pub struct ZcashFullViewingKey(FullViewingKey);

impl ZcashFullViewingKey {
    pub fn from_bytes(bytes: Vec<u8>) -> ZcashResult<Self> {
        FullViewingKey::read(bytes.as_slice())
            .map(From::from)
            .map_err(|error| error.to_string().into())
    }

    pub fn from_expanded_spending_key(expsk: Arc<ZcashExpandedSpendingKey>) -> Self {
        FullViewingKey::from_expanded_spending_key(&expsk.0).into()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    pub fn vk(&self) -> Arc<ZcashViewingKey> {
        Arc::new(self.0.vk.clone().into())
    }

    pub fn ovk(&self) -> Arc<ZcashOutgoingViewingKey> {
        Arc::new(self.0.ovk.into())
    }
}
