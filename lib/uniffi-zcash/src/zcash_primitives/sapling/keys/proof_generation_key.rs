use std::sync::Arc;

use zcash_primitives::sapling::ProofGenerationKey;

use crate::ZcashViewingKey;

pub struct ZcashProofGenerationKey(ProofGenerationKey);

impl From<ProofGenerationKey> for ZcashProofGenerationKey {
    fn from(key: ProofGenerationKey) -> Self {
        ZcashProofGenerationKey(key)
    }
}

impl ZcashProofGenerationKey {
    pub fn to_viewing_key(&self) -> Arc<ZcashViewingKey> {
        Arc::new(self.0.to_viewing_key().into())
    }
}
