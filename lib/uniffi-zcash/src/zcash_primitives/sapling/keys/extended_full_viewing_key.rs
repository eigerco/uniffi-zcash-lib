use std::sync::Arc;

use zcash_client_backend::encoding;
use zcash_primitives::{consensus::Parameters, zip32::ExtendedFullViewingKey};

use crate::{
    ZcashChildIndex, ZcashConsensusParameters, ZcashDiversifiableFullViewingKey,
    ZcashDiversifierIndex, ZcashDiversifierIndexAndPaymentAddress, ZcashPaymentAddress,
    ZcashResult,
};

impl From<ExtendedFullViewingKey> for ZcashExtendedFullViewingKey {
    fn from(key: ExtendedFullViewingKey) -> Self {
        ZcashExtendedFullViewingKey(key)
    }
}

impl From<ZcashExtendedFullViewingKey> for ExtendedFullViewingKey {
    fn from(key: ZcashExtendedFullViewingKey) -> Self {
        key.0.clone()
    }
}

impl From<&ZcashExtendedFullViewingKey> for ExtendedFullViewingKey {
    fn from(key: &ZcashExtendedFullViewingKey) -> Self {
        key.0.clone()
    }
}

#[derive(Clone)]
pub struct ZcashExtendedFullViewingKey(ExtendedFullViewingKey);

impl ZcashExtendedFullViewingKey {
    /// Writes an [`ExtendedFullViewingKey`] as a Bech32-encoded string.
    pub fn encode(&self, params: ZcashConsensusParameters) -> String {
        encoding::encode_extended_full_viewing_key(
            params.hrp_sapling_extended_full_viewing_key(),
            &self.0,
        )
    }

    /// Decodes an [`ExtendedFullViewingKey`] from a Bech32-encoded string.
    pub fn decode(params: ZcashConsensusParameters, input: &str) -> ZcashResult<Self> {
        encoding::decode_extended_full_viewing_key(
            params.hrp_sapling_extended_full_viewing_key(),
            input,
        )
        .map_err(From::from)
        .map(From::from)
    }

    pub fn from_bytes(bytes: &[u8]) -> ZcashResult<Self> {
        ExtendedFullViewingKey::read(bytes)
            .map(From::from)
            .map_err(|error| error.to_string().into())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        self.0.write(&mut bytes).unwrap();
        bytes
    }

    pub fn derive_child(
        &self,
        i: ZcashChildIndex,
    ) -> ZcashResult<Arc<ZcashExtendedFullViewingKey>> {
        self.0
            .derive_child(i.into())
            .map_err(|_| "error ocurred while deriving child".into())
            .map(From::from)
            .map(Arc::new)
    }

    /// Attempt to produce a payment address given the specified diversifier
    /// index, and return None if the specified index does not produce a valid
    /// diversifier.
    pub fn address(&self, j: Arc<ZcashDiversifierIndex>) -> Option<Arc<ZcashPaymentAddress>> {
        self.0
            .address(j.as_ref().into())
            .map(From::from)
            .map(Arc::new)
    }

    /// Search the diversifier space starting at diversifier index `j` for
    /// one which will produce a valid diversifier, and return the payment address
    /// constructed using that diversifier along with the index at which the
    /// valid diversifier was found.
    pub fn find_address(
        &self,
        j: Arc<ZcashDiversifierIndex>,
    ) -> Option<ZcashDiversifierIndexAndPaymentAddress> {
        self.0.find_address(j.as_ref().into()).map(From::from)
    }

    /// Returns the payment address corresponding to the smallest valid diversifier
    /// index, along with that index.
    pub fn default_address(&self) -> ZcashDiversifierIndexAndPaymentAddress {
        self.0.default_address().into()
    }

    /// Derives an internal full viewing key used for internal operations such
    /// as change and auto-shielding. The internal FVK has the same spend authority
    /// (the private key corresponding to ak) as the original, but viewing authority
    /// only for internal transfers.
    ///
    /// Specified in [ZIP 32](https://zips.z.cash/zip-0032#deriving-a-sapling-internal-full-viewing-key).
    pub fn derive_internal(&self) -> Arc<ZcashExtendedFullViewingKey> {
        Arc::new(self.0.derive_internal().into())
    }

    pub fn to_diversifiable_full_viewing_key(&self) -> Arc<ZcashDiversifiableFullViewingKey> {
        Arc::new(self.0.to_diversifiable_full_viewing_key().into())
    }
}
