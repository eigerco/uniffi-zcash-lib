use std::sync::Arc;

use zcash_client_backend::keys::UnifiedSpendingKey;

use crate::{
    ZcashAccountId, ZcashAccountPrivKey, ZcashConsensusParameters, ZcashExtendedSpendingKey,
    ZcashKeysEra, ZcashOrchardSpendingKey, ZcashResult, ZcashUnifiedFullViewingKey,
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

    /// Returns the transparent component of the unified key at the
    /// BIP44 path `m/44'/<coin_type>'/<account>'`.
    pub fn transparent(&self) -> Arc<ZcashAccountPrivKey> {
        Arc::new(self.0.transparent().clone().into())
    }

    /// Returns the Sapling extended spending key component of this unified spending key.
    pub fn sapling(&self) -> Arc<ZcashExtendedSpendingKey> {
        Arc::new(self.0.sapling().clone().into())
    }

    /// Returns the Orchard spending key component of this unified spending key.
    pub fn orchard(&self) -> Arc<ZcashOrchardSpendingKey> {
        Arc::new((*self.0.orchard()).into())
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

    /// Decodes a [`UnifiedSpendingKey`] value from its serialized representation.
    ///
    /// See [`to_bytes`] for additional detail about the encoded form.
    pub fn from_bytes(era: ZcashKeysEra, encoded: &[u8]) -> ZcashResult<Self> {
        UnifiedSpendingKey::from_bytes(era.into(), encoded)
            .map_err(From::from)
            .map(From::from)
    }
}
