mod incremental_witness;
use std::sync::Arc;

use derive_more::{From, Into};
use incrementalmerkletree::MerklePath;
use zcash_primitives::sapling::Node;

use crate::ZcashSaplingNode;
const DEPTH: u8 = 32;

pub use self::incremental_witness::*;

mod commitment_tree;
pub use self::commitment_tree::*;

#[derive(From, Into)]
pub struct ZcashSaplingMerklePath(MerklePath<Node, DEPTH>);

impl ZcashSaplingMerklePath {
    pub fn auth_path(&self) -> Vec<ZcashAuthPath> {
        self.0
            .path_elems()
            .iter()
            .map(|node| ZcashAuthPath {
                node: Arc::new((*node).into()),
            })
            .collect()
    }

    pub fn position(&self) -> u64 {
        self.0.position().into()
    }
}

impl From<&ZcashSaplingMerklePath> for MerklePath<Node, DEPTH> {
    fn from(value: &ZcashSaplingMerklePath) -> Self {
        value.0.clone()
    }
}

pub struct ZcashAuthPath {
    pub node: Arc<ZcashSaplingNode>,
}
