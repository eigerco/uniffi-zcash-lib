use std::sync::Arc;

use orchard::{primitives::redpallas::Signature, Action};
use reddsa::orchard::SpendAuth;

use crate::{
    ZcashExtractedNoteCommitment, ZcashOrchardNullifier, ZcashOrchardTransmittedNoteCiphertext,
    ZcashOrchardValueCommitment,
};

/// An action applied to the global ledger.
///
/// Externally, this both creates a note (adding a commitment to the global ledger),
/// and consumes some note created prior to this action (adding a nullifier to the
/// global ledger).
///
/// Internally, this may both consume a note and create a note, or it may do only one of
/// the two. TODO: Determine which is more efficient (circuit size vs bundle size).
pub struct ZcashOrchardAction(Action<Signature<SpendAuth>>);

impl ZcashOrchardAction {
    /// Returns the nullifier of the note being spent.
    pub fn nullifier(&self) -> Arc<ZcashOrchardNullifier> {
        Arc::new(self.0.nullifier().into())
    }

    /// Returns the commitment to the new note being created.
    pub fn cmx(&self) -> Arc<ZcashExtractedNoteCommitment> {
        Arc::new(self.0.cmx().into())
    }

    /// Returns the encrypted note ciphertext.
    pub fn encrypted_note(&self) -> ZcashOrchardTransmittedNoteCiphertext {
        self.0.encrypted_note().into()
    }

    /// Returns the commitment to the net value created or consumed by this action.
    pub fn cv_net(&self) -> Arc<ZcashOrchardValueCommitment> {
        Arc::new(self.0.cv_net().into())
    }
}

impl From<&Action<Signature<SpendAuth>>> for ZcashOrchardAction {
    fn from(inner: &Action<Signature<SpendAuth>>) -> Self {
        ZcashOrchardAction(inner.clone())
    }
}
