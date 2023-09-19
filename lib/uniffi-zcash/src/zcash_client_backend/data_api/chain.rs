use std::sync::Arc;
use zcash_client_backend::data_api::chain;
use zcash_client_backend::data_api::chain::CommitmentTreeRoot;
use zcash_primitives::sapling::Node;

use crate::{
    ZcashBlockHeight, ZcashConsensusParameters, ZcashError, ZcashFsBlockDb, ZcashResult,
    ZcashSaplingNode, ZcashWalletDb,
};

pub fn scan_cached_blocks(
    params: ZcashConsensusParameters,
    z_db_cache: ZcashFsBlockDb,
    z_db_data: ZcashWalletDb,
    height: ZcashBlockHeight,
    limit: u32,
) -> ZcashResult<()> {
    let db_cache = z_db_cache.fs_block_db.into_inner().unwrap();

    match params {
        ZcashConsensusParameters::MainNetwork => {
            let mut main_db_data = z_db_data.sup.main.lock().unwrap();
            match chain::scan_cached_blocks(
                &params,
                &db_cache,
                &mut (*main_db_data),
                height.into(),
                limit as usize,
            ) {
                Ok(_) => Ok(()),
                Err(_) => Err(ZcashError::Unknown),
            }
        }

        ZcashConsensusParameters::TestNetwork => {
            let mut test_db_data = z_db_data.sup.test.lock().unwrap();
            match chain::scan_cached_blocks(
                &params,
                &db_cache,
                &mut (*test_db_data),
                height.into(),
                limit as usize,
            ) {
                Ok(_) => Ok(()),
                Err(_) => Err(ZcashError::Unknown),
            }
        }
    }
}

pub struct ZcashCommitmentTreeRoot(CommitmentTreeRoot<Node>);

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
