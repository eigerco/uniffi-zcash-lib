use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::num::NonZeroU32;
use std::panic;
use std::path::Path;
use std::ptr;

use failure::format_err;
use jni::objects::{JObject, JValue};
use jni::{
    objects::{JClass, JString},
    sys::{jboolean, jbyteArray, jint, jlong, jobject, jobjectArray, jstring, JNI_FALSE, JNI_TRUE},
    JNIEnv,
};

use schemer::MigratorError;
use secrecy::{ExposeSecret, SecretVec};
use tracing::{debug, error};
use tracing_subscriber::prelude::*;
use tracing_subscriber::reload;

use crate::native_utils as utils;

use utils::exception::unwrap_exc_or;

// use zcash_address::{ToAddress, ZcashAddress};

use crate::{
    ZcashAccountId,
    ZcashAmount,
    ZcashBlockHash,
    // primitives
    ZcashBlockHeight,
    ZcashBlockMeta,
    ZcashBranchId,
    // zcash_client_sqlite
    ZcashChain, // init_blockmeta_db
    ZcashConsensusParameters,
    ZcashConsensusParameters::{MainNetwork, TestNetwork}, // consensus
    ZcashDecodingError,                                   // keys
    ZcashDiversifierIndex,
    ZcashDustOutputPolicy, // fees
    ZcashKeysEra,
    ZcashLocalTxProver,
    ZcashMemoBytes,
    ZcashOutPoint,
    ZcashOvkPolicy,
    ZcashPayment,
    // encoding::AddressCodec, // NOT USED
    ZcashRecipientAddress,
    ZcashScript,
    ZcashTransaction,
    ZcashTransactionRequest, // zip321
    ZcashTransparentAddress,
    ZcashTxId,
    ZcashTxOut,
    ZcashUnifiedAddress, // address
    ZcashUnifiedFullViewingKey,
    ZcashUnifiedSpendingKey,
    ZcashWalletTransparentOutput, // wallet
};

// use crate::chain::ZcashBlockMeta;

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
//     wallet::init::{init_accounts_table, init_blocks_table, init_wallet_db, WalletMigrationError},
//     FsBlockDb, WalletDb,
// };

// use zcash_primitives::{
//     consensus::{, Network, Parameters},
//     memo::{Memo,},
//     merkle_tree::HashSer, // to do
//     sapling, // ?
//     transaction::{
//         components::{amount::NonNegativeAmount, },
//     },
// };

// fn wallet_db<P: Parameters>(
//     params: P,
//     db_data: String,
// ) -> Result<WalletDb<rusqlite::Connection, P>, failure::Error> {
//     WalletDb::for_path(db_data, params)
//         .map_err(|e| format_err!("Error opening wallet database connection: {}", e))
// }

// needed ?
// fn block_db(fsblockdb_root: String) -> Result<FsBlockDb, failure::Error> {
//     FsBlockDb::for_path(fsblockdb_root)
//         .map_err(|e| format_err!("Error opening block source database connection: {:?}", e))
// }

#[cfg(debug_assertions)]
fn print_debug_state() {
    debug!("WARNING! Debugging enabled! This will likely slow things down 10X!");
}

#[cfg(not(debug_assertions))]
fn print_debug_state() {
    debug!("Release enabled (congrats, this is NOT a debug build).");
}

//subssitute with USK constructor
// fn encode_usk(
//     // env: &JNIEnv<'_>,
//     seed: Vec<u8>,
//     aid: ZcashAccountId,
//     usk: ZcashUnifiedSpendingKey,
// ) -> Result<jobject, failure::Error> {
//     let encoded = SecretVec::new(usk.to_bytes(ZcashKeysEra::Orchard));
//     let bytes = encoded.expose_secret().to_vec();
//     let output = env.new_object(
//         "cash/z/ecc/android/sdk/internal/model/JniUnifiedSpendingKey",
//         "(I[B)V",
//         &[
//             JValue::Int(u32::from(account) as i32),
//             JValue::Object(unsafe { JObject::from_raw(bytes) }),
//         ],
//     )?;
//     Ok(output.into_raw())
//     ZcashUnifiedSpendingKey::from_seed(params, seed, aid)
// }

// not needed
// fn decode_usk(env: &JNIEnv<'_>, usk: jbyteArray) -> Result<UnifiedSpendingKey, failure::Error> {
//     let usk_bytes = SecretVec::new(env.convert_byte_array(usk).unwrap());

