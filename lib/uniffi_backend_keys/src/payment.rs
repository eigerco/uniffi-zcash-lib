use std::sync::Arc;

use zcash_primitives::transaction::components::amount::{self, Amount};
use zcash_primitives::memo::MemoBytes;
use zcash_client_backend::address::RecipientAddress;

#[derive(Clone)]
pub enum ZcashRecipientAddress {
    Shielded(Arc<crate::ZcashPaymentAddress>),
    Transparent(Arc<crate::ZcashTransparentAddress>),
    Unified(Arc<crate::ZcashUnifiedAddress>),
}

impl From<RecipientAddress> for ZcashRecipientAddress {
    fn from(addr: RecipientAddress) -> Self {
        match addr {
            RecipientAddress::Shielded(addr) => ZcashRecipientAddress::Shielded(Arc::new(addr.into())),
            RecipientAddress::Transparent(addr) => ZcashRecipientAddress::Transparent(Arc::new(addr.into())),
            RecipientAddress::Unified(addr) => ZcashRecipientAddress::Unified(Arc::new(addr.into())),
        }
    }
}

impl From<ZcashRecipientAddress> for RecipientAddress {
    fn from(addr: ZcashRecipientAddress) -> Self {
        match addr {
            ZcashRecipientAddress::Shielded(addr) => RecipientAddress::Shielded((&*addr).clone().into()),
            ZcashRecipientAddress::Transparent(addr) => RecipientAddress::Transparent((&*addr).clone().into()),
            ZcashRecipientAddress::Unified(addr) => RecipientAddress::Unified((&*addr).clone().into()),
        }
    }
}

impl ZcashRecipientAddress {
    pub fn shielded(addr: Arc<crate::ZcashPaymentAddress>) -> Self {
        ZcashRecipientAddress::Shielded(addr)
    }

    pub fn transparent(addr: Arc<crate::ZcashTransparentAddress>) -> Self {
        ZcashRecipientAddress::Transparent(addr)
    }

    pub fn unified(addr: Arc<crate::ZcashUnifiedAddress>) -> Self {
        ZcashRecipientAddress::Unified(addr)
    }

    pub fn encode(&self, params: crate::ZcashConsensusParameters) -> String {
        let addr: RecipientAddress = self.clone().into();
        addr.encode(&params)
    }
}

#[derive(Clone)]
pub struct ZcashPaymentParam {
    pub key: String,
    pub value: String,
}

#[derive(Clone)]
pub struct ZcashMemoBytes(MemoBytes);

impl ZcashMemoBytes {
    pub fn new(data: &[u8]) -> crate::ZcashResult<Self> {
        let memo = MemoBytes::from_bytes(data).map_err(|_| crate::ZcashError::Unknown)?;

        Ok(ZcashMemoBytes(memo))
    }

    pub fn data(&self) -> Vec<u8> {
        self.0.as_slice().to_owned()
    }
}

#[derive(Clone, Copy)]
pub struct ZcashAmount(Amount);

impl std::ops::Deref for ZcashAmount {
    type Target = Amount;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Amount> for ZcashAmount {
    fn from(value: Amount) -> Self {
        ZcashAmount(value)
    }
}

impl From<ZcashAmount> for Amount {
    fn from(value: ZcashAmount) -> Self {
        value.0
    }
}

impl ZcashAmount {
    pub fn new(value: i64) -> crate::ZcashResult<Self> {
        let amount = Amount::from_i64(value).map_err(|_| crate::ZcashError::ValueOutOfRange {
            val: value,
            from: -amount::MAX_MONEY,
            to: amount::MAX_MONEY,
        })?;

        Ok(ZcashAmount(amount))
    }

    pub fn zero() -> Self { ZcashAmount(Amount::zero()) }
}

#[derive(Clone)]
pub struct ZcashPayment {
    pub recipent_address: Arc<ZcashRecipientAddress>,
    pub amount: i64,
    pub memo: Option<Arc<ZcashMemoBytes>>,
    pub label: Option<String>,
    pub message: Option<String>,
    pub other_params: Vec<ZcashPaymentParam>,
}
