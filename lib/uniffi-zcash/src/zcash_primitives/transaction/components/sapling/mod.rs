mod builder;
pub use self::builder::*;

use std::sync::Arc;

use zcash_primitives::transaction::components::{
    sapling::{Authorized, Bundle, GrothProofBytes, SpendDescription},
    OutputDescription,
};

use crate::{
    ZcashAmount, ZcashSaplingExtractedNoteCommitment, ZcashSaplingNullifier, ZcashSaplingPublicKey,
    ZcashSaplingValueCommitment,
};

pub struct ZcashSaplingBundle(Bundle<Authorized>);

impl ZcashSaplingBundle {
    /// Returns the list of spends in this bundle.
    pub fn shielded_spends(&self) -> Vec<Arc<ZcashSaplingSpendDescription>> {
        self.0
            .shielded_spends()
            .iter()
            .map(|s| Arc::new(s.into()))
            .collect()
    }

    /// Returns the list of outputs in this bundle.
    pub fn shielded_outputs(&self) -> Vec<Arc<ZcashSaplingOutputDescription>> {
        self.0
            .shielded_outputs()
            .iter()
            .map(|o| Arc::new(o.into()))
            .collect()
    }

    /// Returns the net value moved into or out of the Sapling shielded pool.
    ///
    /// This is the sum of Sapling spends minus the sum of Sapling outputs.
    pub fn value_balance(&self) -> Arc<ZcashAmount> {
        Arc::new(self.0.value_balance().into())
    }
}

impl From<&Bundle<Authorized>> for ZcashSaplingBundle {
    fn from(inner: &Bundle<Authorized>) -> Self {
        ZcashSaplingBundle(inner.clone())
    }
}

pub struct ZcashSaplingSpendDescription(SpendDescription<Authorized>);

impl ZcashSaplingSpendDescription {
    /// Returns the commitment to the value consumed by this spend.
    pub fn cv(&self) -> Arc<ZcashSaplingValueCommitment> {
        Arc::new(self.0.cv().into())
    }

    /// Returns the root of the Sapling commitment tree that this spend commits to.
    pub fn anchor(&self) -> Vec<u8> {
        self.0.anchor().to_bytes().to_vec()
    }

    /// Returns the nullifier of the note being spent.
    pub fn nullifier(&self) -> Arc<ZcashSaplingNullifier> {
        Arc::new(self.0.nullifier().into())
    }

    /// Returns the randomized verification key for the note being spent.
    pub fn rk(&self) -> Arc<ZcashSaplingPublicKey> {
        Arc::new(self.0.rk().into())
    }
}

impl From<&SpendDescription<Authorized>> for ZcashSaplingSpendDescription {
    fn from(inner: &SpendDescription<Authorized>) -> Self {
        ZcashSaplingSpendDescription(inner.clone())
    }
}

pub struct ZcashSaplingOutputDescription(OutputDescription<GrothProofBytes>); // Looks like GrothProofBytes corresponds to the authorized state proof.

impl ZcashSaplingOutputDescription {
    pub fn cv(&self) -> Arc<ZcashSaplingValueCommitment> {
        Arc::new(self.0.cv().into())
    }

    pub fn cmu(&self) -> Arc<ZcashSaplingExtractedNoteCommitment> {
        Arc::new(self.0.cmu().into())
    }
}

impl From<&OutputDescription<GrothProofBytes>> for ZcashSaplingOutputDescription {
    fn from(inner: &OutputDescription<GrothProofBytes>) -> Self {
        ZcashSaplingOutputDescription(inner.clone())
    }
}
