use zcash_primitives::zip32::ExtendedSpendingKey;

use crate::ZcashResult;

/// A Sapling extended spending key
pub struct ZcashExtendedSpendingKey(ExtendedSpendingKey);

impl From<ExtendedSpendingKey> for ZcashExtendedSpendingKey {
    fn from(key: ExtendedSpendingKey) -> Self {
        ZcashExtendedSpendingKey(key)
    }
}

impl ZcashExtendedSpendingKey {
    pub fn master(_seed: &[u8]) -> Self {
        todo!()
    }

    /// Decodes the extended spending key from its serialized representation as defined in
    /// [ZIP 32](https://zips.z.cash/zip-0032)
    pub fn from_bytes(_bytes: Vec<u8>) -> ZcashResult<Self> {
        todo!()
    }

    /// Encodes the extended spending key to the its seralized representation as defined in
    /// [ZIP 32](https://zips.z.cash/zip-0032)
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    /*
    /// Returns the child key corresponding to the path derived from the master key
    pub fn from_path(master: &ExtendedSpendingKey, path: &[ChildIndex]) -> Self {
    }

    #[must_use]
    pub fn derive_child(&self, i: ChildIndex) -> Self {
    }

    /// Returns the address with the lowest valid diversifier index, along with
    /// the diversifier index that generated that address.
    pub fn default_address(&self) -> (DiversifierIndex, PaymentAddress) {
    }
    */

    /// Derives an internal spending key given an external spending key.
    ///
    /// Specified in [ZIP 32](https://zips.z.cash/zip-0032#deriving-a-sapling-internal-spending-key).
    #[must_use]
    pub fn derive_internal(&self) -> Self {
        todo!()
    }

    /*
    pub fn to_diversifiable_full_viewing_key(&self) -> DiversifiableFullViewingKey {
        todo!()
    }
    */
}
