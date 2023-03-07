use std::sync::Arc;

use zcash_client_backend::keys::UnifiedFullViewingKey;

use crate::{
    ZcashConsensusParameters, ZcashDiversifiableFullViewingKey, ZcashOrchardFullViewingKey,
    ZcashResult,
};

/// A [ZIP 316](https://zips.z.cash/zip-0316) unified full viewing key.
pub struct ZcashUnifiedFullViewingKey(UnifiedFullViewingKey);

impl From<UnifiedFullViewingKey> for ZcashUnifiedFullViewingKey {
    fn from(key: UnifiedFullViewingKey) -> Self {
        ZcashUnifiedFullViewingKey(key)
    }
}

impl ZcashUnifiedFullViewingKey {
    /// Parses a `UnifiedFullViewingKey` from its [ZIP 316] string encoding.
    ///
    /// [ZIP 316]: https://zips.z.cash/zip-0316
    pub fn decode(params: ZcashConsensusParameters, encoding: &str) -> ZcashResult<Self> {
        let key = zcash_client_backend::keys::UnifiedFullViewingKey::decode(&params, encoding)?;

        Ok(key.into())
    }

    /// Returns the string encoding of this `UnifiedFullViewingKey` for the given network.
    pub fn encode(&self, params: ZcashConsensusParameters) -> String {
        self.0.encode(&params)
    }

    /// Returns the Sapling diversifiable full viewing key component of this unified key.
    pub fn sapling(&self) -> Option<Arc<ZcashDiversifiableFullViewingKey>> {
        self.0
            .sapling()
            .cloned()
            .map(ZcashDiversifiableFullViewingKey::from)
            .map(Arc::new)
    }

    pub fn orchard(&self) -> Option<Arc<ZcashOrchardFullViewingKey>> {
        self.0
            .orchard()
            .cloned()
            .map(ZcashOrchardFullViewingKey::from)
            .map(Arc::new)
    }
}
