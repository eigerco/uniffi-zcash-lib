use hdwallet::extended_key::ExtendedPrivKey;

use crate::ZcashResult;

/// ExtendedPrivKey is used for child key derivation.
/// See [secp256k1 crate documentation](https://docs.rs/secp256k1) for SecretKey signatures usage.
pub struct ZcashExtendedPrivKey(pub ExtendedPrivKey);

impl ZcashExtendedPrivKey {
    pub fn with_seed(seed: Vec<u8>) -> ZcashResult<Self> {
        let key = ExtendedPrivKey::with_seed(&seed)?;
        Ok(key.into())
    }
}

impl From<ExtendedPrivKey> for ZcashExtendedPrivKey {
    fn from(key: ExtendedPrivKey) -> Self {
        Self(key)
    }
}
