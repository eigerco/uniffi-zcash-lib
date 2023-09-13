use crate::{
    ZcashBlockHeight, ZcashConsensusParameters, ZcashError, ZcashResult,
    ZcashWalletTransparentOutput, ZcashMemo
};
use rusqlite::Connection;
use std::fmt;
use std::sync::{Arc, Mutex};
use zcash_client_backend::data_api::WalletWrite;
use zcash_client_sqlite::chain::BlockMeta;
// use zcash_client_sqlite::wallet as original_wallet;
use zcash_client_sqlite::{NoteId, DataConnStmtCache};
use zcash_client_sqlite::{FsBlockDb, WalletDb};
use zcash_primitives::consensus;
use zcash_primitives::consensus::{MAIN_NETWORK, TEST_NETWORK};
use zcash_client_backend::data_api::WalletRead;

mod chain;
pub use self::chain::*;

mod wallet;
pub use self::wallet::*;

/// this is needed because WalletDb uses a generic argument
/// and UniFFI doesn't support it. So we may init all the used types
/// in order to avoid the need of using that argument.
// #[derive(Clone)]
pub struct SuperWalletDb {
    pub main: Mutex<WalletDb<zcash_primitives::consensus::MainNetwork>>,
    pub test: Mutex<WalletDb<zcash_primitives::consensus::TestNetwork>>,
}

/// A wrapper for the SQLite connection to the wallet database.
pub struct ZcashWalletDb {
    pub sup: Arc<SuperWalletDb>,
    pub params: ZcashConsensusParameters,
}

// #[derive(Debug)]
// pub struct SuperDataConnStmtCache<'a> {
//     pub main: DataConnStmtCache<'a, zcash_primitives::consensus::MainNetwork>,
//     pub test: DataConnStmtCache<'a, zcash_primitives::consensus::TestNetwork>,
// }

// pub struct ZcashDataConnStmtCache<'a> {
//     pub sup: Arc<SuperDataConnStmtCache<'a>>,
//     pub params: ZcashConsensusParameters,
// }

// pub struct ZcashDataConnStmtCacheTwo<'a, P> {
//     pub data: Arc<DataConnStmtCache<'a, P>>,
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


    //NOTE: if I use this approach, it complains about a reference borrowed and not given back

    // pub fn get_update_ops(&self) -> Arc<ZcashDataConnStmtCache> {
    //     let man = (&self.sup).main.lock().unwrap();
    //     let tst = (&self.sup).test.lock().unwrap();

    //     let sup = SuperDataConnStmtCache {
    //         main: man.get_update_ops().unwrap(),
    //         test: tst.get_update_ops().unwrap(),
    //     };

    //     Arc::new(ZcashDataConnStmtCache {
    //         sup: Arc::new(sup),
    //         params: self.params,
    //     })
    // }


    //NOTE: if I use this, something else is wrong

    // match self.params {
    //     ZcashConsensusParameters::MainNetwork => {
    //         let tmp = (&self.sup).main.lock().unwrap().get_update_ops().unwrap();
    //         Arc::new( ZcashDataConnStmtCacheTwo { data: Arc::new(tmp) } )
    //     } ,
    //     ZcashConsensusParameters::TestNetwork => {
    //         let tmp = (&self.sup).test.lock().unwrap().get_update_ops().unwrap();
    //         Arc::new( ZcashDataConnStmtCacheTwo { data: Arc::new(tmp) } )
    //     }
    // }

    pub fn put_received_transparent_utxo(
        &mut self,
        output: &ZcashWalletTransparentOutput,
    ) -> ZcashResult<i64> {
        match self.params {
            ZcashConsensusParameters::MainNetwork => {
                match self
                    .sup
                    .main
                    .lock()
                    .unwrap()
                    .get_update_ops()
                    .unwrap()
                    .put_received_transparent_utxo(&output.0)
                {
                    Ok(utxo_id) => Ok(utxo_id.0),
                    Err(e) => Err(ZcashError::Message {
                        error: format!("Err: {}", e),
                    }),
                }
            }
            ZcashConsensusParameters::TestNetwork => {
                match self
                    .sup
                    .test
                    .lock()
                    .unwrap()
                    .get_update_ops()
                    .unwrap()
                    .put_received_transparent_utxo(&output.0)
                {
                    Ok(utxo_id) => Ok(utxo_id.0),
                    Err(e) => Err(ZcashError::Message {
                        error: format!("Err: {}", e),
                    }),
                }
            }
        }
    }

    pub fn get_memo(&self, id_note: ZcashNoteId) -> ZcashResult<ZcashMemo> {
        let note: NoteId = id_note.into();

        match self.params {
            ZcashConsensusParameters::MainNetwork => {
                match self
                    .sup
                    .main
                    .lock()
                    .unwrap()
                    .get_update_ops()
                    .unwrap()
                    .get_memo(note)
                {
                    Ok(memo) => Ok(memo.into()),
                    Err(e) => Err(ZcashError::Message {
                        error: format!("Err: {}", e),
                    }),
                }
            }
            ZcashConsensusParameters::TestNetwork => {
                match self
                    .sup
                    .test
                    .lock()
                    .unwrap()
                    .get_update_ops()
                    .unwrap()
                    .get_memo(note)
                {
                    Ok(memo) => Ok(memo.into()),
                    Err(e) => Err(ZcashError::Message {
                        error: format!("Err: {}", e),
                    }),
                }
            }
        }
    }
}

pub struct ZcashFsBlockDb {
    pub fs_block_db: Mutex<FsBlockDb>,
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

    // pub fn get_memo
}

pub enum ZcashNoteId {
    SentNoteId{v: i64},
    ReceivedNoteId{v: i64},
}

impl From<NoteId> for ZcashNoteId {
    fn from(e: NoteId) -> Self {
        match e {
            NoteId::SentNoteId(v) => ZcashNoteId::SentNoteId{ v },
            NoteId::ReceivedNoteId(v) => ZcashNoteId::ReceivedNoteId{ v },
        }
    }
}

impl From<ZcashNoteId> for NoteId {
    fn from(e: ZcashNoteId) -> Self {
        match e {
            ZcashNoteId::SentNoteId{ v } => NoteId::SentNoteId(v),
            ZcashNoteId::ReceivedNoteId{ v } => NoteId::ReceivedNoteId(v),
        }
    }
}

