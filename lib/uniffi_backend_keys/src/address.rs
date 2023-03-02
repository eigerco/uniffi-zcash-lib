use std::sync::Arc;

use crate::ZcashConsensusParameters;
use crate::ZcashResult;

use zcash_client_backend::address::UnifiedAddress;
use zcash_client_backend::encoding::AddressCodec;

pub struct ZcashUnifiedAddress(UnifiedAddress);

impl ZcashUnifiedAddress {
    pub fn new(
        orchard: Option<Arc<crate::ZcashOrchardAddress>>,
        sapling: Option<Arc<crate::ZcashPaymentAddress>>,
        // _transparent: Option<()>,
    ) -> ZcashResult<Self> {
        let orchard = orchard.map(|o| o.inner.clone());
        let sapling = sapling.map(|s| s.inner.clone());

        UnifiedAddress::from_receivers(orchard, sapling, None)
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
}
