use zcash_client_sqlite::chain::BlockMeta;

use crate::{ZcashConsensusParameters, ZcashError, ZcashResult};
use zcash_client_backend::data_api::WalletRead;
use zcash_client_sqlite::{chain::init, FsBlockDb, WalletDb};

/// Data structure representing a row in the block metadata database.
// #[cfg(feature = "unstable")]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZcashBlockMeta(pub BlockMeta);

// #[cfg(feature = "unstable")]
impl ZcashBlockMeta {
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

/// Sets up the internal structure of the metadata cache database.
///
/// # Examples
///
/// ```
/// use tempfile::{tempdir, NamedTempFile};
/// use zcash_client_sqlite::{
///     FsBlockDb,
///     chain::init::init_blockmeta_db,
/// };
///
/// let cache_file = NamedTempFile::new().unwrap();
/// let blocks_dir = tempdir().unwrap();
/// let mut db = FsBlockDb::for_path(blocks_dir.path()).unwrap();
/// init_blockmeta_db(&mut db).unwrap();
/// ```

// NOTE the UDL format seemingly doesn't let me put a void function in the global scope,
// so I had to put it in an empty struct.
#[derive(Default)]
pub struct ZcashChain;

impl ZcashChain {
    pub fn new() -> Self {
        Self
    }

    pub fn init_blockmeta_db(&self, blocks_dir: String) -> Result<(), ZcashError> {
        let mut db = FsBlockDb::for_path(blocks_dir).unwrap();

        match init::init_blockmeta_db(&mut db) {
            Ok(()) => Ok(()),
            _ => Err(ZcashError::Message {
                error: "MigratorError".to_string(),
            }),
        }
    }

    // NOTE: this was originally in native.rs, but it suits here
    pub fn get_nearest_rewind_height(
        &self,
        db_data: String,
        height: u32,
        params: ZcashConsensusParameters,
    ) -> ZcashResult<u32> {
        if height < 100 {
            Ok(height)
        } else {
            let db_data =
                WalletDb::for_path(db_data, params).expect("Could not connect to WalletDb");

            match db_data.get_min_unspent_height() {
                Ok(Some(best_height)) => Ok(std::cmp::min(best_height.into(), height)),
                Ok(None) => Ok(height),
                Err(e) => Err(ZcashError::Message {
                    error: format!(
                        "Error while getting nearest rewind height for {}: {}",
                        height, e
                    ),
                }),
            }
        }
    }
}
