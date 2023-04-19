use std::sync::Arc;

use zcash_primitives::legacy::{
    keys::{ExternalIvk, IncomingViewingKey, InternalIvk},
    TransparentAddress,
};

use crate::{utils, ZcashResult, ZcashTransparentAddress};

/// A type representing an incoming viewing key at the BIP-44 "external"
/// path `m/44'/<coin_type>'/<account>'/0`. This allows derivation
/// of child addresses that may be provided to external parties.
pub struct ZcashExternalIvk(ExternalIvk);

impl ZcashExternalIvk {
    pub fn derive_address(&self, child_index: u32) -> ZcashResult<Arc<ZcashTransparentAddress>> {
        self.0.derive_address(child_index).map_err(From::from).map(From::from).map(Arc::new)
    }

    /// Searches the space of child indexes for an index that will
    /// generate a valid transparent address, and returns the resulting
    /// address and the index at which it was generated.
    pub fn default_address(&self) -> ZcashTransparentAddressAndIndex {
        self.0.default_address().into()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.serialize()
    }

    pub fn from_bytes(data: &[u8]) -> ZcashResult<Self> {
        let array = utils::cast_slice(data)?;
        ExternalIvk::deserialize(&array)
            .map_err(From::from)
            .map(From::from)
    }
}

impl From<ExternalIvk> for ZcashExternalIvk {
    fn from(inner: ExternalIvk) -> Self {
        ZcashExternalIvk(inner)
    }
}

pub struct ZcashInternalIvk(InternalIvk);

impl ZcashInternalIvk {}

impl From<InternalIvk> for ZcashInternalIvk {
    fn from(inner: InternalIvk) -> Self {
        ZcashInternalIvk(inner)
    }
}

impl ZcashInternalIvk {
    /// Searches the space of child indexes for an index that will
    /// generate a valid transparent address, and returns the resulting
    /// address and the index at which it was generated.
    pub fn default_address(&self) -> ZcashTransparentAddressAndIndex {
        self.0.default_address().into()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.serialize()
    }

    pub fn from_bytes(data: &[u8]) -> ZcashResult<Self> {
        let array = utils::cast_slice(data)?;
        InternalIvk::deserialize(&array)
            .map_err(From::from)
            .map(From::from)
    }
}

pub struct ZcashTransparentAddressAndIndex {
    pub transparent_address: Arc<ZcashTransparentAddress>,
    pub index: u32,
}

impl From<(TransparentAddress, u32)> for ZcashTransparentAddressAndIndex {
    fn from(value: (TransparentAddress, u32)) -> Self {
        ZcashTransparentAddressAndIndex {
            transparent_address: Arc::new(value.0.into()),
            index: value.1,
        }
    }
}
