use std::sync::Arc;

use zcash_primitives::zip32::{ChildIndex, ExtendedSpendingKey};

use crate::{
    ZcashChildIndex, ZcashDiversifiableFullViewingKey, ZcashDiversifierIndexAndPaymentAddress,
    ZcashError,
};

pub struct ZcashExtendedSpendingKey(ExtendedSpendingKey);

impl ZcashExtendedSpendingKey {
    pub fn master(seed: Vec<u8>) -> Self {
        ExtendedSpendingKey::master(seed.as_slice()).into()
    }

    /// Decodes the extended spending key from its serialized representation as defined in
    /// [ZIP 32](https://zips.z.cash/zip-0032)
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, ZcashError> {
        let key = ExtendedSpendingKey::from_bytes(bytes.as_slice())
            .or(Err("decoding error".to_string()))?;
        Ok(key.into())
    }

    /// Returns the child key corresponding to the path derived from the master key
    pub fn from_path(master: &ZcashExtendedSpendingKey, path: Vec<ZcashChildIndex>) -> Self {
        let inner_path: Vec<ChildIndex> = path.into_iter().map(|i| i.into()).collect();
        ExtendedSpendingKey::from_path(&master.into(), inner_path.as_slice()).into()
    }

    /// Encodes the extended spending key to the its seralized representation as defined in
    /// [ZIP 32](https://zips.z.cash/zip-0032)
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    #[must_use]
    pub fn derive_child(&self, index: ZcashChildIndex) -> Arc<Self> {
        Arc::new(self.0.derive_child(index.into()).into())
    }

    /// Returns the address with the lowest valid diversifier index, along with
    /// the diversifier index that generated that address.
    pub fn default_address(&self) -> ZcashDiversifierIndexAndPaymentAddress {
        self.0.default_address().into()
    }

    /// Derives an internal spending key given an external spending key.
    ///
    /// Specified in [ZIP 32](https://zips.z.cash/zip-0032#deriving-a-sapling-internal-spending-key).
    #[must_use]
    pub fn derive_internal(&self) -> Arc<Self> {
        Arc::new(self.0.derive_internal().into())
    }

    pub fn to_diversifiable_full_viewing_key(&self) -> Arc<ZcashDiversifiableFullViewingKey> {
        Arc::new(self.0.to_diversifiable_full_viewing_key().into())
    }
}

impl From<&ZcashExtendedSpendingKey> for ExtendedSpendingKey {
    fn from(value: &ZcashExtendedSpendingKey) -> Self {
        value.0.clone()
    }
}

impl From<ExtendedSpendingKey> for ZcashExtendedSpendingKey {
    fn from(inner: ExtendedSpendingKey) -> Self {
        ZcashExtendedSpendingKey(inner)
    }
}
