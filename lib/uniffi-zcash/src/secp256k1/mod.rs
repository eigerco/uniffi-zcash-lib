use secp256k1::SecretKey;

use crate::{ZcashError, ZcashResult};

pub struct SecpSecretKey(SecretKey);

impl SecpSecretKey {
    /// Converts bytes to a secret key.
    pub fn new(data: Vec<u8>) -> ZcashResult<Self> {
        let key = SecretKey::from_slice(data.as_slice()).map_err(ZcashError::from)?;
        Ok(key.into())
    }

    /// Serializes the secret key as byte value.
    pub fn serialize_secret(&self) -> Vec<u8> {
        self.0.secret_bytes().to_vec()
    }
}

impl From<secp256k1::SecretKey> for SecpSecretKey {
    fn from(inner: secp256k1::SecretKey) -> Self {
        SecpSecretKey(inner)
    }
}

impl From<SecpSecretKey> for SecretKey {
    fn from(value: SecpSecretKey) -> Self {
        value.0
    }
}

impl From<&SecpSecretKey> for SecretKey {
    fn from(value: &SecpSecretKey) -> Self {
        value.0
    }
}
