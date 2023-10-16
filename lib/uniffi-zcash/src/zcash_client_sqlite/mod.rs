use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};
use zcash_client_backend::address::AddressMetadata;
use zcash_client_backend::data_api::chain::CommitmentTreeRoot;
use zcash_client_backend::data_api::scanning::ScanRange;
use zcash_client_backend::data_api::{NoteId, WalletCommitmentTrees, WalletRead, WalletWrite};
use zcash_client_backend::encoding::AddressCodec;
use zcash_client_backend::keys::UnifiedFullViewingKey;
use zcash_client_backend::wallet::WalletTransparentOutput;
use zcash_client_sqlite::{chain::BlockMeta, FsBlockDb, WalletDb};
use zcash_primitives::transaction::components::{Amount, OutPoint};
use zcash_primitives::zip32::AccountId;
use zcash_primitives::{legacy::TransparentAddress, sapling};

mod chain;
pub use self::chain::*;

mod wallet;
pub use self::wallet::*;

use crate::{
    ZcashAccountId, ZcashAddressMetadata, ZcashAmount, ZcashBlockHeight, ZcashBlockMetadata,
    ZcashCommitmentTreeRoot, ZcashConsensusParameters, ZcashDecryptedTransaction, ZcashError,
    ZcashMemo, ZcashOutPoint, ZcashResult, ZcashScanRange, ZcashShieldedProtocol,
    ZcashTransparentAddress, ZcashTxId, ZcashUnifiedAddress, ZcashUnifiedFullViewingKey,
    ZcashWalletSummary, ZcashWalletTransparentOutput,
};

pub struct MinAndMaxZcashBlockHeight {
    pub min: Arc<ZcashBlockHeight>,
    pub max: Arc<ZcashBlockHeight>,
}

/// A wrapper for the SQLite connection to the wallet database.
pub struct ZcashWalletDb {
    pub path: String,
    pub params: ZcashConsensusParameters,
}

fn cast_err(e: zcash_client_sqlite::error::SqliteClientError) -> ZcashError {
    ZcashError::Message {
        error: format!("Err: {:?}", e),
    }
}

impl ZcashWalletDb {
    /// Construct a connection to the wallet database stored at the specified path.
    pub fn for_path(path: String, params: ZcashConsensusParameters) -> ZcashResult<Self> {
        Ok(ZcashWalletDb { path, params })
    }

