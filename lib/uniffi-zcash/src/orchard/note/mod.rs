mod commitment;
pub use self::commitment::*;

use std::sync::Arc;

use orchard::{
    note::{Nullifier, RandomSeed},
    Note,
};

use crate::{utils::cast_slice, ZcashOrchardAddress, ZcashOrchardNoteValue, ZcashResult};

/// A discrete amount of funds received by an address.
pub struct ZcashOrchardNote(Note);

impl ZcashOrchardNote {
    /// Creates a `Note` from its component parts.
    ///
    /// This low-level constructor enforces that the provided arguments produce an
    /// internally valid `Note`. However, it allows notes to be constructed in a way that
    /// violates required security checks for note decryption, as specified in
    /// [Section 4.19] of the Zcash Protocol Specification. Users of this constructor
    /// should only call it with note components that have been fully validated by
    /// decrypting a received note according to [Section 4.19].
    ///
    /// [Section 4.19]: https://zips.z.cash/protocol/protocol.pdf#saplingandorchardinband
    pub fn from_parts(
        recipient: Arc<ZcashOrchardAddress>,
        value: Arc<ZcashOrchardNoteValue>,
        rho: Arc<ZcashOrchardNullifier>,
        rseed: Arc<ZcashOrchardRandomSeed>,
    ) -> ZcashResult<Self> {
        let opt: Option<Note> = Note::from_parts(
            recipient.as_ref().into(),
            value.as_ref().into(),
            rho.as_ref().into(),
            rseed.as_ref().into(),
        )
        .into();
        match opt {
            Some(note) => Ok(note.into()),
            None => Err("Cannot parse from bytes".into()),
        }
    }

    /// Derives the commitment to this note.
    ///
    /// Defined in [Zcash Protocol Spec § 3.2: Notes][notes].
    ///
    /// [notes]: https://zips.z.cash/protocol/nu5.pdf#notes
    pub fn commitment(&self) -> Arc<ZcashOrchardNoteCommitment> {
        Arc::new(self.0.commitment().into())
    }
}

impl From<Note> for ZcashOrchardNote {
    fn from(inner: Note) -> Self {
        ZcashOrchardNote(inner)
    }
}

impl From<&ZcashOrchardNote> for Note {
    fn from(value: &ZcashOrchardNote) -> Self {
        value.0
    }
}

/// A unique nullifier for a note.
pub struct ZcashOrchardNullifier(Nullifier);

impl ZcashOrchardNullifier {
    /// Deserialize the nullifier from a byte array.
    pub fn from_bytes(bytes: &[u8]) -> ZcashResult<Self> {
        let opt: Option<Nullifier> = Nullifier::from_bytes(&cast_slice(bytes)?).into();
        match opt {
            Some(nullifier) => Ok(nullifier.into()),
            None => Err("Cannot parse from bytes".into()),
        }
    }

    /// Serialize the nullifier to its canonical byte representation.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

impl From<Nullifier> for ZcashOrchardNullifier {
    fn from(inner: Nullifier) -> Self {
        ZcashOrchardNullifier(inner)
    }
}

impl From<&ZcashOrchardNullifier> for Nullifier {
    fn from(value: &ZcashOrchardNullifier) -> Self {
        value.0
    }
}

/// The ZIP 212 seed randomness for a note.
pub struct ZcashOrchardRandomSeed(RandomSeed);

impl ZcashOrchardRandomSeed {
    /// Reads a note's random seed from bytes, given the note's nullifier.
    ///
    /// Returns `None` if the nullifier is not for the same note as the seed.
    pub fn from_bytes(bytes: &[u8], rho: Arc<ZcashOrchardNullifier>) -> ZcashResult<Self> {
        let opt: Option<RandomSeed> =
            RandomSeed::from_bytes(cast_slice(bytes)?, &rho.as_ref().into()).into();
        match opt {
            Some(rseed) => Ok(rseed.into()),
            None => Err("Cannot parse from bytes".into()),
        }
    }

    /// Returns the byte array corresponding to this seed.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}

impl From<RandomSeed> for ZcashOrchardRandomSeed {
    fn from(inner: RandomSeed) -> Self {
        ZcashOrchardRandomSeed(inner)
    }
}

impl From<&ZcashOrchardRandomSeed> for RandomSeed {
    fn from(value: &ZcashOrchardRandomSeed) -> Self {
        value.0
    }
}
