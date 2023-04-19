use std::sync::Arc;

use orchard::note::{ExtractedNoteCommitment, NoteCommitment};

use crate::{utils::cast_slice, ZcashResult};

pub struct ZcashOrchardNoteCommitment(NoteCommitment);

impl ZcashOrchardNoteCommitment {
    pub fn to_extracted_note_commitment(&self) -> Arc<ZcashExtractedNoteCommitment> {
        Arc::new(self.into())
    }
}

impl From<NoteCommitment> for ZcashOrchardNoteCommitment {
    fn from(inner: NoteCommitment) -> Self {
        ZcashOrchardNoteCommitment(inner)
    }
}

impl From<&ZcashOrchardNoteCommitment> for ZcashExtractedNoteCommitment {
    fn from(value: &ZcashOrchardNoteCommitment) -> Self {
        ZcashExtractedNoteCommitment(value.0.clone().into())
    }
}

pub struct ZcashExtractedNoteCommitment(ExtractedNoteCommitment);

impl ZcashExtractedNoteCommitment {
    pub fn from_bytes(data: &[u8]) -> ZcashResult<Self> {
        let casted_data = cast_slice(data)?;
        let opt: Option<ExtractedNoteCommitment> =
            ExtractedNoteCommitment::from_bytes(&casted_data).into();
        match opt {
            Some(enc) => Ok(enc.into()),
            None => Err("Cannot parse bytes".into()),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

impl From<&ExtractedNoteCommitment> for ZcashExtractedNoteCommitment {
    fn from(inner: &ExtractedNoteCommitment) -> Self {
        ZcashExtractedNoteCommitment(*inner)
    }
}

impl From<&ZcashExtractedNoteCommitment> for ExtractedNoteCommitment {
    fn from(value: &ZcashExtractedNoteCommitment) -> Self {
        value.0
    }
}

impl From<ExtractedNoteCommitment> for ZcashExtractedNoteCommitment {
    fn from(inner: ExtractedNoteCommitment) -> Self {
        ZcashExtractedNoteCommitment(inner)
    }
}
