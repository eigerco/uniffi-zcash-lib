use crate::{
    ZcashAccountId, ZcashAddressMetadata, ZcashAmount, ZcashBlockHeight, ZcashCommitmentTreeRoot,
    ZcashConsensusParameters, ZcashDecryptedTransaction, ZcashError, ZcashMemo, ZcashOutPoint,
    ZcashResult, ZcashScanRange, ZcashShieldedProtocol, ZcashTransparentAddress, ZcashTxId,
    ZcashUnifiedAddress, ZcashUnifiedFullViewingKey, ZcashWalletSummary,
    ZcashWalletTransparentOutput,
};
use rusqlite::Connection;
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};
use zcash_client_backend::data_api::WalletWrite;
use zcash_client_sqlite::chain::BlockMeta;
// use zcash_client_sqlite::wallet as original_wallet;
use zcash_client_backend::address::AddressMetadata;
use zcash_client_backend::data_api::chain::CommitmentTreeRoot;
use zcash_client_backend::data_api::scanning::ScanRange;
use zcash_client_backend::data_api::NoteId;
use zcash_client_backend::data_api::WalletCommitmentTrees;
use zcash_client_backend::data_api::WalletRead;
use zcash_client_backend::wallet::WalletTransparentOutput;
use zcash_client_sqlite::{FsBlockDb, WalletDb};
use zcash_primitives::consensus::{MAIN_NETWORK, TEST_NETWORK};
use zcash_primitives::legacy::TransparentAddress;
use zcash_primitives::sapling;
use zcash_primitives::transaction::components::Amount;
use zcash_primitives::transaction::components::OutPoint;

mod chain;
pub use self::chain::*;

mod wallet;
pub use self::wallet::*;

/// this is needed because WalletDb uses a generic argument
/// and UniFFI doesn't support it. So we may init all the used types
/// in order to avoid the need of using that argument.
// #[derive(Clone)]
pub struct SuperWalletDb<C> {
    pub main: Mutex<WalletDb<C, zcash_primitives::consensus::MainNetwork>>,
    pub test: Mutex<WalletDb<C, zcash_primitives::consensus::TestNetwork>>,
}

