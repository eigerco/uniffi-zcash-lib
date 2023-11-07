use std::sync::Arc;

use zcash_client_backend::{address::UnifiedAddress, encoding::AddressCodec};

use crate::{
    ZcashConsensusParameters, ZcashError, ZcashOrchardAddress, ZcashPaymentAddress, ZcashResult,
    ZcashTransparentAddress,
};

/// A Unified Address.
#[derive(Debug, Clone)]
pub struct ZcashUnifiedAddress(UnifiedAddress);

impl From<UnifiedAddress> for ZcashUnifiedAddress {
    fn from(addr: UnifiedAddress) -> Self {
        ZcashUnifiedAddress(addr)
    }
}

impl From<ZcashUnifiedAddress> for UnifiedAddress {
    fn from(addr: ZcashUnifiedAddress) -> Self {
        addr.0
    }
}

impl ZcashUnifiedAddress {
    /// Constructs a Unified Address from a given set of receivers.
    ///
    /// Returns `None` if the receivers would produce an invalid Unified Address (namely,
    /// if no shielded receiver is provided).
    pub fn new(
        orchard: Option<Arc<ZcashOrchardAddress>>,
        sapling: Option<Arc<ZcashPaymentAddress>>,
        transparent: Option<Arc<ZcashTransparentAddress>>,
    ) -> ZcashResult<Self> {
        let orchard = orchard.map(|o| o.0);
        let sapling = sapling.map(|s| s.as_ref().into());
        let transparent = transparent.map(|t| t.as_ref().into());

        UnifiedAddress::from_receivers(orchard, sapling, transparent)
            .map(ZcashUnifiedAddress)
            .ok_or(ZcashError::Unknown)
    }

    /// Returns the Orchard receiver within this Unified Address, if any.
    pub fn orchard(&self) -> Option<Arc<ZcashOrchardAddress>> {
        self.0.orchard().cloned().map(Into::into).map(Arc::new)
    }

    /// Returns the Sapling receiver within this Unified Address, if any.
    pub fn sapling(&self) -> Option<Arc<ZcashPaymentAddress>> {
        self.0.sapling().cloned().map(Into::into).map(Arc::new)
    }

    /// Returns the transparent receiver within this Unified Address, if any.
    pub fn transparent(&self) -> Option<Arc<ZcashTransparentAddress>> {
        self.0.transparent().cloned().map(Into::into).map(Arc::new)
    }

    pub fn decode(params: ZcashConsensusParameters, addr: &str) -> ZcashResult<Self> {
        Ok(AddressCodec::decode(&params, addr).map(ZcashUnifiedAddress)?)
    }

    /// Returns the string encoding of this `UnifiedAddress` for the given network.
    pub fn encode(&self, params: ZcashConsensusParameters) -> String {
        self.0.encode(&params)
    }
}
