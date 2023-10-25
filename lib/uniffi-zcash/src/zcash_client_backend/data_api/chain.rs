use std::sync::Arc;
use zcash_client_backend::data_api::chain;
use zcash_client_backend::data_api::chain::CommitmentTreeRoot;
use zcash_client_sqlite::{FsBlockDb, WalletDb};
use zcash_primitives::sapling::Node;

use crate::{
    ZcashBlockHeight, ZcashConsensusParameters, ZcashError, ZcashResult, ZcashSaplingNode,
};

pub fn scan_cached_blocks(
    params: ZcashConsensusParameters,
    fsblockdb_root: String,
    db_data_path: String,
    height: Arc<ZcashBlockHeight>,
    limit: u32,
) -> ZcashResult<()> {
    let db_cache = FsBlockDb::for_path(fsblockdb_root).expect("Cannot access FsBlockDb");
    let mut db_data = WalletDb::for_path(db_data_path, params).expect("Cannot access WalletDb");

    chain::scan_cached_blocks(
        &params,
        &db_cache,
        &mut db_data,
        (*height).into(),
        limit as usize,
    )
    .map_err(|e| ZcashError::Message {
        error: format!("Error for scan_cached_blocks: {:?}", e),
    })
}

pub struct ZcashCommitmentTreeRoot(CommitmentTreeRoot<Node>);

impl Clone for ZcashCommitmentTreeRoot {
    fn clone(&self) -> Self {
        Self(CommitmentTreeRoot::from_parts(
            self.0.subtree_end_height(),
            *self.0.root_hash(),
        ))
    }
}

impl ZcashCommitmentTreeRoot {
    /// Construct a new `CommitmentTreeRoot` from its constituent parts.
    pub fn from_parts(
        subtree_end_height: Arc<ZcashBlockHeight>,
        root_hash: Arc<ZcashSaplingNode>,
    ) -> Self {
        Self(CommitmentTreeRoot::from_parts(
            (*subtree_end_height).into(),
            (*root_hash).into(),
        ))
    }

    /// Returns the block height at which the leaf that completed the subtree was added.
    pub fn subtree_end_height(&self) -> Arc<ZcashBlockHeight> {
        Arc::new(self.0.subtree_end_height().into())
    }

    /// Returns the root of the complete subtree.
    pub fn root_hash(&self) -> Arc<ZcashSaplingNode> {
        Arc::new((*self.0.root_hash()).into())
    }
}

impl From<ZcashCommitmentTreeRoot> for CommitmentTreeRoot<Node> {
    fn from(outer: ZcashCommitmentTreeRoot) -> Self {
        outer.0
    }
}

impl From<CommitmentTreeRoot<Node>> for ZcashCommitmentTreeRoot {
    fn from(inner: CommitmentTreeRoot<Node>) -> Self {
        Self(inner)
    }
}
