use std::sync::Arc;

use zcash_primitives::transaction::components::{OutPoint, TxOut};

use crate::{utils::cast_slice, ZcashAmount, ZcashResult, ZcashScript, ZcashTransparentAddress};

pub struct ZcashOutPoint(OutPoint);

impl ZcashOutPoint {
    pub fn new(hash: &[u8], n: u32) -> ZcashResult<Self> {
        let casted_data = cast_slice(hash)?;
        Ok(OutPoint::new(casted_data, n).into())
    }
}

impl From<OutPoint> for ZcashOutPoint {
    fn from(inner: OutPoint) -> Self {
        ZcashOutPoint(inner)
    }
}

impl From<ZcashOutPoint> for OutPoint {
    fn from(value: ZcashOutPoint) -> Self {
        value.0
    }
}

impl From<&ZcashOutPoint> for OutPoint {
    fn from(value: &ZcashOutPoint) -> Self {
        value.0.clone()
    }
}

pub struct ZcashTxOut(TxOut);

impl ZcashTxOut {
    pub fn new(value: Arc<ZcashAmount>, script_pubkey: Arc<ZcashScript>) -> Self {
        Self(TxOut {
            value: value.as_ref().into(),
            script_pubkey: script_pubkey.as_ref().into(),
        })
    }

    pub fn value(&self) -> Arc<ZcashAmount> {
        Arc::new(self.0.value.into())
    }

    pub fn script_pubkey(&self) -> Arc<ZcashScript> {
        Arc::new(self.0.script_pubkey.clone().into())
    }

    /// Returns the address to which the TxOut was sent, if this is a valid P2SH or P2PKH output.
    pub fn recipient_address(&self) -> Option<Arc<ZcashTransparentAddress>> {
        self.0.recipient_address().map(From::from).map(Arc::new)
    }
}

impl From<ZcashTxOut> for TxOut {
    fn from(value: ZcashTxOut) -> Self {
        value.0
    }
}

impl From<&ZcashTxOut> for TxOut {
    fn from(value: &ZcashTxOut) -> Self {
        value.0.clone()
    }
}
