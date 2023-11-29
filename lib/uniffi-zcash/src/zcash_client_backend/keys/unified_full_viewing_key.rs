use std::sync::Arc;

use zcash_client_backend::keys::UnifiedFullViewingKey;

use crate::{
    ZcashAccountPubKey, ZcashConsensusParameters, ZcashDiversifiableFullViewingKey,
    ZcashDiversifierIndex, ZcashError, ZcashOrchardFullViewingKey, ZcashResult,
    ZcashUnifiedAddress, ZcashUnifiedAddressAndDiversifierIndex,
};

use derive_more::{From, Into};

/// A [ZIP 316](https://zips.z.cash/zip-0316) unified full viewing key.
#[derive(Clone, Debug, From, Into)]
pub struct ZcashUnifiedFullViewingKey(UnifiedFullViewingKey);

impl ZcashUnifiedFullViewingKey {
    /// Construct a new unified full viewing key, if the required components are present.
    pub fn new(
        transparent: Option<Arc<ZcashAccountPubKey>>,
        sapling: Option<Arc<ZcashDiversifiableFullViewingKey>>,
        orchard: Option<Arc<ZcashOrchardFullViewingKey>>,
    ) -> ZcashResult<Self> {
        let key = UnifiedFullViewingKey::new(
            transparent.as_deref().map(From::from),
            sapling.as_deref().map(From::from),
            orchard.as_deref().map(From::from),
        )
        .ok_or::<ZcashError>("unable to create key".to_string().into())?;
        Ok(ZcashUnifiedFullViewingKey(key))
    }

    /// Parses a `UnifiedFullViewingKey` from its [ZIP 316] string encoding.
    ///
    /// [ZIP 316]: https://zips.z.cash/zip-0316
    pub fn decode(params: ZcashConsensusParameters, encoding: &str) -> ZcashResult<Self> {
        let key = UnifiedFullViewingKey::decode(&params, encoding)?;

        Ok(key.into())
    }

    /// Returns the string encoding of this `UnifiedFullViewingKey` for the given network.
    pub fn encode(&self, params: ZcashConsensusParameters) -> String {
        self.0.encode(&params)
    }

    /// Returns the transparent component of the unified key at the
    /// BIP44 path `m/44'/<coin_type>'/<account>'`.
    pub fn transparent(&self) -> Option<Arc<ZcashAccountPubKey>> {
        self.0.transparent().cloned().map(From::from).map(Arc::new)
    }

    /// Returns the Sapling diversifiable full viewing key component of this unified key.
    pub fn sapling(&self) -> Option<Arc<ZcashDiversifiableFullViewingKey>> {
        self.0
            .sapling()
            .cloned()
            .map(ZcashDiversifiableFullViewingKey::from)
            .map(Arc::new)
    }

    /// Returns the Orchard full viewing key component of this unified key.
    pub fn orchard(&self) -> Option<Arc<ZcashOrchardFullViewingKey>> {
        self.0
            .orchard()
            .cloned()
            .map(ZcashOrchardFullViewingKey::from)
            .map(Arc::new)
    }

    /// Attempts to derive the Unified Address for the given diversifier index.
    ///
    /// Returns `None` if the specified index does not produce a valid diversifier.
    pub fn address(&self, j: Arc<ZcashDiversifierIndex>) -> Option<Arc<ZcashUnifiedAddress>> {
        self.0
            .address(j.as_ref().into())
            .map(From::from)
            .map(Arc::new)
    }

    /// Searches the diversifier space starting at diversifier index `j` for one which will
    /// produce a valid diversifier, and return the Unified Address constructed using that
    /// diversifier along with the index at which the valid diversifier was found.
    ///
    /// Returns `None` if no valid diversifier exists
    pub fn find_address(
        &self,
        j: Arc<ZcashDiversifierIndex>,
    ) -> Option<ZcashUnifiedAddressAndDiversifierIndex> {
        self.0.find_address(j.as_ref().into()).map(From::from)
    }

    /// Returns the Unified Address corresponding to the smallest valid diversifier index,
    /// along with that index.
    pub fn default_address(&self) -> ZcashUnifiedAddressAndDiversifierIndex {
        self.0.default_address().into()
    }
}
