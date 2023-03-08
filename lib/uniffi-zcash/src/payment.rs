use std::sync::Arc;

use crate::{ZcashMemoBytes, ZcashRecipientAddress};

#[derive(Clone)]
pub struct ZcashPayment {
    pub recipent_address: Arc<ZcashRecipientAddress>,
    pub amount: i64,
    pub memo: Option<Arc<ZcashMemoBytes>>,
    pub label: Option<String>,
    pub message: Option<String>,
    pub other_params: Vec<ZcashPaymentParam>,
}

#[derive(Clone)]
pub struct ZcashPaymentParam {
    pub key: String,
    pub value: String,
}
