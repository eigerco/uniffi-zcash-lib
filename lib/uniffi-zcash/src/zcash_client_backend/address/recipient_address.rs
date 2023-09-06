use std::sync::Arc;

use zcash_client_backend::address::RecipientAddress;

use crate::{
    ZcashConsensusParameters, ZcashError, ZcashPaymentAddress, ZcashResult,
    ZcashTransparentAddress, ZcashUnifiedAddress,
};

/// An address that funds can be sent to.
#[derive(Debug, Clone)]
pub enum ZcashRecipientAddress {
    Shielded(Arc<ZcashPaymentAddress>),
    Transparent(Arc<ZcashTransparentAddress>),
    Unified(Arc<ZcashUnifiedAddress>),
}

impl From<RecipientAddress> for ZcashRecipientAddress {
    fn from(addr: RecipientAddress) -> Self {
        match addr {
            RecipientAddress::Shielded(addr) => {
                ZcashRecipientAddress::Shielded(Arc::new(addr.into()))
            }
            RecipientAddress::Transparent(addr) => {
                ZcashRecipientAddress::Transparent(Arc::new(addr.into()))
            }
            RecipientAddress::Unified(addr) => {
                ZcashRecipientAddress::Unified(Arc::new(addr.into()))
            }
        }
    }
}

impl From<ZcashRecipientAddress> for RecipientAddress {
    fn from(addr: ZcashRecipientAddress) -> Self {
        match addr {
            ZcashRecipientAddress::Shielded(addr) => {
                RecipientAddress::Shielded(addr.as_ref().into())
            }
            ZcashRecipientAddress::Transparent(addr) => {
                RecipientAddress::Transparent(addr.as_ref().into())
            }
            ZcashRecipientAddress::Unified(addr) => {
                RecipientAddress::Unified((*addr).clone().into())
            }
        }
    }
}

impl ZcashRecipientAddress {
    pub fn shielded(addr: Arc<ZcashPaymentAddress>) -> Self {
        ZcashRecipientAddress::Shielded(addr)
    }

    pub fn transparent(addr: Arc<ZcashTransparentAddress>) -> Self {
        ZcashRecipientAddress::Transparent(addr)
    }

    pub fn unified(addr: Arc<ZcashUnifiedAddress>) -> Self {
        ZcashRecipientAddress::Unified(addr)
    }

    pub fn decode(params: ZcashConsensusParameters, address: &str) -> ZcashResult<Self> {
        RecipientAddress::decode(&params, address)
            .map(From::from)
            .ok_or::<ZcashError>("unable to parse address".into())
    }

    pub fn encode(&self, params: crate::ZcashConsensusParameters) -> String {
        let addr: RecipientAddress = self.clone().into();
        addr.encode(&params)
    }
}
