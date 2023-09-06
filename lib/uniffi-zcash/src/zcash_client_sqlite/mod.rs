use zcash_client_sqlite::WalletDb;
use crate::{ZcashConsensusParameters, ZcashError};
use zcash_primitives::consensus::{MAIN_NETWORK, TEST_NETWORK};
use std::sync::Arc;
use std::sync::Mutex;
use rusqlite::Connection;

mod chain;
pub use self::chain::*;

mod wallet;
pub use self::wallet::*;

// struct SuperWalletDb {
// 	pub main: WalletDb<zcash_primitives::consensus::MainNetwork>,
// 	pub test: WalletDb<zcash_primitives::consensus::TestNetwork>
// }

// /// A wrapper for the SQLite connection to the wallet database.
// pub struct ZcashWalletDb {
// 	pub sup: Arc<SuperWalletDb>,
// 	pub params: ZcashConsensusParameters
// }
pub struct ZcashWalletDb {
	params: ZcashConsensusParameters,
	conn: Arc<Mutex<Connection>>
}

// how to get WDB:
// let zwdb = ZcashWalletDb( ... )
// zwdb.sup.main or zwdb.sup.test
impl ZcashWalletDb {
    /// Construct a connection to the wallet database stored at the specified path.
    // pub fn for_path(path: String, params: ZcashConsensusParameters) -> Result<ZcashWalletDb, ZcashError> {
    // 	let sup = SuperWalletDb {
    // 		main: WalletDb::for_path(path.to_owned(), MAIN_NETWORK).unwrap(),
    // 		test: WalletDb::for_path(path.to_owned(), TEST_NETWORK).unwrap(),
    // 	};
	//     Ok(ZcashWalletDb { sup: Arc::new(sup), params })
    // }
    pub fn for_path(path: String, params: ZcashConsensusParameters) -> Result<ZcashWalletDb, ZcashError> {
    	let conn = Connection::open(&path);
	    Ok(ZcashWalletDb { conn: Arc::new(Mutex::new(conn.unwrap())), params })
    }
    // apparently it works but how to use this?

    // NOTE not needed for now
    // pub fn get_update_ops(&self) -> ZcashResult<DataConnStmtCache<'_, P>, SqliteClientError>
}