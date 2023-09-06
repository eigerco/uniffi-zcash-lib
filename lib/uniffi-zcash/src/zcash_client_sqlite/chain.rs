use zcash_client_sqlite::chain::BlockMeta;

mod init;
pub use self::init::*;

/// Data structure representing a row in the block metadata database.
// #[cfg(feature = "unstable")]
pub struct ZcashBlockMeta(BlockMeta);

// #[cfg(feature = "unstable")]
impl ZcashBlockMeta {
    pub fn block_file_path(&self, blocks_dir: String) -> String {
    	self.0.block_file_path(&blocks_dir).to_string_lossy().to_string()
    }
}