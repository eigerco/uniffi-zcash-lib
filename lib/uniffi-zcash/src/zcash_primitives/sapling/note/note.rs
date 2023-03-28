use std::sync::Arc;
use zcash_primitives::sapling::Note;
use crate::{ZcashPaymentAddress, ZcashResult, ZcashRseed, ZcashSaplingNoteValue};

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
