use std::sync::Arc;

use zcash_primitives::transaction::components::{
    transparent::{Authorized, Bundle, TxIn}, OutPoint, TxOut,
};

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

    /// Returns `true` if this bundle matches the definition of a coinbase transaction.
    ///
    /// Note that this is defined purely in terms of the transparent transaction part. The
    /// consensus rules enforce additional rules on the shielded parts (namely, that they
    /// don't have any inputs) of transactions with a transparent part that matches this
    /// definition.
    pub fn is_coinbase(&self) -> bool {
        self.0.is_coinbase()
    }

    // /// The amount of value added to or removed from the transparent pool by the action of this
    // /// bundle. A positive value represents that the containing transaction has funds being
    // /// transferred out of the transparent pool into shielded pools or to fees; a negative value
    // /// means that the containing transaction has funds being transferred into the transparent pool
    // /// from the shielded pools.
    // pub fn value_balance(&self) -> ZcashResult<Arc<ZcashAmount>> {
    //     match self
    //         .0
    //         .value_balance::<BalanceError, _>(|_| Ok(Amount::zero()))
    //     {
    //         Ok(amount) => Ok(Arc::new(amount.into())),
    //         Err(err) => Err(err.into()),
    //     }
    // }
}

impl From<&Bundle<Authorized>> for ZcashTransparentBundle {
    fn from(inner: &Bundle<Authorized>) -> Self {
        ZcashTransparentBundle(inner.clone())
    }
}