/// A wrapper for the SQLite connection to the wallet database.
pub struct ZcashWalletDb {
    pub sup: Arc<SuperWalletDb<Connection>>,
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
                match self.sup.main.lock().unwrap().get_memo(note) {
                    // NOTE this is stupid
                    Ok(memo) => Ok(memo.unwrap().into()),
                    Err(e) => Err(ZcashError::Message {
                        error: format!("Err: {}", e),
                    }),
                }
            }
            ZcashConsensusParameters::TestNetwork => {
                match self.sup.test.lock().unwrap().get_memo(note) {
                    Ok(memo) => Ok(memo.unwrap().into()),
                    Err(e) => Err(ZcashError::Message {
                        error: format!("Err: {}", e),
                    }),
                }
            }
        }
    }

    pub fn truncate_to_height(&mut self, block_height: ZcashBlockHeight) -> ZcashResult<()> {
        match self.params {
            ZcashConsensusParameters::MainNetwork => self
                .sup
                .main
                .lock()
                .unwrap()
                .truncate_to_height(block_height.into())
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
            ZcashConsensusParameters::TestNetwork => self
                .sup
                .test
                .lock()
                .unwrap()
                .truncate_to_height(block_height.into())
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
        }
    }

    pub fn update_chain_tip(&mut self, tip_height: ZcashBlockHeight) -> ZcashResult<()> {
        match self.params {
            ZcashConsensusParameters::MainNetwork => self
                .sup
                .main
                .lock()
                .unwrap()
                .update_chain_tip(tip_height.into())
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
            ZcashConsensusParameters::TestNetwork => self
                .sup
                .test
                .lock()
                .unwrap()
                .update_chain_tip(tip_height.into())
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
        }
    }

    pub fn get_target_and_anchor_heights(
        &self,
        min_confirmations: NonZeroU32,
    ) -> ZcashResult<Option<(ZcashBlockHeight, ZcashBlockHeight)>> {
        match self.params {
            ZcashConsensusParameters::MainNetwork => {
                match self
                    .sup
                    .main
                    .lock()
                    .unwrap()
                    .get_target_and_anchor_heights(min_confirmations)
                {
                    Ok(None) => Ok(None),
                    Ok(Some((bh1, bh2))) => Ok(Some((bh1.into(), bh2.into()))),
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
                    .get_target_and_anchor_heights(min_confirmations)
                {
                    Ok(None) => Ok(None),
                    Ok(Some((bh1, bh2))) => Ok(Some((bh1.into(), bh2.into()))),
                    Err(e) => Err(ZcashError::Message {
                        error: format!("Err: {}", e),
                    }),
                }
            }
        }
    }

    pub fn get_wallet_summary(
        &self,
        min_confirmations: u32,
    ) -> ZcashResult<Option<ZcashWalletSummary>> {
        match self.params {
            ZcashConsensusParameters::MainNetwork => self
                .sup
                .main
                .lock()
                .unwrap()
                .get_wallet_summary(min_confirmations)
                .map(|x| x.map(From::from))
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
            ZcashConsensusParameters::TestNetwork => self
                .sup
                .test
                .lock()
                .unwrap()
                .get_wallet_summary(min_confirmations)
                .map(|x| x.map(From::from))
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
        }
    }

    pub fn get_account_for_ufvk(
        &mut self,
        zufvk: ZcashUnifiedFullViewingKey,
    ) -> ZcashResult<Option<ZcashAccountId>> {
        match self.params {
            ZcashConsensusParameters::MainNetwork => {
                match self
                    .sup
                    .main
                    .lock()
                    .unwrap()
                    .get_account_for_ufvk(&(zufvk.into()))
                {
                    Ok(aid) => Ok(aid.map(|x| x.into())),
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
                    .get_account_for_ufvk(&(zufvk.into()))
                {
                    Ok(aid) => Ok(aid.map(|x| x.into())),
                    Err(e) => Err(ZcashError::Message {
                        error: format!("Err: {}", e),
                    }),
                }
            }
        }
    }

    pub fn get_transparent_balances(
        &mut self,
        account: ZcashAccountId,
        max_height: ZcashBlockHeight,
    ) -> ZcashResult<HashMap<ZcashTransparentAddress, ZcashAmount>> {
        let convert_hm =
            |hm: HashMap<TransparentAddress, Amount>| -> HashMap<ZcashTransparentAddress, ZcashAmount> {
                hm.into_iter().map(|(x, y)| (x.into(), y.into())).collect()
            };

        match self.params {
            ZcashConsensusParameters::MainNetwork => self
                .sup
                .main
                .lock()
                .unwrap()
                .get_transparent_balances(account.into(), max_height.into())
                .map(convert_hm)
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
            ZcashConsensusParameters::TestNetwork => self
                .sup
                .test
                .lock()
                .unwrap()
                .get_transparent_balances(account.into(), max_height.into())
                .map(convert_hm)
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
        }
    }

    pub fn store_decrypted_tx(&mut self, d_tx: ZcashDecryptedTransaction) -> ZcashResult<()> {
        match self.params {
            ZcashConsensusParameters::MainNetwork => self
                .sup
                .main
                .lock()
                .unwrap()
                .store_decrypted_tx(d_tx.into())
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
            ZcashConsensusParameters::TestNetwork => self
                .sup
                .test
                .lock()
                .unwrap()
                .store_decrypted_tx(d_tx.into())
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
        }
    }

    pub fn get_min_unspent_height(&self) -> ZcashResult<Option<ZcashBlockHeight>> {
        match self.params {
            ZcashConsensusParameters::MainNetwork => {
                match self.sup.main.lock().unwrap().get_min_unspent_height() {
                    Ok(height) => Ok(height.map(From::from)),
                    Err(e) => Err(ZcashError::Message {
                        error: format!("Err: {}", e),
                    }),
                }
            }
            ZcashConsensusParameters::TestNetwork => {
                match self.sup.test.lock().unwrap().get_min_unspent_height() {
                    Ok(height) => Ok(height.map(From::from)),
                    Err(e) => Err(ZcashError::Message {
                        error: format!("Err: {}", e),
                    }),
                }
            }
        }
    }

    pub fn suggest_scan_ranges(&self) -> ZcashResult<Vec<ZcashScanRange>> {
        let heights_arr = |heights: Vec<ScanRange>| -> Vec<ZcashScanRange> {
            heights.into_iter().map(From::from).collect()
        };

        match self.params {
            ZcashConsensusParameters::MainNetwork => self
                .sup
                .main
                .lock()
                .unwrap()
                .suggest_scan_ranges()
                .map(heights_arr)
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),

            ZcashConsensusParameters::TestNetwork => self
                .sup
                .test
                .lock()
                .unwrap()
                .suggest_scan_ranges()
                .map(heights_arr)
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
        }
    }

    pub fn get_current_address(
        &self,
        aid: ZcashAccountId,
    ) -> ZcashResult<Option<ZcashUnifiedAddress>> {
        match self.params {
            ZcashConsensusParameters::MainNetwork => {
                match self
                    .sup
                    .main
                    .lock()
                    .unwrap()
                    .get_current_address(aid.into())
                {
                    Ok(addr) => Ok(addr.map(From::from)),
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
                    .get_current_address(aid.into())
                {
                    Ok(addr) => Ok(addr.map(From::from)),
                    Err(e) => Err(ZcashError::Message {
                        error: format!("Err: {}", e),
                    }),
                }
            }
        }
    }

    pub fn get_transparent_receivers(
        &self,
        aid: ZcashAccountId,
    ) -> ZcashResult<HashMap<ZcashTransparentAddress, ZcashAddressMetadata>> {
        let convert_hm =
            |hm: HashMap<TransparentAddress, AddressMetadata>| -> HashMap<ZcashTransparentAddress, ZcashAddressMetadata> {
                hm.into_iter().map(|(x, y)| (x.into(), y.into())).collect()
            };

        match self.params {
            ZcashConsensusParameters::MainNetwork => self
                .sup
                .main
                .lock()
                .unwrap()
                .get_transparent_receivers(aid.into())
                .map(convert_hm)
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
            ZcashConsensusParameters::TestNetwork => self
                .sup
                .test
                .lock()
                .unwrap()
                .get_transparent_receivers(aid.into())
                .map(convert_hm)
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
        }
    }

    pub fn put_sapling_subtree_roots(
        &self,
        start_index: u64,
        roots: Vec<ZcashCommitmentTreeRoot>,
    ) -> ZcashResult<()> {
        let roots_arr = roots
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<CommitmentTreeRoot<sapling::Node>>>();

        match self.params {
            ZcashConsensusParameters::MainNetwork => self
                .sup
                .main
                .lock()
                .unwrap()
                .put_sapling_subtree_roots(start_index, &roots_arr)
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
            ZcashConsensusParameters::TestNetwork => self
                .sup
                .test
                .lock()
                .unwrap()
                .put_sapling_subtree_roots(start_index, &roots_arr)
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
        }
    }

    pub fn get_unspent_transparent_outputs(
        &self,
        zta: ZcashTransparentAddress,
        zbh: ZcashBlockHeight,
        zop: Vec<ZcashOutPoint>,
    ) -> ZcashResult<Vec<ZcashWalletTransparentOutput>> {
        let zop_arr = zop.iter().map(|x| x.into()).collect::<Vec<OutPoint>>();

        let convert_arr =
            |wtos: Vec<WalletTransparentOutput>| -> Vec<ZcashWalletTransparentOutput> {
                wtos.iter().map(|x| (*x).clone().into()).collect()
            };

        match self.params {
            ZcashConsensusParameters::MainNetwork => self
                .sup
                .main
                .lock()
                .unwrap()
                .get_unspent_transparent_outputs(&(zta.into()), zbh.into(), &zop_arr)
                .map(convert_arr)
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
            ZcashConsensusParameters::TestNetwork => self
                .sup
                .test
                .lock()
                .unwrap()
                .get_unspent_transparent_outputs(&zta.into(), zbh.into(), &zop_arr)
                .map(convert_arr)
                .map_err(|e| ZcashError::Message {
                    error: format!("Err: {}", e),
                }),
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

    /// Returns the metadata for the block with the given height, if it exists in the
    /// database.
    pub fn find_block(&self, height: ZcashBlockHeight) -> ZcashResult<Option<Arc<ZcashBlockMeta>>> {
        match self.fs_block_db.lock().unwrap().find_block(height.into()) {
            Ok(opt) => Ok(opt.map(From::from).map(Arc::new)),
            Err(e) => Err(ZcashError::Message {
                error: format!("FsBlockDbError: {:?}", e),
            }),
        }
    }

    pub fn get_max_cached_height(&self) -> ZcashResult<Option<Arc<ZcashBlockHeight>>> {
        match self.fs_block_db.lock().unwrap().get_max_cached_height() {
            Ok(opt) => Ok(opt.map(From::from).map(Arc::new)),
            Err(e) => Err(ZcashError::Message {
                error: format!("FsBlockDbError: {:?}", e),
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

pub struct ZcashNoteId(NoteId);

impl ZcashNoteId {
    pub fn new(txid: ZcashTxId, zsp: ZcashShieldedProtocol, output_index: u16) -> Self {
        ZcashNoteId(NoteId::new(txid.into(), zsp.into(), output_index))
    }
}

impl From<NoteId> for ZcashNoteId {
    fn from(e: NoteId) -> Self {
        ZcashNoteId(e)
    }
}

impl From<ZcashNoteId> for NoteId {
    fn from(inner: ZcashNoteId) -> Self {
        inner.0
    }
}
