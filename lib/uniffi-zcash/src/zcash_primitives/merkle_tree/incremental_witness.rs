use std::sync::{Arc, RwLock};

use zcash_primitives::{merkle_tree::IncrementalWitness, sapling::Node};

use crate::{ZcashCommitmentTree, ZcashResult, ZcashSaplingNode};

pub struct ZcashIncrementalWitness(RwLock<IncrementalWitness<Node>>);

impl ZcashIncrementalWitness {
    /// Creates an `IncrementalWitness` for the most recent commitment added to the given
    pub fn from_tree(tree: &ZcashCommitmentTree) -> Self {
        IncrementalWitness::from_tree(&tree.into()).into()
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
}

impl From<IncrementalWitness<Node>> for ZcashIncrementalWitness {
    fn from(value: IncrementalWitness<Node>) -> Self {
        ZcashIncrementalWitness(RwLock::new(value))
    }
}
