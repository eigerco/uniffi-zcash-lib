use std::sync::{Arc, RwLock};

use incrementalmerkletree::witness::IncrementalWitness;
use zcash_primitives::sapling::Node;

use crate::{ZcashCommitmentTree, ZcashResult, ZcashSaplingMerklePath, ZcashSaplingNode};
const DEPTH: u8 = 32;

pub struct ZcashIncrementalWitness(RwLock<IncrementalWitness<Node, DEPTH>>);

impl ZcashIncrementalWitness {
    /// Creates an `IncrementalWitness` for the most recent commitment added to the given
    pub fn from_tree(tree: ZcashCommitmentTree) -> Self {
        IncrementalWitness::from_tree(tree.into()).into()
    }

    /// Tracks a leaf node that has been added to the underlying tree.
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

    pub fn path(&self) -> Option<Arc<ZcashSaplingMerklePath>> {
        self.0.read().unwrap().path().map(From::from).map(Arc::new)
    }
}

impl From<IncrementalWitness<Node, DEPTH>> for ZcashIncrementalWitness {
    fn from(value: IncrementalWitness<Node, DEPTH>) -> Self {
        ZcashIncrementalWitness(RwLock::new(value))
    }
}
