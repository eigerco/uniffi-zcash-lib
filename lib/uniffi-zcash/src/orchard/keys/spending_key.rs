use std::sync::Arc;

use orchard::keys::SpendingKey;

use crate::{utils::cast_slice, ZcashError, ZcashOrchardFullViewingKey, ZcashResult};

pub struct ZcashOrchardSpendingKey(pub(crate) SpendingKey);

impl ZcashOrchardSpendingKey {
    pub fn from_bytes(data: Vec<u8>) -> ZcashResult<Self> {
        let slice = cast_slice(&data)?;
        let sk = SpendingKey::from_bytes(slice);
        if sk.is_none().into() {
            Err("Cannot generate key - invalid input".to_string().into())
        } else {
            Ok(sk.unwrap().into())
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    pub fn to_fvk(&self) -> Arc<ZcashOrchardFullViewingKey> {
        Arc::new(self.into())
    }

    pub fn from_zip32_seed(seed: Vec<u8>, coin_type: u32, account: u32) -> ZcashResult<Self> {
        let key = SpendingKey::from_zip32_seed(seed.as_slice(), coin_type, account)
            .map_err(ZcashError::from)?;
        Ok(key.into())
    }
}

impl From<SpendingKey> for ZcashOrchardSpendingKey {
    fn from(inner: SpendingKey) -> Self {
        Self(inner)
    }
}

impl From<&ZcashOrchardSpendingKey> for SpendingKey {
    fn from(value: &ZcashOrchardSpendingKey) -> Self {
        value.0
    }
}