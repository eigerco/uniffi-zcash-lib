use std::sync::Arc;
use zcash_primitives::sapling::Node;

use crate::ZcashSaplingExtractedNoteCommitment;

use derive_more::{From, Into};

#[derive(Clone, Copy, From, Into)]
pub struct ZcashSaplingNode(Node);

impl ZcashSaplingNode {
    /// Creates a tree leaf from the given Sapling note commitment.
    pub fn from_cmu(cmu: Arc<ZcashSaplingExtractedNoteCommitment>) -> Self {
        Self(Node::from_cmu(&cmu.0))
    }
}
