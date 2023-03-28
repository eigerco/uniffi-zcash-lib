use zcash_primitives::sapling::{value::NoteValue, Rseed};
use crate::{utils::cast_slice, ZcashError, ZcashJubjubFr};

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
