use std::sync::Arc;

use orchard::{
    bundle::{Authorized, Flags},
    keys::{IncomingViewingKey, OutgoingViewingKey},
    Address, Bundle, Note,
};
use zcash_primitives::transaction::components::Amount;

use crate::{
    ZcashAmount, ZcashAnchor, ZcashError, ZcashOrchardAction, ZcashOrchardAddress,
    ZcashOrchardIncomingViewingKey, ZcashOrchardNote, ZcashOrchardOutgoingViewingKey, ZcashResult,
    ZcashVerifyingKey,
};

/// A bundle of actions to be applied to the ledger.
pub struct ZcashOrchardBundle(Bundle<Authorized, Amount>);

impl ZcashOrchardBundle {
    /// The list of actions that make up this bundle.
    pub fn actions(&self) -> Vec<Arc<ZcashOrchardAction>> {
        self.0
            .actions()
            .iter()
            .map(|a| a.into())
            .map(Arc::new)
            .collect()
    }

    /// Returns the Orchard-specific transaction-level flags for this bundle.
    pub fn flags(&self) -> Arc<ZcashOrchardFlags> {
        Arc::new(self.0.flags().into())
    }

    /// Returns the net value moved into or out of the Orchard shielded pool.
    ///
    /// This is the sum of Orchard spends minus the sum Orchard outputs.
    pub fn value_balance(&self) -> Arc<ZcashAmount> {
        Arc::new(self.0.value_balance().into())
    }

    /// Returns the root of the Orchard commitment tree that this bundle commits to.
    pub fn anchor(&self) -> Arc<ZcashAnchor> {
        Arc::new((*self.0.anchor()).into())
    }

    /// Verifies the proof for this bundle.
    pub fn verify_proof(&self, key: Arc<ZcashVerifyingKey>) -> ZcashResult<()> {
        self.0
            .verify_proof(&key.0)
            .or(Err("Error verifying proof".into()))
    }

    /// Performs trial decryption of the action at `action_idx` in the bundle with the
    /// specified incoming viewing key, and returns the decrypted note plaintext
    /// contents if successful.
    pub fn decrypt_output_with_key(
        &self,
        action_idx: u64,
        ivk: Arc<ZcashOrchardIncomingViewingKey>,
    ) -> ZcashResult<ZcashOrchardDecryptOutput> {
        match self
            .0
            .decrypt_output_with_key(action_idx.try_into()?, &ivk.0)
        {
            Some(result) => Ok(result.into()),
            None => Err("Cannot decrypt bundle".into()),
        }
    }

    /// Performs trial decryption of each action in the bundle with each of the
    /// specified incoming viewing keys, and returns a vector of each decrypted
    /// note plaintext contents along with the index of the action from which it
    /// was derived.
    pub fn decrypt_output_with_keys(
        &self,
        ivks: Vec<Arc<ZcashOrchardIncomingViewingKey>>,
    ) -> Vec<ZcashOrchardDecryptOutputForIncomingKeys> {
        let keys = ivks
            .into_iter()
            .map(|f| f.as_ref().into())
            .collect::<Vec<IncomingViewingKey>>();
        self.0
            .decrypt_outputs_with_keys(&keys)
            .into_iter()
            .map(|e| e.try_into().unwrap())
            .collect()
    }

    /// Attempts to decrypt the action at the specified index with the specified
    /// outgoing viewing key, and returns the decrypted note plaintext contents
    /// if successful.
    pub fn recover_output_with_ovk(
        &self,
        action_idx: u64,
        key: Arc<ZcashOrchardOutgoingViewingKey>,
    ) -> ZcashResult<ZcashOrchardDecryptOutput> {
        match self
            .0
            .recover_output_with_ovk(action_idx.try_into()?, &key.0)
        {
            Some(result) => Ok(result.into()),
            None => Err("Cannot recover output".into()),
        }
    }

    /// Performs trial decryption of each action in the bundle with each of the
    /// specified outgoing viewing keys, and returns a vector of each decrypted
    /// note plaintext contents along with the index of the action from which it
    /// was derived.
    pub fn recover_outputs_with_ovks(
        &self,
        ovks: Vec<Arc<ZcashOrchardOutgoingViewingKey>>,
    ) -> Vec<ZcashOrchardDecryptOutputForOutgoingKeys> {
        let keys = ovks
            .into_iter()
            .map(|key| key.as_ref().into())
            .collect::<Vec<OutgoingViewingKey>>();
        self.0
            .recover_outputs_with_ovks(&keys)
            .into_iter()
            .map(|e| e.try_into().unwrap())
            .collect()
    }
}

impl From<&Bundle<Authorized, Amount>> for ZcashOrchardBundle {
    fn from(inner: &Bundle<Authorized, Amount>) -> Self {
        ZcashOrchardBundle(inner.clone())
    }
}

