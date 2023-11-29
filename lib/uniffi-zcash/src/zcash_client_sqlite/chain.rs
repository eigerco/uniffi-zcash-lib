use std::sync::Arc;
use zcash_client_sqlite::chain::BlockMeta;

use crate::{ZcashBlockHash, ZcashBlockHeight};

use derive_more::{From, Into};

#[derive(Clone, Copy, Debug, PartialEq, Eq, From, Into)]
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
