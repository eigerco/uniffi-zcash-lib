use std::sync::Arc;

use zcash_primitives::zip32::DiversifiableFullViewingKey;

use crate::{
    utils, ZcashDiversifier, ZcashDiversifierIndex, ZcashDiversifierIndexAndPaymentAddress,
    ZcashDiversifierIndexAndScope, ZcashError, ZcashFullViewingKey, ZcashNullifierDerivingKey,
    ZcashOutgoingViewingKey, ZcashPaymentAddress, ZcashResult, ZcashSaplingIvk, ZcashScope,
};

/// A Sapling key that provides the capability to view incoming and outgoing transactions.
///
/// This key is useful anywhere you need to maintain accurate balance, but do not want the
/// ability to spend funds (such as a view-only wallet).
///
/// It comprises the subset of the ZIP 32 extended full viewing key that is used for the
/// Sapling item in a [ZIP 316 Unified Full Viewing Key][zip-0316-ufvk].
///
/// [zip-0316-ufvk]: https://zips.z.cash/zip-0316#encoding-of-unified-full-incoming-viewing-keys
pub struct ZcashDiversifiableFullViewingKey(DiversifiableFullViewingKey);

impl From<DiversifiableFullViewingKey> for ZcashDiversifiableFullViewingKey {
    fn from(key: DiversifiableFullViewingKey) -> Self {
        ZcashDiversifiableFullViewingKey(key)
    }
}

impl From<&ZcashDiversifiableFullViewingKey> for DiversifiableFullViewingKey {
    fn from(key: &ZcashDiversifiableFullViewingKey) -> Self {
        key.0.clone()
    }
}

impl ZcashDiversifiableFullViewingKey {
    /// Parses a `DiversifiableFullViewingKey` from its raw byte encoding.
    ///
    /// Returns `None` if the bytes do not contain a valid encoding of a diversifiable
    /// Sapling full viewing key.
    pub fn from_bytes(bytes: Vec<u8>) -> ZcashResult<Self> {
        let array = utils::cast_slice(&bytes)?;
        let key = DiversifiableFullViewingKey::from_bytes(&array).ok_or(ZcashError::Unknown)?;

        Ok(ZcashDiversifiableFullViewingKey(key))
    }
    /// Returns the raw encoding of this `DiversifiableFullViewingKey`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().into()
    }

    /// Exposes the external [`FullViewingKey`] component of this diversifiable full viewing key.
    pub fn fvk(&self) -> Arc<ZcashFullViewingKey> {
        Arc::new(self.0.fvk().clone().into())
    }

    /// Derives a nullifier-deriving key for the provided scope.
    ///
    /// This API is provided so that nullifiers for change notes can be correctly computed.
    pub fn to_nk(&self, scope: ZcashScope) -> Arc<ZcashNullifierDerivingKey> {
        Arc::new(self.0.to_nk(scope.into()).into())
    }

    /// Derives an incoming viewing key corresponding to this full viewing key.
    pub fn to_ivk(&self, scope: ZcashScope) -> Arc<ZcashSaplingIvk> {
        Arc::new(self.0.to_ivk(scope.into()).into())
    }

    /// Derives an outgoing viewing key corresponding to this full viewing key.
    pub fn to_ovk(&self, scope: ZcashScope) -> Arc<ZcashOutgoingViewingKey> {
        Arc::new(self.0.to_ovk(scope.into()).into())
    }

    /// Attempts to produce a valid payment address for the given diversifier index.
    ///
    /// Returns `None` if the diversifier index does not produce a valid diversifier for
    /// this `DiversifiableFullViewingKey`.
    pub fn address(&self, j: Arc<ZcashDiversifierIndex>) -> Option<Arc<ZcashPaymentAddress>> {
        self.0
            .address(j.as_ref().into())
            .map(From::from)
            .map(Arc::new)
    }

    /// Finds the next valid payment address starting from the given diversifier index.
    ///
    /// This searches the diversifier space starting at `j` and incrementing, to find an
    /// index which will produce a valid diversifier (a 50% probability for each index).
    ///
    /// Returns the index at which the valid diversifier was found along with the payment
    /// address constructed using that diversifier, or `None` if the maximum index was
    /// reached and no valid diversifier was found.
    pub fn find_address(
        &self,
        j: Arc<ZcashDiversifierIndex>,
    ) -> Option<ZcashDiversifierIndexAndPaymentAddress> {
        self.0.find_address(j.as_ref().into()).map(From::from)
    }

    /// Returns the payment address corresponding to the smallest valid diversifier index,
    /// along with that index.
    pub fn default_address(&self) -> ZcashDiversifierIndexAndPaymentAddress {
        self.0.default_address().into()
    }

    /// Returns the payment address corresponding to the specified diversifier, if any.
    ///
    /// In general, it is preferable to use `find_address` instead, but this method is
    /// useful in some cases for matching keys to existing payment addresses.
    pub fn diversified_address(
        &self,
        diversifier: Arc<ZcashDiversifier>,
    ) -> Option<Arc<ZcashPaymentAddress>> {
        self.0
            .diversified_address(diversifier.as_ref().into())
            .map(From::from)
            .map(Arc::new)
    }

    /// Returns the internal address corresponding to the smallest valid diversifier index,
    /// along with that index.
    ///
    /// This address **MUST NOT** be encoded and exposed to end users. User interfaces
    /// should instead mark these notes as "change notes" or "internal wallet operations".
    pub fn change_address(&self) -> ZcashDiversifierIndexAndPaymentAddress {
        self.0.change_address().into()
    }

    /// Returns the change address corresponding to the specified diversifier, if any.
    ///
    /// In general, it is preferable to use `change_address` instead, but this method is
    /// useful in some cases for matching keys to existing payment addresses.
    pub fn diversified_change_address(
        &self,
        diversifier: Arc<ZcashDiversifier>,
    ) -> Option<Arc<ZcashPaymentAddress>> {
        self.0
            .diversified_change_address(diversifier.as_ref().into())
            .map(From::from)
            .map(Arc::new)
    }

    /// Attempts to decrypt the given address's diversifier with this full viewing key.
    ///
    /// This method extracts the diversifier from the given address and decrypts it as a
    /// diversifier index, then verifies that this diversifier index produces the same
    /// address. Decryption is attempted using both the internal and external parts of the
    /// full viewing key.
    ///
    /// Returns the decrypted diversifier index and its scope, or `None` if the address
    /// was not generated from this key.
    pub fn decrypt_diversifier(
        &self,
        addr: Arc<ZcashPaymentAddress>,
    ) -> Option<ZcashDiversifierIndexAndScope> {
        self.0
            .decrypt_diversifier(&addr.as_ref().into())
            .map(From::from)
    }
}
