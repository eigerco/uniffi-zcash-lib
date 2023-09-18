// mod init;
// pub use self::init::*;

use zcash_client_sqlite::chain::BlockMeta;

/// Data structure representing a row in the block metadata database.
// #[cfg(feature = "unstable")]
#[derive(Debug)]
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

use crate::ZcashError;
use zcash_client_sqlite::{chain::init, FsBlockDb};

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
pub struct ZcashChain();

impl ZcashChain {
    pub fn init_blockmeta_db(&self, blocks_dir: String) -> Result<(), ZcashError> {
        let mut db = FsBlockDb::for_path(blocks_dir).unwrap();

        match init::init_blockmeta_db(&mut db) {
            Ok(()) => Ok(()),
            _ => Err(ZcashError::Message {
                error: "MigratorError".to_string(),
            }),
        }
    }
}
