use std::{fmt, sync::Arc};
use zcash_client_backend::data_api::chain;
use zcash_client_backend::data_api::chain::CommitmentTreeRoot;
use zcash_primitives::sapling::Node;

use crate::{
    ZcashBlockHeight, ZcashConsensusParameters, ZcashError, ZcashFsBlockDb, ZcashResult,
    ZcashSaplingNode, ZcashWalletDb,
};

pub struct ZcashBackendScan();

impl ZcashBackendScan {
    pub fn scan_cached_blocks(
        &self,
        params: ZcashConsensusParameters,
        z_db_cache: Arc<ZcashFsBlockDb>,
        z_db_data: Arc<ZcashWalletDb>,
        height: Arc<ZcashBlockHeight>,
        limit: u32,
    ) -> ZcashResult<()> {
        let z_db_cache = Arc::try_unwrap(z_db_cache).unwrap();
        let db_cache = z_db_cache.fs_block_db.into_inner().unwrap();

        match params {
            ZcashConsensusParameters::MainNetwork => {
                let mut main_db_data = z_db_data.sup.main.lock().unwrap();

                chain::scan_cached_blocks(
                    &params,
                    &db_cache,
                    &mut (*main_db_data),
                    (*height).into(),
                    limit as usize,
                )
                .map_err(|_| ZcashError::Unknown)
            }

            ZcashConsensusParameters::TestNetwork => {
                let mut test_db_data = z_db_data.sup.test.lock().unwrap();

                chain::scan_cached_blocks(
                    &params,
                    &db_cache,
                    &mut (*test_db_data),
                    (*height).into(),
                    limit as usize,
                )
                .map_err(|_| ZcashError::Unknown)
            }
        }
    }
}

pub struct ZcashCommitmentTreeRoot(CommitmentTreeRoot<Node>);

// NOTE change this
impl fmt::Debug for ZcashCommitmentTreeRoot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "needed for Arc taking out")
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
