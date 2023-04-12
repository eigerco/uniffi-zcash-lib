use orchard::value::NoteValue;

/// The non-negative value of an individual Orchard note.
pub struct ZcashOrchardNoteValue(NoteValue);

impl ZcashOrchardNoteValue {
    /// Creates a note value from its raw numeric value.
    ///
    /// This only enforces that the value is an unsigned 64-bit integer. Callers should
    /// enforce any additional constraints on the value's valid range themselves.
    pub fn from_raw(value: u64) -> Self {
        NoteValue::from_raw(value).into()
    }
}

impl From<NoteValue> for ZcashOrchardNoteValue {
    fn from(inner: NoteValue) -> Self {
        ZcashOrchardNoteValue(inner)
    }
}

impl From<&ZcashOrchardNoteValue> for NoteValue {
    fn from(value: &ZcashOrchardNoteValue) -> Self {
        value.0
    }
}
