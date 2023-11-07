use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};

use zcash_client_backend::address::AddressMetadata;
use zcash_client_backend::data_api::chain::CommitmentTreeRoot;
use zcash_client_backend::data_api::scanning::ScanRange;
use zcash_client_backend::data_api::ScannedBlock;
use zcash_client_backend::data_api::{NoteId, WalletCommitmentTrees, WalletRead, WalletWrite};
use zcash_client_backend::encoding::AddressCodec;
use zcash_client_backend::keys::UnifiedFullViewingKey;
use zcash_client_backend::wallet::WalletTransparentOutput;

use zcash_client_sqlite::chain::init::init_blockmeta_db;
use zcash_client_sqlite::wallet::init::init_wallet_db;
use zcash_client_sqlite::{chain::BlockMeta, FsBlockDb, ReceivedNoteId, WalletDb};

use zcash_primitives::legacy::TransparentAddress;
use zcash_primitives::sapling;
use zcash_primitives::transaction::components::{Amount, OutPoint};
use zcash_primitives::zip32::AccountId;

use secrecy::SecretVec;

mod chain;
pub use self::chain::*;

mod wallet;
pub use self::wallet::*;

use crate::{
    ZcashAccountBirthday, ZcashAccountId, ZcashAddressMetadata, ZcashAmount, ZcashBlockHash,
    ZcashBlockHeight, ZcashBlockMetadata, ZcashCommitmentTreeRoot, ZcashConsensusParameters,
    ZcashDecryptedTransaction, ZcashError, ZcashExtendedFullViewingKey, ZcashMemo,
    ZcashNullifierQuery, ZcashOutPoint, ZcashReceivedSaplingNote, ZcashResult,
    ZcashSaplingNullifier, ZcashScanRange, ZcashScannedBlock, ZcashSentTransaction,
    ZcashShieldedProtocol, ZcashTransaction, ZcashTransparentAddress, ZcashTxId,
    ZcashUnifiedAddress, ZcashUnifiedFullViewingKey, ZcashUnifiedSpendingKey, ZcashWalletSummary,
    ZcashWalletTransparentOutput,
};

pub struct TupleTargetAndAnchorHeight {
    pub target_height: Arc<ZcashBlockHeight>,
    pub anchor_height: Arc<ZcashBlockHeight>,
}

pub struct TupleAccountIdAndUnifiedSpendingKey {
    pub account_id: ZcashAccountId,
    pub unified_spending_key: Arc<ZcashUnifiedSpendingKey>,
}

pub struct TupleAccountIdAndSaplingNullifier {
    pub account_id: ZcashAccountId,
    pub sapling_nullifier: Arc<ZcashSaplingNullifier>,
}

pub struct TupleBlockHeightAndHash {
    pub block_height: Arc<ZcashBlockHeight>,
    pub block_hash: Arc<ZcashBlockHash>,
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

type UFVKMap = HashMap<ZcashAccountId, Arc<ZcashUnifiedFullViewingKey>>;

type TransparentReceiversMap = HashMap<String, Arc<ZcashAddressMetadata>>;

type TransparentBalancesMap = HashMap<String, Arc<ZcashAmount>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZcashReceivedNoteId(ReceivedNoteId);

impl From<ReceivedNoteId> for ZcashReceivedNoteId {
    fn from(e: ReceivedNoteId) -> Self {
        Self(e)
    }
}

impl From<ZcashReceivedNoteId> for ReceivedNoteId {
    fn from(inner: ZcashReceivedNoteId) -> Self {
        inner.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZcashNoteId(NoteId);

impl ZcashNoteId {
    pub fn new(txid: Arc<ZcashTxId>, zsp: ZcashShieldedProtocol, output_index: u16) -> Self {
        Self(NoteId::new((*txid).into(), zsp.into(), output_index))
    }
}

impl From<NoteId> for ZcashNoteId {
    fn from(e: NoteId) -> Self {
        Self(e)
    }
}

impl From<ZcashNoteId> for NoteId {
    fn from(inner: ZcashNoteId) -> Self {
        inner.0
    }
}

impl ZcashWalletDb {
    /// Construct a connection to the wallet database stored at the specified path.
    pub fn for_path(path: String, params: ZcashConsensusParameters) -> ZcashResult<Self> {
        Ok(ZcashWalletDb { path, params })
    }

    /// From wallet::init
    pub fn init(&self, seed: Vec<u8>) -> ZcashResult<()> {
        let mut db_data =
            WalletDb::for_path(&self.path, self.params).expect("Cannot access the DB!");
        let secvec = SecretVec::new(seed);

        init_wallet_db(&mut db_data, Some(secvec)).map_err(|e| ZcashError::Message {
            error: format!("Error while initializing data DB: {:?}", e),
        })
    }

    // ####################################
    // WalletRead implementation methods #
    // ####################################

    pub fn chain_height(&self) -> ZcashResult<Option<Arc<ZcashBlockHeight>>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .chain_height()
            .map(|x| x.map(From::from).map(Arc::new))
            .map_err(cast_err)
    }

    pub fn block_metadata(
        &self,
        height: Arc<ZcashBlockHeight>,
    ) -> ZcashResult<Option<Arc<ZcashBlockMetadata>>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .block_metadata((*height).into())
            .map(|x| x.map(From::from).map(Arc::new))
            .map_err(cast_err)
    }

