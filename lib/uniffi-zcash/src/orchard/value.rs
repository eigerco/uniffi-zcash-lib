use derive_more::{From, Into};
use orchard::value::{NoteValue, ValueCommitment};

/// The non-negative value of an individual Orchard note.
#[derive(Clone, Copy, From, Into)]
pub struct ZcashOrchardNoteValue(NoteValue);

impl ZcashOrchardNoteValue {
    /// Creates a note value from its raw numeric value.
    ///
    /// This only enforces that the value is an unsigned 64-bit integer. Callers should
    /// enforce any additional constraints on the value's valid range themselves.
    pub fn from_raw(value: u64) -> Self {
        NoteValue::from_raw(value).into()
    }

    /// Returns the raw underlying value.
    pub fn value(&self) -> u64 {
        self.0.inner()
    }
}

pub struct ZcashOrchardValueCommitment(ValueCommitment);

impl ZcashOrchardValueCommitment {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

impl From<&ValueCommitment> for ZcashOrchardValueCommitment {
    fn from(inner: &ValueCommitment) -> Self {
        ZcashOrchardValueCommitment(inner.clone())
    }
}
