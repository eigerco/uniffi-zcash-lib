use std::sync::{Arc, RwLock};

use incrementalmerkletree::frontier::CommitmentTree;
use zcash_primitives::sapling::Node;

use crate::{ZcashResult, ZcashSaplingNode};
const DEPTH: u8 = 32;

pub struct ZcashCommitmentTree(RwLock<CommitmentTree<Node, DEPTH>>);

impl ZcashCommitmentTree {
    /// Creates an empty tree.
    pub fn empty() -> Self {
        ZcashCommitmentTree(RwLock::new(CommitmentTree::empty()))
    }

    /// Adds a leaf node to the tree.
    ///
    /// Returns an error if the tree is full.
    pub fn append(&self, node: Arc<ZcashSaplingNode>) -> ZcashResult<()> {
        match self.0.write().unwrap().append((*node).into()) {
            Ok(_) => Ok(()),
            // Underlying implementation throws an Err(()) when tree is full.
            // Changing behavior here.
            Err(_) => Err("Tree is full".into()),
        }
    }
}

impl From<&ZcashCommitmentTree> for CommitmentTree<Node, DEPTH> {
    fn from(value: &ZcashCommitmentTree) -> Self {
        value.0.read().unwrap().clone()
    }
}
