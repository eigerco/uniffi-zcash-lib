use std::sync::Arc;

use zcash_primitives::legacy::keys::{ExternalIvk, IncomingViewingKey, InternalIvk};

use crate::{ZcashError, ZcashResult, ZcashTransparentAddress};

/// A type representing an incoming viewing key at the BIP-44 "external"
/// path `m/44'/<coin_type>'/<account>'/0`. This allows derivation
/// of child addresses that may be provided to external parties.
pub struct ZcashExternalIvk(ExternalIvk);

impl ZcashExternalIvk {
    /// Derives a transparent address at the provided child index.
    pub fn derive_address(&self, child_index: u32) -> ZcashResult<Arc<ZcashTransparentAddress>> {
        let address = self
            .0
            .derive_address(child_index)
            .map_err(ZcashError::from)?;
        Ok(Arc::new(address.into()))
    }
}

impl From<ExternalIvk> for ZcashExternalIvk {
    fn from(inner: ExternalIvk) -> Self {
        ZcashExternalIvk(inner)
    }
}

// InternalIVK (all methods private)
pub struct ZcashInternalIvk(InternalIvk);

impl ZcashInternalIvk {}

impl From<InternalIvk> for ZcashInternalIvk {
    fn from(inner: InternalIvk) -> Self {
        ZcashInternalIvk(inner)
    }
}