//     // The remainder of the function is safe.
//     UnifiedSpendingKey::from_bytes(Era::Orchard, usk_bytes.expose_secret()).map_err(|e| match e {
//         DecodingError::EraMismatch(era) => format_err!(
//             "Spending key was from era {:?}, but {:?} was expected.",
//             era,
//             Era::Orchard
//         ),
//         e => format_err!(
//             "An error occurred decoding the provided unified spending key: {:?}",
//             e
//         ),
//     })
// }

pub fn init_on_load() {
    let trc_info_level = tracing_subscriber::filter::LevelFilter::INFO;
    // Set up the Android tracing layer.
    #[cfg(target_os = "android")]
    let android_layer = paranoid_android::layer("cash.z.rust.logs")
        .with_ansi(false)
        .with_filter(trc_info_level);

    // Generate Android trace events from `tracing` spans.
    let (trace_event_layer, reload_handle) = reload::Layer::new(utils::trace::Layer::new(None));

    // Install the `tracing` subscriber.
    let registry = tracing_subscriber::registry();

    #[cfg(target_os = "android")]
    let registry = registry.with(android_layer);
    registry.with(trace_event_layer).init();

    // Log panics instead of writing them to stderr.
    log_panics::init();

    // Load optional NDK APIs. We do this via a reload so that we can capture any errors
    // that occur while trying to dynamically load the NDK.
    if let Err(e) = reload_handle.modify(|layer| match utils::target_ndk::load() {
        Ok(api) => *layer = utils::trace::Layer::new(Some(api)),
        Err(e) => error!("Could not open NDK library or load symbols: {}", e),
    }) {
        error!("Failed to reload tracing subscriber with NDK APIs: {}", e);
    }

    // NO RAYON here
    // Manually build the Rayon thread pool, so we can name the threads.
    // rayon::ThreadPoolBuilder::new()
    //     .thread_name(|i| format!("zc-rayon-{}", i))
    //     .build_global()
    //     .expect("Only initialized once");

    debug!("Rust backend has been initialized successfully");
    print_debug_state();
}

// NOTE: need to get WalletDB translated first

// pub fn create_account(
//     // db_data: JString<'_>,
//     seed: Vec<u8>,
//     params: ZcashConsensusParameters,
// ) -> ZcashUnifiedSpendingKey {
//     // not needed because we may pass the full param instead of the id
//     // let network = parse_network(network_id)?;

//     let mut db_data = wallet_db(&env, network, db_data)?;

//     // the seed is passed from outside
//     // let seed = SecretVec::new(env.convert_byte_array(seed).unwrap());

//     // NOTE: is it needed to store the account created?
//     // let (account, usk) = db_data
//     //     .create_account(&seed)
//     //     .map_err(|e| format_err!("Error while initializing accounts: {}", e))?;

//     // encode_usk(&env, account, usk)
//     ZcashUnifiedSpendingKey::from_seed(params, seed, ZcashAccountId{id: 1})
// }

// pub fn get_balance(db_data: String, aid: u32, params: ZcashConsensusParameters) -> u32 {
//     // let network = parse_network(network_id as u32)?;
//     let db_data = wallet_db(params, db_data)?;
//     let account = ZcashAccountId::from(aid);

//     // We query the unverified balance including unmined transactions. Shielded notes
//     // in unmined transactions are never spendable, but this ensures that the balance
//     // reported to users does not drop temporarily in a way that they don't expect.
//     // `getVerifiedBalance` requires `ANCHOR_OFFSET` confirmations, which means it
//     // always shows a spendable balance.
//     let min_confirmations = NonZeroU32::new(1).unwrap();

//     (&db_data)
//         .get_target_and_anchor_heights(min_confirmations)
//         .map_err(|e| format_err!("Error while fetching anchor height: {}", e))
//         .and_then(|opt_anchor| {
//             opt_anchor
//                 .map(|(_, a)| a + 1)
//                 .ok_or(format_err!("Anchor height not available; scan required."))
//         })
//         .and_then(|anchor| {
//             (&db_data)
//                 .get_balance_at(account, anchor)
//                 .map_err(|e| format_err!("Error while fetching verified balance: {}", e))
//         })
//         .map(|amount| amount.into())
// }

// put_utxo

// init_data_db

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