pub struct ZcashOrchardDecryptOutput {
    pub note: Arc<ZcashOrchardNote>,
    pub address: Arc<ZcashOrchardAddress>,
    pub data: Vec<u8>,
}

impl From<(Note, Address, [u8; 512])> for ZcashOrchardDecryptOutput {
    fn from((note, address, data): (Note, Address, [u8; 512])) -> Self {
        Self {
            note: Arc::new(note.into()),
            address: Arc::new(address.into()),
            data: data.to_vec(),
        }
    }
}

pub struct ZcashOrchardDecryptOutputForIncomingKeys {
    pub idx: u64,
    pub key: Arc<ZcashOrchardIncomingViewingKey>,
    pub note: Arc<ZcashOrchardNote>,
    pub address: Arc<ZcashOrchardAddress>,
    pub data: Vec<u8>,
}

impl TryFrom<(usize, IncomingViewingKey, Note, Address, [u8; 512])>
    for ZcashOrchardDecryptOutputForIncomingKeys
{
    type Error = ZcashError;
    fn try_from(
        value: (usize, IncomingViewingKey, Note, Address, [u8; 512]),
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            idx: value.0.try_into()?,
            key: Arc::new(value.1.into()),
            note: Arc::new(value.2.into()),
            address: Arc::new(value.3.into()),
            data: value.4.to_vec(),
        })
    }
}

pub struct ZcashOrchardDecryptOutputForOutgoingKeys {
    pub idx: u64,
    pub key: Arc<ZcashOrchardOutgoingViewingKey>,
    pub note: Arc<ZcashOrchardNote>,
    pub address: Arc<ZcashOrchardAddress>,
    pub data: Vec<u8>,
}

impl TryFrom<(usize, OutgoingViewingKey, Note, Address, [u8; 512])>
    for ZcashOrchardDecryptOutputForOutgoingKeys
{
    type Error = ZcashError;
    fn try_from(
        value: (usize, OutgoingViewingKey, Note, Address, [u8; 512]),
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            idx: value.0.try_into()?,
            key: Arc::new(value.1.into()),
            note: Arc::new(value.2.into()),
            address: Arc::new(value.3.into()),
            data: value.4.to_vec(),
        })
    }
}

/// Orchard-specific flags.
pub struct ZcashOrchardFlags(Flags);

impl ZcashOrchardFlags {
    /// Construct a set of flags from its constituent parts
    pub fn from_parts(spends_enabled: bool, outputs_enabled: bool) -> Self {
        Flags::from_parts(spends_enabled, outputs_enabled).into()
    }

    /// Flag denoting whether Orchard spends are enabled in the transaction.
    ///
    /// If `false`, spent notes within [`Action`]s in the transaction's [`Bundle`] are
    /// guaranteed to be dummy notes. If `true`, the spent notes may be either real or
    /// dummy notes.
    pub fn spends_enabled(&self) -> bool {
        self.0.spends_enabled()
    }

    /// Flag denoting whether Orchard outputs are enabled in the transaction.
    ///
    /// If `false`, created notes within [`Action`]s in the transaction's [`Bundle`] are
    /// guaranteed to be dummy notes. If `true`, the created notes may be either real or
    /// dummy notes.
    pub fn outputs_enabled(&self) -> bool {
        self.0.outputs_enabled()
    }

    /// Serialize flags to a byte as defined in [Zcash Protocol Spec § 7.1: Transaction
    /// Encoding And Consensus][txencoding].
    ///
    /// [txencoding]: https://zips.z.cash/protocol/protocol.pdf#txnencoding
    pub fn to_byte(&self) -> u8 {
        self.0.to_byte()
    }

    /// Parses flags from a single byte as defined in [Zcash Protocol Spec § 7.1:
    /// Transaction Encoding And Consensus][txencoding].
    ///
    /// Returns `None` if unexpected bits are set in the flag byte.
    ///
    /// [txencoding]: https://zips.z.cash/protocol/protocol.pdf#txnencoding
    pub fn from_byte(v: u8) -> ZcashResult<Self> {
        match Flags::from_byte(v) {
            Some(flags) => Ok(flags.into()),
            None => Err("Error parsing flags bits".into()),
        }
    }
}

impl From<&ZcashOrchardFlags> for Flags {
    fn from(value: &ZcashOrchardFlags) -> Self {
        value.0
    }
}

impl From<Flags> for ZcashOrchardFlags {
    fn from(inner: Flags) -> Self {
        ZcashOrchardFlags(inner)
    }
}

impl From<&Flags> for ZcashOrchardFlags {
    fn from(inner: &Flags) -> Self {
        ZcashOrchardFlags(*inner)
    }
}
