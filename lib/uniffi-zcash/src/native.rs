use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::num::NonZeroU32;
use std::panic;
use std::path::Path;
use std::ptr;

// use failure::format_err;
// use jni::objects::{JObject, JValue};
// use jni::{
//     objects::{JClass, JString},
//     sys::{jboolean, jbyteArray, jint, jlong, jobject, jobjectArray, jstring, JNI_FALSE, JNI_TRUE},
//     JNIEnv,
// };

// use schemer::MigratorError;
// use secrecy::{ExposeSecret, SecretVec};
// use tracing::{debug, error};
// use tracing_subscriber::prelude::*;
// use tracing_subscriber::reload;

// use zcash_address::{ToAddress, ZcashAddress};

use crate::{
    // encoding::AddressCodec, // NOT USED
    ZcashRecipientAddress, ZcashUnifiedAddress, // zcash_client_backend::address
    ZcashDustOutputPolicy, // zcash_client_backend::fees
    ZcashKeysEra, ZcashUnifiedFullViewingKey, ZcashUnifiedSpendingKey, ZcashDecodingError, // keys
    ZcashOvkPolicy, ZcashWalletTransparentOutput // wallet
    // Payment, TransactionRequest // zip321
};


// use zcash_client_backend::data_api::{
//     chain::{scan_cached_blocks, CommitmentTreeRoot},
//     wallet::{
//         decrypt_and_store_transaction, input_selection::GreedyInputSelector,
//         shield_transparent_funds, spend,
//     },
//     WalletCommitmentTrees, WalletRead, WalletWrite,
//     scanning::{ScanPriority, ScanRange},
//     NoteId, ShieldedProtocol,
// };

// use zcash_client_sqlite::chain::init::init_blockmeta_db;

// use zcash_client_sqlite::{
//     chain::BlockMeta,
//     wallet::init::{init_accounts_table, init_blocks_table, init_wallet_db, WalletMigrationError},
//     FsBlockDb, WalletDb,
// };

// use zcash_primitives::consensus::Network::{MainNetwork, TestNetwork};
// // use zcash_primitives::consensus::parameters::ZcashConsensusParameters::{MainNetwork, TestNetwork};

// use zcash_primitives::{
//     block::BlockHash, // to do
//     consensus::{ZcashBlockHeight, ZcashBranchId, Network, Parameters},
//     legacy::{ZcashScript, ZcashTransparentAddress},
//     memo::{Memo, ZcashMemoBytes},
//     merkle_tree::HashSer, // to do
//     sapling, // ?
//     transaction::{
//         components::{amount::NonNegativeAmount, Amount, OutPoint, TxOut},
//         Transaction, TxId,
//     },
//     zip32::{ZcashAccountId, ZcashDiversifierIndex},
// };
// use zcash_proofs::prover::LocalTxProver;

// fn print_debug_state() {
//     debug!("WARNING! Debugging enabled! This will likely slow things down 10X!");
// }

// use crate::utils::exception::unwrap_exc_or;

// put_utxo

// get_balance

// init_data_db

// init_on_load

// scan_blocks

// create_account

// get_memo_as_utf8

// rewind_to_height

// update_chain_tip

// create_to_address

// get_latest_height

// init_block_meta_db

// init_blocks_table

// shield_to_address

// branch_id_for_height

// find_block_metadata

// get_current_address

// suggest_scan_ranges

// get_verified_balance

// is_valid_spending_key

// write_block_metadata

// is_valid_unified_address

// get_nearest_rewind_height

// is_valid_shielded_address
// put_sapling_subtree_roots
// list_transparent_receivers
// init_accounts_table_with_keys
// is_valid_transparent_address
// decrypt_and_store_transaction
// get_total_transparent_balance
// rewind_block_metadata_to_height
// get_verified_transparent_balance
// get_sapling_receiver_for_unified_address
// get_transparent_receiver_for_unified_address

