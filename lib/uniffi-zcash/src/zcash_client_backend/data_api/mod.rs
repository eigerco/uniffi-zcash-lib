mod chain;
pub use self::chain::*;

mod wallet;
pub use self::wallet::*;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;

use crate::{ZcashAccountId, ZcashBlockHeight, ZcashNonNegativeAmount};

use zcash_client_backend::data_api::{
    AccountBalance, Balance, DecryptedTransaction, Ratio, ShieldedProtocol, WalletSummary,
};
use zcash_primitives::transaction::components::amount::NonNegativeAmount;
use zcash_primitives::zip32::AccountId;

pub enum ZcashShieldedProtocol {
    // Orchard
    Sapling,
}

impl From<ZcashShieldedProtocol> for ShieldedProtocol {
    fn from(e: ZcashShieldedProtocol) -> Self {
        match e {
            ZcashShieldedProtocol::Sapling => ShieldedProtocol::Sapling,
        }
    }
}

impl From<ShieldedProtocol> for ZcashShieldedProtocol {
    fn from(e: ShieldedProtocol) -> Self {
        match e {
            ShieldedProtocol::Sapling => ZcashShieldedProtocol::Sapling,
        }
    }
}

pub struct ZcashDecryptedTransaction(DecryptedTransaction<'static>);

impl From<DecryptedTransaction<'static>> for ZcashDecryptedTransaction {
    fn from(e: DecryptedTransaction<'static>) -> Self {
        Self(e)
    }
}

impl<'a> From<ZcashDecryptedTransaction> for DecryptedTransaction<'a> {
    fn from(inner: ZcashDecryptedTransaction) -> Self {
        inner.0
    }
}

pub struct ZcashWalletSummary(WalletSummary);

impl ZcashWalletSummary {
    pub fn new(
        account_balances: HashMap<ZcashAccountId, ZcashAccountBalance>,
        chain_tip_height: Arc<ZcashBlockHeight>,
        fully_scanned_height: Arc<ZcashBlockHeight>,
        scan_progress: Option<ZcashRatio>,
    ) -> Self {
        Self(WalletSummary::new(
            account_balances
                .into_iter()
                .map(|(x, y)| (x.into(), y.into()))
                .collect::<BTreeMap<AccountId, AccountBalance>>(),
            (*chain_tip_height).into(),
            (*fully_scanned_height).into(),
            scan_progress.map(From::from),
        ))
    }

    /// Returns the balances of accounts in the wallet, keyed by account ID.
    pub fn account_balances(&self) -> HashMap<ZcashAccountId, ZcashAccountBalance> {
        self.0
            .account_balances()
            .iter()
            .map(|(&x, &y)| (x.into(), y.into()))
            .collect::<HashMap<ZcashAccountId, ZcashAccountBalance>>()
    }

    /// Returns the height of the current chain tip.
    pub fn chain_tip_height(&self) -> ZcashBlockHeight {
        self.0.chain_tip_height().into()
    }

    /// Returns the height below which all blocks have been scanned by the wallet, ignoring blocks
    /// below the wallet birthday.
    pub fn fully_scanned_height(&self) -> ZcashBlockHeight {
        self.0.fully_scanned_height().into()
    }

    /// Returns the progress of scanning shielded outputs, in terms of the ratio between notes
    /// scanned and the total number of notes added to the chain since the wallet birthday.
    ///
    /// This ratio should only be used to compute progress percentages, and the numerator and
    /// denominator should not be treated as authoritative note counts. Returns `None` if the
    /// wallet is unable to determine the size of the note commitment tree.
    pub fn scan_progress(&self) -> Option<ZcashRatio> {
        self.0.scan_progress().map(From::from)
    }

    /// Returns whether or not wallet scanning is complete.
    pub fn is_synced(&self) -> bool {
        self.0.is_synced()
    }
}

impl From<WalletSummary> for ZcashWalletSummary {
    fn from(e: WalletSummary) -> Self {
        ZcashWalletSummary(e)
    }
}

impl From<ZcashWalletSummary> for WalletSummary {
    fn from(inner: ZcashWalletSummary) -> Self {
        inner.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZcashRatio(Ratio<u64>);

impl ZcashRatio {
    /// Constructs a new Ratio from a numerator and a denominator.
    pub fn new(numerator: u64, denominator: u64) -> Self {
        Self(Ratio::new(numerator, denominator))
    }

    /// Returns the numerator of the ratio.
    pub fn numerator(&self) -> &u64 {
        self.0.numerator()
    }

    /// Returns the denominator of the ratio.
    pub fn denominator(&self) -> &u64 {
        self.0.denominator()
    }
}

impl From<Ratio<u64>> for ZcashRatio {
    fn from(e: Ratio<u64>) -> Self {
        ZcashRatio(e)
    }
}

impl From<ZcashRatio> for Ratio<u64> {
    fn from(inner: ZcashRatio) -> Self {
        inner.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZcashAccountBalance(AccountBalance);

impl ZcashAccountBalance {
    /// The [`Balance`] value having zero values for all its fields.
    pub const ZERO: Self = Self(AccountBalance {
        sapling_balance: Balance::ZERO,
        unshielded: NonNegativeAmount::ZERO,
    });

    /// Returns the total value of funds belonging to the account.
    pub fn total(&self) -> ZcashNonNegativeAmount {
        self.0.total().into()
    }

    /// Returns the total value of funds belonging to the account.
    pub fn sapling_spendable_value(&self) -> ZcashNonNegativeAmount {
        self.0.sapling_balance.spendable_value.into()
    }
}

impl From<ZcashAccountBalance> for AccountBalance {
    fn from(inner: ZcashAccountBalance) -> Self {
        inner.0
    }
}

impl From<AccountBalance> for ZcashAccountBalance {
    fn from(e: AccountBalance) -> Self {
        ZcashAccountBalance(e)
    }
}
