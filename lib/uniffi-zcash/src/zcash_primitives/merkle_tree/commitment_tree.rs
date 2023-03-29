use std::sync::{Arc, RwLock};

use zcash_primitives::{merkle_tree::CommitmentTree, sapling::Node};

use crate::{ZcashResult, ZcashSaplingNode};

pub struct ZcashCommitmentTree(RwLock<CommitmentTree<Node>>);

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

impl From<&ZcashCommitmentTree> for CommitmentTree<Node> {
    fn from(value: &ZcashCommitmentTree) -> Self {
        value.0.read().unwrap().clone()
    }
}
