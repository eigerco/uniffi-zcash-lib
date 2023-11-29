use std::sync::Arc;

use zcash_client_backend::encoding;
use zcash_primitives::{
    consensus::Parameters,
    zip32::{ChildIndex, ExtendedSpendingKey},
};

use crate::{
    ZcashChildIndex, ZcashConsensusParameters, ZcashDiversifiableFullViewingKey,
    ZcashDiversifierIndexAndPaymentAddress, ZcashError, ZcashResult,
};

use derive_more::{From, Into};

#[derive(From, Into, Clone)]
pub struct ZcashExtendedSpendingKey(ExtendedSpendingKey);

impl ZcashExtendedSpendingKey {
    /// Writes an [`ExtendedSpendingKey`] as a Bech32-encoded string.
    pub fn encode(&self, params: ZcashConsensusParameters) -> String {
        encoding::encode_extended_spending_key(params.hrp_sapling_extended_spending_key(), &self.0)
    }

    /// Decodes an [`ExtendedSpendingKey`] from a Bech32-encoded string.
    pub fn decode(params: ZcashConsensusParameters, input: &str) -> ZcashResult<Self> {
        encoding::decode_extended_spending_key(params.hrp_sapling_extended_spending_key(), input)
            .map_err(From::from)
            .map(From::from)
    }

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
    pub fn from_path(master: Arc<ZcashExtendedSpendingKey>, path: Vec<ZcashChildIndex>) -> Self {
        let inner_path: Vec<ChildIndex> = path.into_iter().map(|i| i.into()).collect();
        ExtendedSpendingKey::from_path(&(*master).clone().into(), inner_path.as_slice()).into()
    }

    /// Encodes the extended spending key to the its seralized representation as defined in
    /// [ZIP 32](https://zips.z.cash/zip-0032)
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

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
    pub fn derive_internal(&self) -> Arc<Self> {
        Arc::new(self.0.derive_internal().into())
    }

    pub fn to_diversifiable_full_viewing_key(&self) -> Arc<ZcashDiversifiableFullViewingKey> {
        Arc::new(self.0.to_diversifiable_full_viewing_key().into())
    }
}
