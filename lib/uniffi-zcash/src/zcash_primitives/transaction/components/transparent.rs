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

pub struct ZcashTxOut(TxOut);

impl ZcashTxOut {
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
