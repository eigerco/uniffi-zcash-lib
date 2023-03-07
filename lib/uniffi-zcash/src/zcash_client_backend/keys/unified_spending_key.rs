use std::sync::Arc;

use zcash_client_backend::keys::UnifiedSpendingKey;

use crate::{
    ZcashAccountId, ZcashConsensusParameters, ZcashKeysEra, ZcashResult, ZcashUnifiedFullViewingKey,
};

/// A set of viewing keys that are all associated with a single
/// ZIP-0032 account identifier.
pub struct ZcashUnifiedSpendingKey(UnifiedSpendingKey);

impl From<UnifiedSpendingKey> for ZcashUnifiedSpendingKey {
    fn from(key: UnifiedSpendingKey) -> Self {
        ZcashUnifiedSpendingKey(key)
    }
}

impl ZcashUnifiedSpendingKey {
    pub fn from_seed(
        params: ZcashConsensusParameters,
        seed: Vec<u8>,
        account: ZcashAccountId,
    ) -> ZcashResult<Self> {
        let key = UnifiedSpendingKey::from_seed(&params, &seed, account.into())?;

        Ok(key.into())
    }

    pub fn to_unified_full_viewing_key(&self) -> Arc<ZcashUnifiedFullViewingKey> {
        Arc::new(self.0.to_unified_full_viewing_key().into())
    }

    /// Returns a binary encoding of this key suitable for decoding with [`decode`].
    ///
    /// The encoded form of a unified spending key is only intended for use
    /// within wallets when required for storage and/or crossing FFI boundaries;
    /// unified spending keys should not be exposed to users, and consequently
    /// no string-based encoding is defined. This encoding does not include any
    /// internal validation metadata (such as checksums) as keys decoded from
    /// this form will necessarily be validated when the attempt is made to
    /// spend a note that they have authority for.
    pub fn to_bytes(&self, era: ZcashKeysEra) -> Vec<u8> {
        self.0.to_bytes(era.into())
    }
}
