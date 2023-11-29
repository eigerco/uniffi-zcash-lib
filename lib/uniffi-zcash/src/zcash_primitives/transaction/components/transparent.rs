use std::sync::Arc;

use zcash_primitives::transaction::components::{
    transparent::{Authorized, Bundle, TxIn},
    OutPoint, TxOut,
};

use crate::{utils::cast_slice, ZcashAmount, ZcashResult, ZcashScript, ZcashTransparentAddress};

use derive_more::{From, Into};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, From, Into)]
pub struct ZcashOutPoint(OutPoint);

impl ZcashOutPoint {
    pub fn new(hash: &[u8], n: u32) -> ZcashResult<Self> {
        let casted_data = cast_slice(hash)?;
        Ok(OutPoint::new(casted_data, n).into())
    }
}

#[derive(From)]
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

    pub fn to_bytes(&self) -> ZcashResult<Vec<u8>> {
        let mut data = Vec::new();
        self.0.write(&mut data).unwrap();
        Ok(data)
    }

    /// Returns the address to which the TxOut was sent, if this is a valid P2SH or P2PKH output.
    pub fn recipient_address(&self) -> Option<Arc<ZcashTransparentAddress>> {
        self.0.recipient_address().map(From::from).map(Arc::new)
    }
}

impl From<&ZcashTxOut> for TxOut {
    fn from(value: &ZcashTxOut) -> Self {
        value.0.clone()
    }
}

impl From<&TxOut> for ZcashTxOut {
    fn from(inner: &TxOut) -> Self {
        ZcashTxOut(inner.clone())
    }
}

pub struct ZcashTxIn(TxIn<Authorized>);

impl ZcashTxIn {
    pub fn to_bytes(&self) -> ZcashResult<Vec<u8>> {
        let mut data = Vec::new();
        self.0.write(&mut data)?;
        Ok(data)
    }
}

impl From<&TxIn<Authorized>> for ZcashTxIn {
    fn from(inner: &TxIn<Authorized>) -> Self {
        ZcashTxIn(inner.clone())
    }
}

pub struct ZcashTransparentBundle(Bundle<Authorized>);

impl ZcashTransparentBundle {
    pub fn vout(&self) -> Vec<Arc<ZcashTxOut>> {
        self.0.vout.iter().map(|o| Arc::new(o.into())).collect()
    }

    pub fn vin(&self) -> Vec<Arc<ZcashTxIn>> {
        self.0.vin.iter().map(|i| Arc::new(i.into())).collect()
    }

    pub fn is_coinbase(&self) -> bool {
        self.0.is_coinbase()
    }
}

impl From<&Bundle<Authorized>> for ZcashTransparentBundle {
    fn from(inner: &Bundle<Authorized>) -> Self {
        ZcashTransparentBundle(inner.clone())
    }
}
