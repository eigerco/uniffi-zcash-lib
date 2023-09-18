use zcash_client_sqlite::wallet::init;
// init_accounts_table, init_blocks_table,
use zcash_client_sqlite::wallet::init::WalletMigrationError;

use crate::{ZcashConsensusParameters, ZcashError, ZcashResult, ZcashWalletDb};

use failure::format_err;

use secrecy::SecretVec;

#[derive(Debug)]
pub enum ZcashWalletMigrationError {
    /// The seed is required for the migration.
    SeedRequired(),

    /// Decoding of an existing value from its serialized form has failed.
    CorruptedData { v: String },

    /// Wrapper for rusqlite errors.
    // DbError(rusqlite::Error),
    DbError { v: String },

    /// Wrapper for amount balance violations
    // BalanceError(BalanceError),
    BalanceError { v: String },
}

impl From<WalletMigrationError> for ZcashWalletMigrationError {
    fn from(e: WalletMigrationError) -> Self {
        match e {
            WalletMigrationError::SeedRequired => ZcashWalletMigrationError::SeedRequired(),
            WalletMigrationError::CorruptedData(v) => {
                ZcashWalletMigrationError::CorruptedData { v }
            }
            WalletMigrationError::DbError { .. } => ZcashWalletMigrationError::DbError {
                v: "DbError".to_string(),
            },
            WalletMigrationError::BalanceError { .. } => ZcashWalletMigrationError::BalanceError {
                v: "BalanceError".to_string(),
            },
            WalletMigrationError::CommitmentTree(_) => todo!(),
        }
    }
}

// pub fn init_accounts_table<P: consensus::Parameters>(
//     wdb: &WalletDb<P>,
//     keys: &HashMap<AccountId, UnifiedFullViewingKey>,
// ) -> Result<(), SqliteClientError> {
// 	init::init_accounts_table(wdb, keys)
// }

// pub fn init_blocks_table<P>(
//     wdb: &WalletDb<P>,
//     height: ZcashBlockHeight,
//     hash: ZcashBlockHash,
//     time: u32,
//     sapling_tree: &[u8],
// ) -> Result<(), SqliteClientError> {
// 	init::init_blocks_table(wdb, height.into(), hash.into(), time, sapling_tree)
// }

pub fn init_wallet_db(
    zwdb: ZcashWalletDb,
    seed: Vec<u8>,
    params: ZcashConsensusParameters,
) -> ZcashResult<()> {
    match params {
        ZcashConsensusParameters::MainNetwork => init::init_wallet_db(
            &mut zwdb.sup.main.lock().unwrap(),
            Some(SecretVec::new(seed)),
        )
        .map_err(|e| ZcashError::Message {
            error: format_err!("Error while initializing data DB: {:?}", e).to_string(),
        }),
        // match  {
        //     Ok(utxo_id) => Ok(utxo_id.0),
        //     Err(e) => Err(ZcashError::Message {
        //         error: format!("Err: {}", e),
        //     }),
        // }
        ZcashConsensusParameters::TestNetwork => init::init_wallet_db(
            &mut zwdb.sup.test.lock().unwrap(),
            Some(SecretVec::new(seed)),
        )
        .map_err(|e| ZcashError::Message {
            error: format_err!("Error while initializing data DB: {:?}", e).to_string(),
        }), // match  {
            //     Ok(utxo_id) => Ok(utxo_id.0),
            //     Err(e) => Err(ZcashError::Message {
            //         error: format!("Err: {}", e),
            //     }),
            // }
    }
}
