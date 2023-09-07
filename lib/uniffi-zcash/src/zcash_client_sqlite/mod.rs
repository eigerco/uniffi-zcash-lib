use crate::{ZcashBlockHeight, ZcashConsensusParameters, ZcashError, ZcashResult};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use zcash_client_sqlite::chain::BlockMeta;
use zcash_client_sqlite::{FsBlockDb, WalletDb};
use zcash_primitives::consensus::{MAIN_NETWORK, TEST_NETWORK};

mod chain;
pub use self::chain::*;

mod wallet;
pub use self::wallet::*;

/// this is needed because WalletDb uses a generic argument
/// and UniFFI doesn't support it. So we may init all the used types
/// in order to avoid the need of using that argument.
pub struct SuperWalletDb {
    pub main: Mutex<WalletDb<zcash_primitives::consensus::MainNetwork>>,
    pub test: Mutex<WalletDb<zcash_primitives::consensus::TestNetwork>>,
}

/// A wrapper for the SQLite connection to the wallet database.
pub struct ZcashWalletDb {
    pub sup: Arc<SuperWalletDb>,
    pub params: ZcashConsensusParameters,
}

// pub struct ZcashWalletDb {
// 	params: ZcashConsensusParameters,
// 	conn: Arc<Mutex<Connection>>
// }

// how to get WDB:
// let zwdb = ZcashWalletDb( ... )
// zwdb.sup.main or zwdb.sup.test
impl ZcashWalletDb {
    /// Construct a connection to the wallet database stored at the specified path.
    pub fn for_path(path: String, params: ZcashConsensusParameters) -> ZcashResult<Self> {
        let sup = SuperWalletDb {
            main: Mutex::new(WalletDb::for_path(&path, MAIN_NETWORK).unwrap()),
            test: Mutex::new(WalletDb::for_path(&path, TEST_NETWORK).unwrap()),
        };
        Ok(ZcashWalletDb {
            sup: Arc::new(sup),
            params,
        })
    }
    // pub fn for_path(path: String, params: ZcashConsensusParameters) -> Result<ZcashWalletDb, ZcashError> {
    // 	let conn = Connection::open(&path);
    //     Ok(ZcashWalletDb { conn: Arc::new(Mutex::new(conn.unwrap())), params })
    // }

    // NOTE not needed for now
    // pub fn get_update_ops(&self) -> ZcashResult<DataConnStmtCache<'_, P>, SqliteClientError>
}

pub struct ZcashFsBlockDb {
    fs_block_db: Mutex<FsBlockDb>,
}

impl ZcashFsBlockDb {
    pub fn for_path(fsblockdb_root: String) -> ZcashResult<Self> {
        Ok(ZcashFsBlockDb {
            fs_block_db: Mutex::new(FsBlockDb::for_path(fsblockdb_root).unwrap()),
        })
    }

    pub fn get_max_cached_height(&self) -> ZcashResult<Option<Arc<ZcashBlockHeight>>> {
        match self.fs_block_db.lock().unwrap().get_max_cached_height() {
            Ok(opt) => Ok(opt.map(From::from).map(Arc::new)),
            Err(_e) => Err(ZcashError::Message {
                error: "FsBlockDbError".to_string(),
            }),
        }
    }

    // NOTE why I can't export ZcashBlockMeta in the whole crate?
    pub fn write_block_metadata(
        &self,
        block_meta: Vec<Arc<chain::ZcashBlockMeta>>,
    ) -> ZcashResult<()> {
        let vec = block_meta
            .into_iter()
            .map(|x| Arc::try_unwrap(x).unwrap())
            .map(From::from)
            .collect::<Vec<BlockMeta>>();

        let res = self
            .fs_block_db
            .lock()
            .unwrap()
            .write_block_metadata(&vec[..]);

        match res {
            Ok(_) => Ok(()),
            Err(_e) => Err(ZcashError::Message {
                error: "err err err".to_string(),
            }),
        }
    }
}
