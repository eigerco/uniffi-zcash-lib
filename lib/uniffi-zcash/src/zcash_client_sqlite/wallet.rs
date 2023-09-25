use zcash_client_sqlite::wallet::init;
use zcash_client_sqlite::wallet::init::WalletMigrationError;

use crate::{ZcashConsensusParameters, ZcashError, ZcashResult, ZcashWalletDb};

use failure::format_err;

use std::sync::Arc;

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

    /// Wrapper for commitment tree invariant violations
    CommitmentTreeError { v: String },
}

impl From<WalletMigrationError> for ZcashWalletMigrationError {
    fn from(e: WalletMigrationError) -> Self {
        match e {
            WalletMigrationError::SeedRequired => ZcashWalletMigrationError::SeedRequired(),
            WalletMigrationError::CorruptedData(v) => {
                ZcashWalletMigrationError::CorruptedData { v }
            }
            WalletMigrationError::DbError(e) => ZcashWalletMigrationError::DbError {
                v: format!("DbError: {:?}", e),
            },
            WalletMigrationError::BalanceError(e) => ZcashWalletMigrationError::BalanceError {
                v: format!("BalanceError: {:?}", e),
            },
            WalletMigrationError::CommitmentTree(e) => {
                ZcashWalletMigrationError::CommitmentTreeError {
                    v: format!("CommitmentTreeError: {:?}", e),
                }
            }
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

pub struct ZcashWallet();

impl ZcashWallet {
    pub fn init_wallet_db(
        &self,
        zwdb: Arc<ZcashWalletDb>,
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
            ZcashConsensusParameters::TestNetwork => init::init_wallet_db(
                &mut zwdb.sup.test.lock().unwrap(),
                Some(SecretVec::new(seed)),
            )
            .map_err(|e| ZcashError::Message {
                error: format_err!("Error while initializing data DB: {:?}", e).to_string(),
            }),
        }
    }
}
