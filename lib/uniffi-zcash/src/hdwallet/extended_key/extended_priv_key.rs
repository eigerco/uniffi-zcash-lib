use std::sync::Arc;

use hdwallet::{
    extended_key::ExtendedPrivKey,
    traits::{Deserialize, Serialize},
    KeySeed,
};
use rand::thread_rng;

use crate::{ZcashKeyIndex, ZcashResult};

/// Indicate bits of random seed used to generate private key, 256 is recommended.
pub enum ZcashKeySeed {
    S128 = 128,
    S256 = 256,
    S512 = 512,
}

impl From<ZcashKeySeed> for KeySeed {
    fn from(value: ZcashKeySeed) -> Self {
        match value {
            ZcashKeySeed::S128 => KeySeed::S128,
            ZcashKeySeed::S256 => KeySeed::S256,
            ZcashKeySeed::S512 => KeySeed::S512,
        }
    }
}

/// ExtendedPrivKey is used for child key derivation.
/// See [secp256k1 crate documentation](https://docs.rs/secp256k1) for SecretKey signatures usage.
pub struct ZcashExtendedPrivKey(pub ExtendedPrivKey);

impl ZcashExtendedPrivKey {
    /// Generate an ExtendedPrivKey, use 256 size random seed.
    ///
    /// Note: it uses [`ThreadRng`](https://docs.rs/rand/latest/rand/rngs/struct.ThreadRng.html)
    /// obtained with [`thread_rng`](https://docs.rs/rand/latest/rand/fn.thread_rng.html) as
    /// random number generator.
    pub fn random() -> ZcashResult<Self> {
        ExtendedPrivKey::random(&mut thread_rng())
            .map_err(From::from)
            .map(From::from)
    }

    /// Generate an ExtendedPrivKey which use 128 or 256 or 512 bits random seed.
    ///
    /// Note: it uses [`ThreadRng`](https://docs.rs/rand/latest/rand/rngs/struct.ThreadRng.html)
    /// obtained with [`thread_rng`](https://docs.rs/rand/latest/rand/fn.thread_rng.html) as
    /// random number generator.
    pub fn random_with_seed_size(seed_size: ZcashKeySeed) -> ZcashResult<Self> {
        ExtendedPrivKey::random_with_seed_size(&mut thread_rng(), seed_size.into())
            .map_err(From::from)
            .map(From::from)
    }

    /// Generate an ExtendedPrivKey from seed
    pub fn with_seed(seed: Vec<u8>) -> ZcashResult<Self> {
        let key = ExtendedPrivKey::with_seed(&seed)?;
        Ok(key.into())
    }

    /// Derive a child key from ExtendedPrivKey.
    pub fn derive_private_key(
        &self,
        key_index: Arc<ZcashKeyIndex>,
    ) -> ZcashResult<Arc<ZcashExtendedPrivKey>> {
        self.0
            .derive_private_key(key_index.as_ref().into())
            .map_err(From::from)
            .map(From::from)
            .map(Arc::new)
    }

    pub fn from_bytes(bytes: &[u8]) -> ZcashResult<Self> {
        ExtendedPrivKey::deserialize(bytes)
            .map_err(From::from)
            .map(From::from)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.serialize()
    }
}

impl From<ExtendedPrivKey> for ZcashExtendedPrivKey {
    fn from(key: ExtendedPrivKey) -> Self {
        Self(key)
    }
}
