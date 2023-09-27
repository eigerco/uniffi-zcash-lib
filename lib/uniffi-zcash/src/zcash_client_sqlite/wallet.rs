use std::{fmt, sync::Arc};

use failure::format_err;
use secrecy::SecretVec;

use zcash_client_sqlite::wallet::init;
use zcash_client_sqlite::wallet::init::WalletMigrationError;
use zcash_client_sqlite::WalletDb;

use crate::{ZcashConsensusParameters, ZcashError, ZcashResult, ZcashWalletDb};

#[derive(Debug, thiserror::Error)]
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

impl fmt::Display for ZcashWalletMigrationError {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZcashWalletMigrationError::SeedRequired() => write!(f, "SeedRequired"),
            ZcashWalletMigrationError::CorruptedData { v } => write!(f, "CorruptedData: {}", v),
            ZcashWalletMigrationError::DbError { v } => write!(f, "DbError: {}", v),
            ZcashWalletMigrationError::BalanceError { v } => write!(f, "BalanceError: {}", v),
            ZcashWalletMigrationError::CommitmentTreeError { v } => {
                write!(f, "CommitmentTreeError: {}", v)
            }
        }
    }
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

pub struct ZcashWallet();

impl ZcashWallet {
    pub fn init_wallet_db(
        &self,
        zwdb: Arc<ZcashWalletDb>,
        seed: Vec<u8>,
        params: ZcashConsensusParameters,
    ) -> ZcashResult<()> {
        let mut db_data = WalletDb::for_path(&zwdb.path, params).unwrap();
        let secvec = SecretVec::new(seed);

        init::init_wallet_db(&mut db_data, Some(secvec)).map_err(|e| ZcashError::Message {
            error: format_err!("Error while initializing data DB: {:?}", e).to_string(),
        })
    }
}
