use zcash_primitives::sapling::value::NoteValue;

use derive_more::{From, Into};

#[derive(Clone, Copy, From, Into)]
pub struct ZcashSaplingNoteValue(NoteValue);

impl ZcashSaplingNoteValue {
    pub fn from_raw(data: u64) -> Self {
        NoteValue::from_raw(data).into()
    }

    /// Returns the raw underlying value.
    pub fn inner(&self) -> u64 {
        self.0.inner()
    }
}
