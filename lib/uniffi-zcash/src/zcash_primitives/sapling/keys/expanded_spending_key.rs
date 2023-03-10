use std::sync::Arc;

use zcash_primitives::sapling::keys::ExpandedSpendingKey;

use crate::{ZcashProofGenerationKey, ZcashResult};

pub struct ZcashExpandedSpendingKey(pub ExpandedSpendingKey);

impl From<ExpandedSpendingKey> for ZcashExpandedSpendingKey {
    fn from(key: ExpandedSpendingKey) -> Self {
        ZcashExpandedSpendingKey(key)
    }
}

impl ZcashExpandedSpendingKey {
    pub fn from_spending_key(sk: Vec<u8>) -> Self {
        ExpandedSpendingKey::from_spending_key(&sk).into()
    }

    pub fn from_bytes(b: Vec<u8>) -> ZcashResult<Self> {
        ExpandedSpendingKey::from_bytes(&b)
            .map(From::from)
            .map_err(From::from)
    }

    pub fn proof_generation_key(&self) -> Arc<ZcashProofGenerationKey> {
        Arc::new(self.0.proof_generation_key().into())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}
