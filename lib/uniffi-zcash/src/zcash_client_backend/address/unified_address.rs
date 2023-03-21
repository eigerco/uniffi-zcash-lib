use std::sync::Arc;

use zcash_client_backend::{address::UnifiedAddress, encoding::AddressCodec};

use crate::{
    ZcashConsensusParameters, ZcashError, ZcashOrchardAddress, ZcashPaymentAddress, ZcashResult,
    ZcashTransparentAddress,
};

#[derive(Clone)]
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
    pub fn new(
        orchard: Option<Arc<ZcashOrchardAddress>>,
        sapling: Option<Arc<ZcashPaymentAddress>>,
        transparent: Option<Arc<ZcashTransparentAddress>>,
    ) -> ZcashResult<Self> {
        let orchard = orchard.map(|o| o.0);
        let sapling = sapling.map(|s| s.as_ref().into());
        let transparent = transparent.map(|t| (*t).into());

        UnifiedAddress::from_receivers(orchard, sapling, transparent)
            .map(ZcashUnifiedAddress)
            .ok_or(ZcashError::Unknown)
    }

    pub fn parse(params: ZcashConsensusParameters, addr: &str) -> ZcashResult<Self> {
        Ok(AddressCodec::decode(&params, addr).map(ZcashUnifiedAddress)?)
    }

    pub fn to_string(&self, params: ZcashConsensusParameters) -> String {
        self.0.encode(&params)
    }

    pub fn orchard(&self) -> Option<Arc<ZcashOrchardAddress>> {
        self.0.orchard().cloned().map(Into::into).map(Arc::new)
    }

    pub fn sapling(&self) -> Option<Arc<ZcashPaymentAddress>> {
        self.0.sapling().cloned().map(Into::into).map(Arc::new)
    }

    pub fn transparent(&self) -> Option<Arc<ZcashTransparentAddress>> {
        self.0.transparent().cloned().map(Into::into).map(Arc::new)
    }
}
