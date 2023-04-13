mod incremental_witness;
use std::sync::Arc;

use zcash_primitives::{merkle_tree::MerklePath, sapling::Node};

use crate::ZcashSaplingNode;

pub use self::incremental_witness::*;

mod commitment_tree;
pub use self::commitment_tree::*;

pub struct ZcashSaplingMerklePath(MerklePath<Node>);

impl ZcashSaplingMerklePath {
    pub fn auth_path(&self) -> Vec<ZcashAuthPath> {
        self.0
            .auth_path
            .iter()
            .map(|(node, bool)| ZcashAuthPath {
                node: Arc::new((*node).into()),
                bool: *bool,
            })
            .collect()
    }

    pub fn position(&self) -> u64 {
        self.0.position
    }
}

impl From<MerklePath<Node>> for ZcashSaplingMerklePath {
    fn from(inner: MerklePath<Node>) -> Self {
        ZcashSaplingMerklePath(inner)
    }
}

impl From<ZcashSaplingMerklePath> for MerklePath<Node> {
    fn from(value: ZcashSaplingMerklePath) -> Self {
        value.0
    }
}

impl From<&ZcashSaplingMerklePath> for MerklePath<Node> {
    fn from(value: &ZcashSaplingMerklePath) -> Self {
        value.0.clone()
    }
}

pub struct ZcashAuthPath {
    pub node: Arc<ZcashSaplingNode>,
    pub bool: bool,
}
