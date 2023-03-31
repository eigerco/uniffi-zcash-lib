use zcash_primitives::sapling::note::ExtractedNoteCommitment;

use crate::{utils::cast_slice, ZcashResult};

pub struct ZcashSaplingExtractedNoteCommitment(pub(crate) ExtractedNoteCommitment);

impl ZcashSaplingExtractedNoteCommitment {
    /// Deserialize the extracted note commitment from a byte array.
    ///
    /// This method enforces the [consensus rule][cmucanon] that the byte representation
    /// of cmu MUST be canonical.
    ///
    /// [cmucanon]: https://zips.z.cash/protocol/protocol.pdf#outputencodingandconsensus
    pub fn new(data: &[u8]) -> ZcashResult<Self> {
        let casted_data = cast_slice(data)?;
        let option: Option<ExtractedNoteCommitment> =
            ExtractedNoteCommitment::from_bytes(&casted_data).into();
        match option {
            Some(note_commitment) => Ok(note_commitment.into()),
            None => Err("Invalid data".into()),
        }
    }

    /// Serialize the value commitment to its canonical byte representation.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

impl From<ExtractedNoteCommitment> for ZcashSaplingExtractedNoteCommitment {
    fn from(inner: ExtractedNoteCommitment) -> Self {
        ZcashSaplingExtractedNoteCommitment(inner)
    }
}
