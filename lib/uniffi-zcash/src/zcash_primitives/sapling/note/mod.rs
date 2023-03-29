mod commitment;
pub use self::commitment::*;

use crate::{
    utils::cast_slice, ZcashError, ZcashJubjubFr, ZcashPaymentAddress, ZcashResult,
    ZcashSaplingNoteValue,
};

use std::sync::Arc;
use zcash_primitives::sapling::{Note, Rseed};

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

    /// Computes the note commitment
    pub fn cmu(&self) -> Arc<ZcashSaplingExtractedNoteCommitment> {
        Arc::new(self.0.cmu().into())
    }
}

impl From<Note> for ZcashSaplingNote {
    fn from(inner: Note) -> Self {
        ZcashSaplingNote(inner)
    }
}

/// Enum for note randomness before and after [ZIP 212](https://zips.z.cash/zip-0212).
///
/// Before ZIP 212, the note commitment trapdoor `rcm` must be a scalar value.
/// After ZIP 212, the note randomness `rseed` is a 32-byte sequence, used to derive
/// both the note commitment trapdoor `rcm` and the ephemeral private key `esk`.
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