    pub fn chain_height(&self) -> ZcashResult<Option<Arc<ZcashBlockHeight>>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .chain_height()
            .map(|x| x.map(|y| Arc::new(y.into())))
            .map_err(cast_err)
    }

    pub fn block_metadata(
        &self,
        height: Arc<ZcashBlockHeight>,
    ) -> ZcashResult<Option<Arc<ZcashBlockMetadata>>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .block_metadata((*height).into())
            .map(|x| x.map(|y| Arc::new(y.into())))
            .map_err(cast_err)
    }

    pub fn block_fully_scanned(&self) -> ZcashResult<Option<Arc<ZcashBlockMetadata>>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .block_fully_scanned()
            .map(|x| x.map(|y| Arc::new(y.into())))
            .map_err(cast_err)
    }

    pub fn block_max_scanned(&self) -> ZcashResult<Option<Arc<ZcashBlockMetadata>>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .block_max_scanned()
            .map(|x| x.map(|y| Arc::new(y.into())))
            .map_err(cast_err)
    }

    pub fn get_unified_full_viewing_keys(
        &self,
    ) -> ZcashResult<HashMap<ZcashAccountId, Arc<ZcashUnifiedFullViewingKey>>> {
        let convert_hm =
            |hm: HashMap<AccountId, UnifiedFullViewingKey>| -> HashMap<ZcashAccountId, Arc<ZcashUnifiedFullViewingKey>> {
                hm.into_iter().map(|(x, y)| (x.into(), Arc::new(y.into()))).collect()
            };

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_unified_full_viewing_keys()
            .map(convert_hm)
            .map_err(cast_err)
    }

    pub fn put_received_transparent_utxo(
        &self,
        output: Arc<ZcashWalletTransparentOutput>,
    ) -> ZcashResult<i64> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .put_received_transparent_utxo(&output.0)
            .map(|x| x.0)
            .map_err(cast_err)
    }

    pub fn get_memo(&self, id_note: Arc<ZcashNoteId>) -> ZcashResult<ZcashMemo> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_memo((*id_note).into())
            .map(|memo| memo.unwrap().into())
            .map_err(cast_err)
    }

    pub fn truncate_to_height(&self, block_height: u32) -> ZcashResult<()> {
        let zheight = ZcashBlockHeight::new(block_height).into();

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .truncate_to_height(zheight)
            .map_err(cast_err)
    }

    pub fn update_chain_tip(&self, tip_height: u32) -> ZcashResult<()> {
        let zheight = ZcashBlockHeight::new(tip_height).into();

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .update_chain_tip(zheight)
            .map_err(cast_err)
    }

    pub fn get_target_and_anchor_heights(
        &self,
        min_confirmations: u32,
    ) -> ZcashResult<Option<MinAndMaxZcashBlockHeight>> {
        let min = NonZeroU32::new(min_confirmations).unwrap();

        match WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_target_and_anchor_heights(min)
        {
            Ok(None) => Ok(None),
            Ok(Some((bh1, bh2))) => Ok(Some(MinAndMaxZcashBlockHeight {
                min: Arc::new(bh1.into()),
                max: Arc::new(bh2.into()),
            })),
            Err(e) => Err(ZcashError::Message {
                error: format!("Err: {}", e),
            }),
        }
    }

    pub fn get_wallet_summary(
        &self,
        min_confirmations: u32,
    ) -> ZcashResult<Option<Arc<ZcashWalletSummary>>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_wallet_summary(min_confirmations)
            .map(|x| x.map(From::from).map(Arc::new))
            .map_err(cast_err)
    }

    pub fn get_account_for_ufvk(
        &self,
        zufvk: Arc<ZcashUnifiedFullViewingKey>,
    ) -> ZcashResult<Option<ZcashAccountId>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_account_for_ufvk(&((*zufvk).clone().into()))
            .map(|aid| aid.map(From::from))
            .map_err(cast_err)
    }

    pub fn get_transparent_balances(
        &self,
        account: ZcashAccountId,
        max_height: Arc<ZcashBlockHeight>,
    ) -> ZcashResult<HashMap<Arc<ZcashTransparentAddress>, Arc<ZcashAmount>>> {
        let convert_hm =
            |hm: HashMap<TransparentAddress, Amount>| -> HashMap<Arc<ZcashTransparentAddress>, Arc<ZcashAmount>> {
                hm.into_iter().map(|(x, y)| (Arc::new(x.into()), Arc::new(y.into()))).collect()
            };

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_transparent_balances(account.into(), (*max_height).into())
            .map(convert_hm)
            .map_err(cast_err)
    }

    pub fn store_decrypted_tx(&self, d_tx: Arc<ZcashDecryptedTransaction>) -> ZcashResult<()> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .store_decrypted_tx((*d_tx).clone().into())
            .map_err(cast_err)
    }

    pub fn get_min_unspent_height(&self) -> ZcashResult<Option<Arc<ZcashBlockHeight>>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_min_unspent_height()
            .map(|x| x.map(From::from).map(Arc::new))
            .map_err(cast_err)
    }

    pub fn suggest_scan_ranges(&self) -> ZcashResult<Vec<Arc<ZcashScanRange>>> {
        let heights = |heights: Vec<ScanRange>| -> Vec<Arc<ZcashScanRange>> {
            heights.into_iter().map(From::from).map(Arc::new).collect()
        };

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .suggest_scan_ranges()
            .map(heights)
            .map_err(cast_err)
    }

    pub fn get_current_address(
        &self,
        aid: ZcashAccountId,
    ) -> ZcashResult<Option<Arc<ZcashUnifiedAddress>>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_current_address(aid.into())
            .map(|x| x.map(From::from).map(Arc::new))
            .map_err(cast_err)
    }

    pub fn get_transparent_receivers(
        &self,
        aid: ZcashAccountId,
    ) -> ZcashResult<HashMap<String, Arc<ZcashAddressMetadata>>> {
        let convert_hm =
            |hm: HashMap<TransparentAddress, AddressMetadata>| -> HashMap<String, Arc<ZcashAddressMetadata>> {
                hm.into_iter().map(|(x, y)| (x.encode(&self.params), Arc::new(y.into()))).collect()
            };

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_transparent_receivers(aid.into())
            .map(convert_hm)
            .map_err(cast_err)
    }

    pub fn put_sapling_subtree_roots(
        &self,
        start_index: u64,
        roots: Vec<Arc<ZcashCommitmentTreeRoot>>,
    ) -> ZcashResult<()> {
        let roots_arr = roots
            .into_iter()
            .map(|x| (*x).clone().into())
            .collect::<Vec<CommitmentTreeRoot<sapling::Node>>>();

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .put_sapling_subtree_roots(start_index, &roots_arr)
            .map_err(|e| ZcashError::Message {
                error: format!("ShardTreeError: {:?}", e),
            })
    }

    pub fn get_unspent_transparent_outputs(
        &self,
        zta: Arc<ZcashTransparentAddress>,
        zbh: Arc<ZcashBlockHeight>,
        zop: Vec<Arc<ZcashOutPoint>>,
    ) -> ZcashResult<Vec<Arc<ZcashWalletTransparentOutput>>> {
        let zop_arr = zop
            .into_iter()
            .map(|x| (*x).clone().into())
            .collect::<Vec<OutPoint>>();

        let convert_arr =
            |wtos: Vec<WalletTransparentOutput>| -> Vec<Arc<ZcashWalletTransparentOutput>> {
                wtos.into_iter()
                    .map(|x| Arc::new(x.clone().into()))
                    .collect()
            };

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_unspent_transparent_outputs(&((*zta).into()), (*zbh).into(), &zop_arr)
            .map(convert_arr)
            .map_err(cast_err)
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
    pub fn find_block(
        &self,
        height: Arc<ZcashBlockHeight>,
    ) -> ZcashResult<Option<Arc<chain::ZcashBlockMeta>>> {
        match self
            .fs_block_db
            .lock()
            .unwrap()
            .find_block((*height).into())
        {
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
            .map(|x| (*x).into())
            .collect::<Vec<BlockMeta>>();

        self.fs_block_db
            .lock()
            .unwrap()
            .write_block_metadata(&vec[..])
            .map(|_| ())
            .map_err(|e| ZcashError::Message {
                error: format!("FsBlockDbError: {:?}", e),
            })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZcashNoteId(NoteId);

impl ZcashNoteId {
    pub fn new(txid: Arc<ZcashTxId>, zsp: ZcashShieldedProtocol, output_index: u16) -> Self {
        ZcashNoteId(NoteId::new((*txid).into(), zsp.into(), output_index))
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
