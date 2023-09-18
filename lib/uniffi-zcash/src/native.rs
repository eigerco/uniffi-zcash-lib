// INSTRUCTIONS
// this file is supposed to contain only UniFFI-compatible structs (Zcash-Structs)
// so, no imports directly from librustzcash
// Afterwards, this should serve as SDK for mobile.
// at the moment there are also external libraries - they will be gone too.

// use std::collections::HashMap;
// use std::convert::{TryFrom, TryInto};
// use std::panic;
// use std::path::Path;
// use std::ptr;

use std::num::NonZeroU32;

use failure::format_err;

use std::sync::Arc;

// NOTE shouldn't be needed because we will be using Kotlin instead
// use jni::objects::{JObject, JValue};
// use jni::{
//     // objects::{JClass, JString},
//     sys::{jboolean, jbyteArray, jint, jlong, jobject, jobjectArray, jstring, JNI_FALSE, JNI_TRUE},
//     JNIEnv,
// };

// use schemer::MigratorError;
// use secrecy::{ExposeSecret, SecretVec};

use tracing::{debug, error};
use tracing_subscriber::prelude::*;
use tracing_subscriber::reload;

// use zcash_address::{ToAddress, ZcashAddress};

use crate::native_utils as utils;
use crate::zcash_client_sqlite::init_wallet_db;
use crate::{
    scan_cached_blocks,
    ZcashAccountId,
    ZcashAmount,
    // ZcashBlockHash,
    // primitives
    ZcashBlockHeight,
    ZcashBlockMeta,
    // ZcashBranchId,
    // zcash_client_sqlite
    // ZcashChain, // init_blockmeta_db
    ZcashConsensusParameters,
    // ZcashConsensusParameters::{MainNetwork, TestNetwork}, // consensus
    // ZcashDecodingError,                                   // keys
    // ZcashDiversifierIndex,
    // ZcashDustOutputPolicy, // fees
    ZcashError,
    ZcashFsBlockDb,
    // ZcashKeysEra,
    // ZcashLocalTxProver,
    ZcashMemo,
    // ZcashMemoBytes,
    ZcashNonNegativeAmount,
    ZcashNoteId,
    ZcashOutPoint,
    // ZcashOvkPolicy,
    // ZcashPayment,
    // encoding::AddressCodec, // NOT USED
    // ZcashRecipientAddress,
    ZcashResult,
    ZcashScript,
    ZcashShieldedProtocol,
    // ZcashTransaction,
    // ZcashTransactionRequest, // zip321
    ZcashTransparentAddress,
    ZcashTxId,
    ZcashTxOut,
    // ZcashUnifiedAddress, // address
    // ZcashUnifiedFullViewingKey,
    ZcashUnifiedSpendingKey,
    ZcashWalletDb,
    ZcashWalletTransparentOutput, // wallet
};

// use zcash_client_backend::data_api::{
//     chain::CommitmentTreeRoot,
//     wallet::{
//         decrypt_and_store_transaction, input_selection::GreedyInputSelector,
//         shield_transparent_funds, spend,
//     },
//     WalletCommitmentTrees,
//     scanning::{ScanPriority, ScanRange},
//     ShieldedProtocol,
// };

// use zcash_client_sqlite::chain::init::init_blockmeta_db;
// use zcash_client_sqlite::wallet::init::{init_accounts_table, init_blocks_table}

// use zcash_primitives::merkle_tree::HashSer

fn wallet_db(params: ZcashConsensusParameters, db_data: String) -> ZcashResult<ZcashWalletDb> {
    ZcashWalletDb::for_path(db_data, params).map_err(|e| ZcashError::Message {
        error: format_err!("Error opening wallet database connection: {}", e).to_string(),
    })
}

fn block_db(fsblockdb_root: String) -> ZcashResult<ZcashFsBlockDb> {
    ZcashFsBlockDb::for_path(fsblockdb_root).map_err(|e| ZcashError::Message {
        error: format_err!("Error opening block source database connection: {:?}", e).to_string(),
    })
}

