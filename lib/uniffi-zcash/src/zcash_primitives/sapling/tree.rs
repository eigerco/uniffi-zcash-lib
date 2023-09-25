use zcash_primitives::sapling::Node;

use crate::ZcashSaplingExtractedNoteCommitment;

#[derive(Clone, Copy)]
pub struct ZcashSaplingNode(Node);

impl ZcashSaplingNode {
    /// Creates a tree leaf from the given Sapling note commitment.
    pub fn from_cmu(cmu: ZcashSaplingExtractedNoteCommitment) -> Self {
        Self(Node::from_cmu(&cmu.0))
    }
}

impl From<ZcashSaplingNode> for Node {
    fn from(value: ZcashSaplingNode) -> Self {
        value.0
    }
}

impl From<Node> for ZcashSaplingNode {
    fn from(inner: Node) -> Self {
        ZcashSaplingNode(inner)
    }
}
