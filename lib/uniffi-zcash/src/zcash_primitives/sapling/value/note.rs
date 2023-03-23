use std::sync::Arc;

use zcash_primitives::sapling::{value::NoteValue, Note, Rseed};

use crate::{utils::cast_slice, ZcashError, ZcashJubjubFr, ZcashPaymentAddress, ZcashResult};

pub struct ZcashSaplingNote(Note);

impl ZcashSaplingNote {
    /// Creates a note from its component parts.
    ///
    /// # Caveats
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
        recipient: Arc<ZcashPaymentAddress>,
        value: Arc<ZcashSaplingNoteValue>,
        rseed: ZcashRseed,
    ) -> ZcashResult<Self> {
        Ok(Note::from_parts(
            recipient.as_ref().into(),
            (*value).into(),
            rseed.try_into()?,
        )
        .into())
    }
}

impl From<Note> for ZcashSaplingNote {
    fn from(inner: Note) -> Self {
        ZcashSaplingNote(inner)
    }
}

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

pub enum ZcashRseed {
    /// This expects the data from ZcashJubjubFr,
    /// which can be obtained by calling ZcashJubjubFr::to_bytes().
    BeforeZip212 {
        fr_data: Vec<u8>,
    },
    AfterZip212 {
        data: Vec<u8>,
    },
}

impl TryFrom<ZcashRseed> for Rseed {
    type Error = ZcashError;

    fn try_from(value: ZcashRseed) -> Result<Self, Self::Error> {
        match value {
            ZcashRseed::BeforeZip212 { fr_data } => Ok(Rseed::BeforeZip212(
                ZcashJubjubFr::from_bytes(fr_data.as_slice())?.into(),
            )),
            ZcashRseed::AfterZip212 { data } => {
                let casted_data = cast_slice(data.as_slice())?;
                Ok(Rseed::AfterZip212(casted_data))
            }
        }
    }
}
