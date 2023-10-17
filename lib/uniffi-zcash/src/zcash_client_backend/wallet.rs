use std::sync::Arc;
use zcash_client_backend::wallet::{
    OvkPolicy, WalletSaplingOutput, WalletSaplingSpend, WalletTransparentOutput, WalletTx,
};
use zcash_primitives::keys::OutgoingViewingKey;
use zcash_primitives::sapling;

use crate::{
    ZcashAmount, ZcashBlockHeight, ZcashOutPoint, ZcashResult, ZcashTransparentAddress, ZcashTxId,
    ZcashTxOut,
};

pub struct ZcashWalletSaplingSpend(WalletSaplingSpend);

impl Clone for ZcashWalletSaplingSpend {
    fn clone(&self) -> Self {
        Self(WalletSaplingSpend::from_parts(
            self.0.index(),
            *self.0.nf(),
            self.0.account(),
        ))
    }
}

impl From<WalletSaplingSpend> for ZcashWalletSaplingSpend {
    fn from(inner: WalletSaplingSpend) -> Self {
        Self(inner)
    }
}

impl From<ZcashWalletSaplingSpend> for WalletSaplingSpend {
    fn from(outer: ZcashWalletSaplingSpend) -> Self {
        outer.0
    }
}

pub struct ZcashWalletSaplingOutput(WalletSaplingOutput<sapling::Nullifier>);

impl Clone for ZcashWalletSaplingOutput {
    fn clone(&self) -> Self {
        Self(WalletSaplingOutput::from_parts(
            self.0.index(),
            *self.0.cmu(),
            self.0.ephemeral_key().clone(),
            self.0.account(),
            self.0.note().clone(),
            self.0.is_change(),
            self.0.note_commitment_tree_position(),
            *self.0.nf(),
        ))
    }
}

impl From<WalletSaplingOutput<sapling::Nullifier>> for ZcashWalletSaplingOutput {
    fn from(inner: WalletSaplingOutput<sapling::Nullifier>) -> Self {
        Self(inner)
    }
}

impl From<ZcashWalletSaplingOutput> for WalletSaplingOutput<sapling::Nullifier> {
    fn from(outer: ZcashWalletSaplingOutput) -> Self {
        outer.0
    }
}

/// A subset of a [`ZcashTransaction`] relevant to wallets and light clients.
pub struct ZcashWalletTx(WalletTx<sapling::Nullifier>);

impl ZcashWalletTx {
    pub fn new(
        txid: Arc<ZcashTxId>,
        index: u32,
        sapling_spends: Vec<Arc<ZcashWalletSaplingSpend>>,
        sapling_outputs: Vec<Arc<ZcashWalletSaplingOutput>>,
    ) -> Self {
        Self(WalletTx {
            txid: (*txid).into(),
            index: index.try_into().unwrap(),
            sapling_spends: sapling_spends
                .into_iter()
                .map(|x| (*x).clone().into())
                .collect(),
            sapling_outputs: sapling_outputs
                .into_iter()
                .map(|x| (*x).clone().into())
                .collect(),
        })
    }
}

impl Clone for ZcashWalletTx {
    fn clone(&self) -> Self {
        let sapling_outputs = self
            .0
            .sapling_outputs
            .iter()
            .map(|x| {
                WalletSaplingOutput::from_parts(
                    x.index(),
                    *x.cmu(),
                    x.ephemeral_key().clone(),
                    x.account(),
                    x.note().clone(),
                    x.is_change(),
                    x.note_commitment_tree_position(),
                    *x.nf(),
                )
            })
            .collect::<Vec<WalletSaplingOutput<sapling::Nullifier>>>();

        let sapling_spends = self
            .0
            .sapling_spends
            .iter()
            .map(|x| WalletSaplingSpend::from_parts(x.index(), *x.nf(), x.account()))
            .collect::<Vec<WalletSaplingSpend>>();

        Self(WalletTx {
            txid: self.0.txid,
            index: self.0.index,
            sapling_spends,
            sapling_outputs,
        })
    }
}

impl From<WalletTx<sapling::Nullifier>> for ZcashWalletTx {
    fn from(inner: WalletTx<sapling::Nullifier>) -> Self {
        Self(inner)
    }
}

impl From<ZcashWalletTx> for WalletTx<sapling::Nullifier> {
    fn from(outer: ZcashWalletTx) -> Self {
        outer.0
    }
}

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
        Self(inner)
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