#[cfg(debug_assertions)]
fn print_debug_state() {
    debug!("WARNING! Debugging enabled! This will likely slow things down 10X!");
}

#[cfg(not(debug_assertions))]
fn print_debug_state() {
    debug!("Release enabled (congrats, this is NOT a debug build).");
}

// NOTE subssitute with USK constructor
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

// NOTE not needed
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
    let _trc_info_level = tracing_subscriber::filter::LevelFilter::INFO;
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

pub fn create_account(
    db_data: String,
    seed: Vec<u8>,
    params: ZcashConsensusParameters,
) -> ZcashUnifiedSpendingKey {
    // not needed because we may pass the full param instead of the id
    // let network = parse_network(network_id)?;

    let _db_data = wallet_db(params, db_data).unwrap();
    let account = ZcashAccountId { id: 55 };

    // the seed is passed from outside
    // let seed = SecretVec::new(env.convert_byte_array(seed).unwrap());

    // NOTE: is it needed to store the account created?
    // let (account, usk) = db_data
    //     .create_account(&seed)
    //     .map_err(|e| format_err!("Error while initializing accounts: {}", e))?;

    // encode_usk(&env, account, usk)
    ZcashUnifiedSpendingKey::from_seed(params, seed, account).unwrap()
}

// DEPR_NOTE get_balance_at was deprecated in newer version, instead get_wallet_summary was used
pub fn get_balance(
    db_data: String,
    aid: u32,
    params: ZcashConsensusParameters,
) -> ZcashResult<ZcashNonNegativeAmount> {
    // let network = parse_network(network_id as u32)?;
    let db_data = wallet_db(params, db_data).unwrap();
    let account = ZcashAccountId { id: aid };

    // We query the unverified balance including unmined transactions. Shielded notes
    // in unmined transactions are never spendable, but this ensures that the balance
    // reported to users does not drop temporarily in a way that they don't expect.
    // `getVerifiedBalance` requires `ANCHOR_OFFSET` confirmations, which means it
    // always shows a spendable balance.
    let min_confirmations = NonZeroU32::new(1).unwrap();

    // (&db_data)
    //     .get_target_and_anchor_heights(min_confirmations)
    //     .map_err(|e| format_err!("Error while fetching anchor height: {}", e))
    //     .and_then(|opt_anchor| {
    //         opt_anchor
    //             .map(|(_, a)| a.value() + 1)
    //             .ok_or(format_err!("Anchor height not available; scan required."))
    //     })
    //     .and_then(|anchor| {
    Ok((*db_data
        // get_balance_at
        .get_wallet_summary(min_confirmations.into())
        .unwrap() //Result
        .unwrap() //Option
        .account_balances()
        .get(&account)
        .unwrap())
    .total())
    // })
    // .map_err(|e| ZcashError::Message { error: format!("Err: {}", e) })
}

