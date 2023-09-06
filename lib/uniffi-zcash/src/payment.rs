use std::sync::Arc;

use crate::{ZcashAmount, ZcashMemoBytes, ZcashRecipientAddress};
use zcash_client_backend::zip321::Payment;

#[derive(Clone)]
pub struct ZcashPayment {
    pub recipient_address: Arc<ZcashRecipientAddress>,
    pub amount: Arc<ZcashAmount>,
    pub memo: Option<Arc<ZcashMemoBytes>>,
    pub label: Option<String>,
    pub message: Option<String>,
    pub other_params: Vec<ZcashPaymentParam>,
}

impl From<&Payment> for ZcashPayment {
    fn from(inner: &Payment) -> Self {
        ZcashPayment {
            recipient_address: Arc::new(inner.recipient_address.to_owned().into()),
            amount: Arc::new(inner.amount.into()),
            memo: inner.memo.to_owned().map(|x| Arc::new(x.into())),
            label: inner.label.to_owned(),
            message: inner.message.to_owned(),
            other_params: inner.other_params.iter().cloned().map(From::from).collect(),
        }
    }
}

impl From<ZcashPayment> for Payment {
    fn from(inner: ZcashPayment) -> Self {
        Payment {
            recipient_address: Arc::try_unwrap(inner.recipient_address).unwrap().into(),
            amount: Arc::try_unwrap(inner.amount).unwrap().into(),
            memo: inner.memo.map(|x| Arc::try_unwrap(x).unwrap().into()),
            label: inner.label,
            message: inner.message,
            other_params: inner.other_params.into_iter().map(From::from).collect(),
        }
    }
}

#[derive(Clone)]
pub struct ZcashPaymentParam {
    pub key: String,
    pub value: String,
}

impl From<ZcashPaymentParam> for (String, String) {
    fn from(inner: ZcashPaymentParam) -> Self {
        (inner.key, inner.value)
    }
}

impl From<(String, String)> for ZcashPaymentParam {
    fn from(inner: (String, String)) -> Self {
        ZcashPaymentParam {
            key: inner.0,
            value: inner.1,
        }
    }
}