    pub fn block_fully_scanned(&self) -> ZcashResult<Option<Arc<ZcashBlockMetadata>>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .block_fully_scanned()
            .map(|x| x.map(From::from).map(Arc::new))
            .map_err(cast_err)
    }

    pub fn block_max_scanned(&self) -> ZcashResult<Option<Arc<ZcashBlockMetadata>>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .block_max_scanned()
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

    pub fn get_target_and_anchor_heights(
        &self,
        min_confirmations: u32,
    ) -> ZcashResult<Option<TupleTargetAndAnchorHeight>> {
        let min = NonZeroU32::new(min_confirmations).unwrap();

        match WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_target_and_anchor_heights(min)
        {
            Ok(None) => Ok(None),
            Ok(Some((target_height, anchor_height))) => Ok(Some(TupleTargetAndAnchorHeight {
                target_height: Arc::new(target_height.into()),
                anchor_height: Arc::new(anchor_height.into()),
            })),
            Err(e) => Err(ZcashError::Message {
                error: format!("Err: {}", e),
            }),
        }
    }

    pub fn get_min_unspent_height(&self) -> ZcashResult<Option<Arc<ZcashBlockHeight>>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_min_unspent_height()
            .map(|x| x.map(From::from).map(Arc::new))
            .map_err(cast_err)
    }

    pub fn get_block_hash(
        &self,
        height: Arc<ZcashBlockHeight>,
    ) -> ZcashResult<Option<Arc<ZcashBlockHash>>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_block_hash((*height).into())
            .map(|x| x.map(From::from).map(Arc::new))
            .map_err(cast_err)
    }

    pub fn get_max_height_hash(&self) -> ZcashResult<Option<TupleBlockHeightAndHash>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_max_height_hash()
            .map(|x| {
                x.map(|(height, hash)| TupleBlockHeightAndHash {
                    block_height: Arc::new(height.into()),
                    block_hash: Arc::new(hash.into()),
                })
            })
            .map_err(cast_err)
    }

    pub fn get_tx_height(
        &self,
        txid: Arc<ZcashTxId>,
    ) -> ZcashResult<Option<Arc<ZcashBlockHeight>>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_tx_height((*txid).into())
            .map(|x| x.map(From::from).map(Arc::new))
            .map_err(cast_err)
    }

    pub fn get_wallet_birthday(&self) -> ZcashResult<Option<Arc<ZcashBlockHeight>>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_wallet_birthday()
            .map(|x| x.map(From::from).map(Arc::new))
            .map_err(cast_err)
    }

    pub fn get_account_birthday(
        &self,
        account: ZcashAccountId,
    ) -> ZcashResult<Arc<ZcashBlockHeight>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_account_birthday(account.into())
            .map(From::from)
            .map(Arc::new)
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

    pub fn get_unified_full_viewing_keys(&self) -> ZcashResult<UFVKMap> {
        let convert_hm = |hm: HashMap<AccountId, UnifiedFullViewingKey>| -> UFVKMap {
            hm.into_iter()
                .map(|(x, y)| (x.into(), Arc::new(y.into())))
                .collect()
        };

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_unified_full_viewing_keys()
            .map(convert_hm)
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

    pub fn is_valid_account_extfvk(
        &self,
        account: ZcashAccountId,
        extfvk: Arc<ZcashExtendedFullViewingKey>,
    ) -> ZcashResult<bool> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .is_valid_account_extfvk(account.into(), &(*extfvk).clone().into())
            .map_err(cast_err)
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

    pub fn get_memo(&self, id_note: Arc<ZcashNoteId>) -> ZcashResult<ZcashMemo> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_memo((*id_note).into())
            .map(|memo| memo.unwrap().into())
            .map_err(cast_err)
    }

    pub fn get_transaction(&self, txid: Arc<ZcashTxId>) -> ZcashResult<Arc<ZcashTransaction>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_transaction((*txid).into())
            .map(From::from)
            .map(Arc::new)
            .map_err(cast_err)
    }

    pub fn get_sapling_nullifiers(
        &self,
        query: ZcashNullifierQuery,
    ) -> ZcashResult<Vec<TupleAccountIdAndSaplingNullifier>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_sapling_nullifiers(query.into())
            .map(|x| {
                x.iter()
                    .map(|(aid, nf)| TupleAccountIdAndSaplingNullifier {
                        account_id: (*aid).into(),
                        sapling_nullifier: Arc::new(nf.into()),
                    })
                    .collect()
            })
            .map_err(cast_err)
    }

    pub fn get_spendable_sapling_notes(
        &self,
        account: ZcashAccountId,
        anchor_height: Arc<ZcashBlockHeight>,
        exclude: Vec<Arc<ZcashReceivedNoteId>>,
    ) -> ZcashResult<Vec<Arc<ZcashReceivedSaplingNote>>> {
        let exclude: Vec<ReceivedNoteId> = exclude.iter().map(|x| (**x).into()).collect();

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_spendable_sapling_notes(account.into(), (*anchor_height).into(), &exclude[..])
            .map(|notes| notes.into_iter().map(From::from).map(Arc::new).collect())
            .map_err(cast_err)
    }

    pub fn select_spendable_sapling_notes(
        &self,
        account: ZcashAccountId,
        target_value: Arc<ZcashAmount>,
        anchor_height: Arc<ZcashBlockHeight>,
        exclude: Vec<Arc<ZcashReceivedNoteId>>,
    ) -> ZcashResult<Vec<Arc<ZcashReceivedSaplingNote>>> {
        let exclude: Vec<ReceivedNoteId> = exclude.iter().map(|x| (**x).into()).collect();

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .select_spendable_sapling_notes(
                account.into(),
                (*target_value).into(),
                (*anchor_height).into(),
                &exclude[..],
            )
            .map(|notes| notes.into_iter().map(From::from).map(Arc::new).collect())
            .map_err(cast_err)
    }

    pub fn get_transparent_receivers(
        &self,
        aid: ZcashAccountId,
    ) -> ZcashResult<TransparentReceiversMap> {
        let convert_hm =
            |hm: HashMap<TransparentAddress, AddressMetadata>| -> TransparentReceiversMap {
                hm.into_iter()
                    .map(|(x, y)| (x.encode(&self.params), Arc::new(y.into())))
                    .collect()
            };

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_transparent_receivers(aid.into())
            .map(convert_hm)
            .map_err(cast_err)
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

    pub fn get_transparent_balances(
        &self,
        account: ZcashAccountId,
        max_height: Arc<ZcashBlockHeight>,
    ) -> ZcashResult<TransparentBalancesMap> {
        let convert_hm = |hm: HashMap<TransparentAddress, Amount>| -> TransparentBalancesMap {
            hm.into_iter()
                .map(|(x, y)| (x.encode(&self.params), Arc::new(y.into())))
                .collect()
        };

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_transparent_balances(account.into(), (*max_height).into())
            .map(convert_hm)
            .map_err(cast_err)
    }

    // ####################################
    // WalletWrite implementation methods #
    // ####################################

    pub fn create_account(
        &self,
        seed: Vec<u8>,
        birthday: Arc<ZcashAccountBirthday>,
    ) -> ZcashResult<TupleAccountIdAndUnifiedSpendingKey> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .create_account(&SecretVec::new(seed), (*birthday).clone().into())
            .map(|(aid, usk)| TupleAccountIdAndUnifiedSpendingKey {
                account_id: aid.into(),
                unified_spending_key: Arc::new(usk.into()),
            })
            .map_err(cast_err)
    }

    pub fn get_next_available_address(
        &self,
        account: ZcashAccountId,
    ) -> ZcashResult<Option<Arc<ZcashUnifiedAddress>>> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_next_available_address(account.into())
            .map(|addr| addr.map(From::from).map(Arc::new))
            .map_err(cast_err)
    }

    pub fn put_blocks(&self, blocks: Vec<Arc<ZcashScannedBlock>>) -> ZcashResult<()> {
        let blocks: Vec<ScannedBlock<sapling::Nullifier>> =
            blocks.iter().map(|x| (**x).clone().into()).collect();

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .put_blocks(blocks)
            .map_err(cast_err)
    }

    pub fn update_chain_tip(&self, tip_height: u32) -> ZcashResult<()> {
        let zheight = ZcashBlockHeight::new(tip_height).into();

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .update_chain_tip(zheight)
            .map_err(cast_err)
    }

    pub fn store_decrypted_tx(&self, d_tx: Arc<ZcashDecryptedTransaction>) -> ZcashResult<()> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .store_decrypted_tx((*d_tx).clone().into())
            .map_err(cast_err)
    }

    pub fn store_sent_tx(&self, sent_tx: ZcashSentTransaction) -> ZcashResult<()> {
        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .store_sent_tx(&(&sent_tx).into())
            .map_err(cast_err)
    }

    pub fn truncate_to_height(&self, block_height: u32) -> ZcashResult<()> {
        let zheight = ZcashBlockHeight::new(block_height).into();

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .truncate_to_height(zheight)
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

    // WalletCommitmentTrees implementation methods

    // with_sapling_tree_mut

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

    pub fn get_checkpoint_depth(&self, min_confirmations: u32) -> ZcashResult<u32> {
        let min_confirmations = NonZeroU32::new(min_confirmations).unwrap();

        WalletDb::for_path(&self.path, self.params)
            .expect("Cannot access the DB!")
            .get_checkpoint_depth(min_confirmations)
            .map(|x| x.try_into().unwrap())
            .map_err(|e| ZcashError::Message {
                error: format!("ShardTreeError: {:?}", e),
            })
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

    // init_blockmeta_db
    pub fn init(&self, blocks_dir: String) -> ZcashResult<()> {
        let mut db = FsBlockDb::for_path(blocks_dir).unwrap();

        match init_blockmeta_db(&mut db) {
            Ok(()) => Ok(()),
            e => Err(ZcashError::Message {
                error: format!("MigratorError: {:?}", e),
            }),
        }
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
