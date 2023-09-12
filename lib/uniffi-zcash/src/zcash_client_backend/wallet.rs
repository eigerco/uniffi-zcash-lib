use std::sync::Arc;
use zcash_client_backend::wallet::{OvkPolicy, WalletTransparentOutput};
use zcash_primitives::keys::OutgoingViewingKey;
use zcash_primitives::transaction::components::transparent::{self, OutPoint, TxOut};

use crate::{
    ZcashAmount, ZcashBlockHeight, ZcashOutPoint, ZcashOutgoingViewingKey, ZcashResult,
    ZcashTransparentAddress, ZcashTxOut,
};

#[derive(Debug, Clone)]
pub struct ZcashWalletTransparentOutput(pub WalletTransparentOutput);

impl ZcashWalletTransparentOutput {
    pub fn from_parts(
        outpoint: Arc<ZcashOutPoint>,
        txout: Arc<ZcashTxOut>,
        height: Arc<ZcashBlockHeight>,
    ) -> ZcashResult<Self> {
        let opt: Option<WalletTransparentOutput> = WalletTransparentOutput::from_parts(
            outpoint.as_ref().into(),
            txout.as_ref().into(),
            height.as_ref().into(),
        );

        match opt {
            Some(out) => Ok(out.into()),
            None => Err("Cannot do".into()),
        }
    }

    pub fn outpoint(&self) -> Arc<ZcashOutPoint> {
        Arc::new(self.0.outpoint().clone().into())
    }

    pub fn txout(&self) -> Arc<ZcashTxOut> {
        Arc::new(self.0.txout().into())
    }

    pub fn height(&self) -> Arc<ZcashBlockHeight> {
        Arc::new(self.0.height().into())
    }

    pub fn recipient_address(&self) -> Arc<ZcashTransparentAddress> {
        Arc::new((*self.0.recipient_address()).into())
    }

    pub fn value(&self) -> Arc<ZcashAmount> {
        Arc::new(self.0.txout().value.into())
    }
}

impl From<WalletTransparentOutput> for ZcashWalletTransparentOutput {
    fn from(inner: WalletTransparentOutput) -> Self {
        ZcashWalletTransparentOutput(inner)
    }
}

// impl From<&ZcashWalletTransparentOutput> for &WalletTransparentOutput {
//     fn from(output: &ZcashWalletTransparentOutput) -> Self {
//         &output.0
//     }
// }

pub enum ZcashOvkPolicy {
    /// Use the outgoing viewing key from the sender's [`ExtendedFullViewingKey`].
    ///
    /// Transaction outputs will be decryptable by the sender, in addition to the
    /// recipients.
    ///
    /// [`ExtendedFullViewingKey`]: zcash_primitives::zip32::ExtendedFullViewingKey
    Sender,

    /// Use a custom outgoing viewing key. This might for instance be derived from a
    /// separate seed than the wallet's spending keys.
    ///
    /// Transaction outputs will be decryptable by the recipients, and whoever controls
    /// the provided outgoing viewing key.
    Custom { bytes: Vec<u8> },

    /// Use no outgoing viewing key. Transaction outputs will be decryptable by their
    /// recipients, but not by the sender.
    Discard,
}

impl From<ZcashOvkPolicy> for OvkPolicy {
    fn from(value: ZcashOvkPolicy) -> Self {
        match value {
            ZcashOvkPolicy::Sender => OvkPolicy::Sender,
            ZcashOvkPolicy::Custom { bytes } => {
                OvkPolicy::Custom(OutgoingViewingKey(bytes.try_into().unwrap()))
            }
            ZcashOvkPolicy::Discard => OvkPolicy::Discard,
        }
    }
}
