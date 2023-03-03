use std::sync::Arc;

use crate::ZcashConsensusParameters;
use crate::ZcashResult;

use zcash_client_backend::address::UnifiedAddress;
use zcash_client_backend::encoding::AddressCodec;

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
        orchard: Option<Arc<crate::ZcashOrchardAddress>>,
        sapling: Option<Arc<crate::ZcashPaymentAddress>>,
        transparent: Option<Arc<crate::ZcashTransparentAddress>>,
    ) -> ZcashResult<Self> {
        let orchard = orchard.map(|o| o.inner.clone());
        let sapling = sapling.map(|s| (&*s).clone().into());
        let transparent = transparent.map(|t| (&*t).clone().into());

        UnifiedAddress::from_receivers(orchard, sapling, transparent)
            .map(ZcashUnifiedAddress)
            .ok_or(crate::ZcashError::Unknown)
    }

    pub fn parse(params: ZcashConsensusParameters, addr: &str) -> ZcashResult<Self> {
        Ok(AddressCodec::decode(&params, addr).map(ZcashUnifiedAddress)?)
    }

    pub fn to_string(&self, params: ZcashConsensusParameters) -> String {
        self.0.encode(&params)
    }

    pub fn orchard(&self) -> Option<Arc<crate::ZcashOrchardAddress>> {
        self.0.orchard().cloned().map(Into::into).map(Arc::new)
    }

    pub fn sapling(&self) -> Option<Arc<crate::ZcashPaymentAddress>> {
        self.0.sapling().cloned().map(Into::into).map(Arc::new)
    }

    pub fn transparent(&self) -> Option<Arc<crate::ZcashTransparentAddress>> {
        self.0.transparent().cloned().map(Into::into).map(Arc::new)
    }
}
