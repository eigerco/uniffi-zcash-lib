use std::sync::Arc;
use zcash_client_sqlite::chain::BlockMeta;

use crate::{ZcashBlockHash, ZcashBlockHeight};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZcashBlockMeta(pub BlockMeta);

impl ZcashBlockMeta {
    pub fn new(
        height: Arc<ZcashBlockHeight>,
        block_hash: Arc<ZcashBlockHash>,
        block_time: u32,
        sapling_outputs_count: u32,
        orchard_actions_count: u32,
    ) -> Self {
        Self(BlockMeta {
            height: (*height).into(),
            block_hash: (*block_hash).into(),
            block_time,
            sapling_outputs_count,
            orchard_actions_count,
        })
    }

    pub fn block_file_path(&self, blocks_dir: String) -> String {
        self.0
            .block_file_path(&blocks_dir)
            .to_string_lossy()
            .to_string()
    }
}

impl From<ZcashBlockMeta> for BlockMeta {
    fn from(inner: ZcashBlockMeta) -> Self {
        inner.0
    }
}

impl From<BlockMeta> for ZcashBlockMeta {
    fn from(e: BlockMeta) -> Self {
        ZcashBlockMeta(e)
    }
}
