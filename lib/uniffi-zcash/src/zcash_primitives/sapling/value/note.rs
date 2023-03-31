use zcash_primitives::sapling::value::NoteValue;

#[derive(Clone, Copy)]
pub struct ZcashSaplingNoteValue(NoteValue);

impl ZcashSaplingNoteValue {
    pub fn from_raw(data: u64) -> Self {
        NoteValue::from_raw(data).into()
    }

    pub fn inner(&self) -> u64 {
        self.0.inner()
    }
}

impl From<NoteValue> for ZcashSaplingNoteValue {
    fn from(inner: NoteValue) -> Self {
        ZcashSaplingNoteValue(inner)
    }
}

impl From<ZcashSaplingNoteValue> for NoteValue {
    fn from(value: ZcashSaplingNoteValue) -> Self {
        value.0
    }
}