pub fn get_latest_height(fsblockdb_root: String) -> ZcashResult<i64> {
    let block_db = block_db(fsblockdb_root)?;

    match block_db.get_max_cached_height() {
        Ok(Some(block_height)) => Ok(i64::from(block_height.value())),
        // Use -1 to return null across the FFI.
        Ok(None) => Ok(-1),
        Err(e) => Err(ZcashError::Message {
            error: format!("Failed to read block metadata from FsBlockDb: {}", e),
        }),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn put_utxo(
    db_data: String,
    address: String,
    params: ZcashConsensusParameters,
    txid_bytes: Vec<u8>,
    script_bytes: Vec<u8>,
    index: u32,
    value: i64,
    height: u32,
) -> ZcashResult<bool> {
    let mut txid = [0u8; 32];
    txid.copy_from_slice(&txid_bytes[..]);

    let mut script = [0u8; 512];
    script.copy_from_slice(&script_bytes[..]);

    let script_pubkey = ZcashScript::from_bytes(&script);
    let mut db_data = wallet_db(params, db_data)?;

    // just making sure the process doesn't fail, that's why the underscore
    let _address = ZcashTransparentAddress::decode(params, &address).unwrap();

    let output = ZcashWalletTransparentOutput::from_parts(
        ZcashOutPoint::new(&txid, index).unwrap().into(),
        ZcashTxOut::new(
            ZcashAmount::new(value).unwrap().into(),
            script_pubkey.unwrap().into(),
        )
        .into(),
        ZcashBlockHeight::new(height).into(),
    )
    .unwrap();
    //"UTXO is not P2PKH or P2SH"

    debug!("Storing UTXO in db_data");

    match db_data.put_received_transparent_utxo(&output) {
        Ok(_) => Ok(true),
        Err(e) => Err(ZcashError::Message {
            error: format!("Error while inserting UTXO: {}", e),
        }),
    }
}

pub fn scan_blocks(
    db_cache: String,
    db_data: String,
    from_height: u32,
    limit: u32,
    params: ZcashConsensusParameters,
) -> ZcashResult<bool> {
    let db_cache = block_db(db_cache)?;
    let db_data = wallet_db(params, db_data)?;
    let from_height = ZcashBlockHeight::new(from_height);

    match scan_cached_blocks(params, db_cache, db_data, from_height, limit) {
        Ok(()) => Ok(true),
        Err(e) => Err(ZcashError::Message {
            error: format!(
                "Rust error while scanning blocks (limit {:?}): {:?}",
                limit, e
            ),
        }),
    }
}

pub fn get_memo_as_utf8(
    db_data: String,
    txid_bytes: Vec<u8>,
    output_index: u32,
    params: ZcashConsensusParameters,
) -> ZcashResult<String> {
    let db_data = wallet_db(params, db_data)?;

    // let mut txid = [0u8; 32];
    // txid.copy_from_slice(&txid_bytes[..]);

    // NOTE probably in a new version this is needed
    let txid = ZcashTxId::from_bytes(&txid_bytes[..])?;

    db_data
        .get_memo(ZcashNoteId::new(
            txid,
            ZcashShieldedProtocol::Sapling,
            output_index as u16,
        ))
        .map_err(|e| format_err!("An error occurred retrieving the memo, {}", e))
        .and_then(|memo| match memo {
            ZcashMemo::Empty => Ok("".to_string()),
            ZcashMemo::Text{v} => Ok(v),
            ZcashMemo::Future { .. } | ZcashMemo::Arbitrary { .. } => todo!()
            // None => Err(format_err!("Memo not available")),
            // _ => Err(format_err!("This memo does not contain UTF-8 text")),
        })
        .map_err(|e| ZcashError::Message {
            error: format_err!("some err {}", e).to_string(),
        })

    // NOTE into_raw was here
}

pub fn init_data_db(
    db_path: String,
    seed: Vec<u8>,
    params: ZcashConsensusParameters,
) -> ZcashResult<u8> {
    let db_data = wallet_db(params, db_path)?;

    init_wallet_db(db_data, seed, params).map(|_| 0u8)

    // match  {
    //     Ok(()) => Ok(0),
    //     Err(MigratorError::Migration { error, .. })
    //         if matches!(error, WalletMigrationError::SeedRequired) => { Ok(1) }
    //     Err(e) => Err(format_err!("Error while initializing data DB: {}", e)),
    // }
}

pub fn rewind_to_height(
    db_data: String,
    height: u32,
    params: ZcashConsensusParameters,
) -> ZcashResult<u8> {
    let mut db_data = wallet_db(params, db_data)?;

    let z_height = ZcashBlockHeight::new(height);

    db_data
        .truncate_to_height(z_height)
        .map(|_| 1u8)
        .map_err(|e| ZcashError::Message {
            error: format_err!("Error while rewinding data DB to height {}: {}", height, e)
                .to_string(),
        })
}

pub fn rewind_block_metadata_to_height(
    db_data: String,
    height: u32,
    params: ZcashConsensusParameters,
) -> ZcashResult<u8> {
    let mut db_data = wallet_db(params, db_data)?;
    let z_height = ZcashBlockHeight::new(height);

    db_data
        .truncate_to_height(z_height)
        .map(|_| 1u8)
        .map_err(|e| ZcashError::Message {
            error: format_err!(
                "Error while rewinding block metadata DB to height {}: {}",
                height,
                e
            )
            .to_string(),
        })
}

pub fn update_chain_tip(
    db_data: String,
    height: u32,
    params: ZcashConsensusParameters,
) -> ZcashResult<u8> {
    let mut db_data = wallet_db(params, db_data)?;

    let z_height = ZcashBlockHeight::new(height);

    db_data
        .update_chain_tip(z_height)
        .map(|_| 1u8)
        .map_err(|e| ZcashError::Message {
            error: format_err!("Error while rewinding data DB to height {}: {}", height, e)
                .to_string(),
        })
}

// NOTE unused so far
// fn encode_blockmeta(meta: Arc<ZcashBlockMeta>) -> Result<Vec<String>, failure::Error> {
//     let u32_arr_to_str = |arr: [u8;32]| -> String {
//         arr.iter().map(|&id| id.to_string() + ",").collect::<String>()
//     };

//     Ok(vec![
//         u32::from(meta.0.height).to_string(),
//         u32_arr_to_str(meta.0.block_hash.0),
//         meta.0.block_time.to_string(),
//         meta.0.sapling_outputs_count.to_string(),
//         meta.0.orchard_actions_count.to_string(),
//     ])
// }

// fn decode_blockmeta(obj: JObject<'_>) -> Result<BlockMeta, failure::Error> {
//     let long_as_u32 = |name| -> Result<u32, failure::Error> {
//         Ok(u32::try_from(env.get_field(obj, name, "J")?.j()?)?)
//     };

//     fn byte_array<const N: usize>(
//         env: &JNIEnv<'_>,
//         obj: JObject<'_>,
//         name: &str,
//     ) -> Result<[u8; N], failure::Error> {
//         let field = env.get_field(obj, name, "[B")?.l()?.into_raw();
//         Ok(env.convert_byte_array(field)?[..].try_into()?)
//     }

//     Ok(ZcashBlockMeta {
//         height: BlockHeight::from_u32(long_as_u32("height")?),
//         block_hash: BlockHash(byte_array(env, obj, "hash")?),
//         block_time: long_as_u32("time")?,
//         sapling_outputs_count: long_as_u32("saplingOutputsCount")?,
//         orchard_actions_count: long_as_u32("orchardOutputsCount")?,
//     })
// }

// pub fn write_block_metadata(fsblockdb_root: String, block_meta: Vec<String>) -> ZcashResult<bool> {

//     let block_db = block_db(fsblockdb_root)?;

//     // let block_meta = {
//     //     // let count = env.get_array_length(block_meta).unwrap();
//     //     (0..count)
//     //         .map(|i| {
//     //             env.get_object_array_element(block_meta, i)
//     //                 .map_err(|e| e.into())
//     //                 .and_then(|jobj| decode_blockmeta(&env, jobj))
//     //         })
//     //         .collect::<Result<Vec<_>, _>>()?
//     // };

//     match block_db.write_block_metadata(&block_meta) {
//         Ok(()) => Ok(true),
//         Err(e) => Err(format_err!(
//             "Failed to write block metadata to FsBlockDb: {:?}",
//             e
//         )),
//     }
// }

pub fn find_block_metadata(
    fsblockdb_root: String,
    height: u32,
) -> ZcashResult<Option<Arc<ZcashBlockMeta>>> {
    let block_db = block_db(fsblockdb_root)?;
    let height = ZcashBlockHeight::new(height);

    block_db
        .find_block(height)
        .map_err(|e| ZcashError::Message {
            error: format_err!("Failed to read block metadata from FsBlockDb: {:?}", e).to_string(),
        })
}

// create_to_address

// init_block_meta_db

// init_blocks_table

// shield_to_address

// branch_id_for_height

// find_block_metadata

// get_current_address

// suggest_scan_ranges

// get_verified_balance

// is_valid_spending_key

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
