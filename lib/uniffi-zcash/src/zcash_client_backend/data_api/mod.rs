mod chain;
pub use self::chain::*;

mod wallet;
pub use self::wallet::*;

mod scanning;
pub use self::scanning::*;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;

use incrementalmerkletree::frontier::Frontier;

use crate::{
    ZcashBlockHash, ZcashBlockHeight, ZcashError, ZcashNonNegativeAmount, ZcashResult,
    ZcashTreeState,
};

use zcash_client_backend::data_api::AccountBirthday;
use zcash_client_backend::data_api::{
    AccountBalance, Balance, BlockMetadata, DecryptedTransaction, Ratio, ShieldedProtocol,
    WalletSummary,
};
use zcash_primitives::sapling;
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

impl Clone for ZcashDecryptedTransaction {
    fn clone(&self) -> Self {
        Self(DecryptedTransaction {
            tx: self.0.tx,
            sapling_outputs: self.0.sapling_outputs,
        })
    }
}

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
        account_balances: HashMap<String, Arc<ZcashAccountBalance>>,
        chain_tip_height: Arc<ZcashBlockHeight>,
        fully_scanned_height: Arc<ZcashBlockHeight>,
        scan_progress: Option<Arc<ZcashRatio>>,
    ) -> Self {
        Self(WalletSummary::new(
            account_balances
                .into_iter()
                .map(|(x, y)| (x.parse::<u32>().unwrap().into(), (*y).into()))
                .collect::<BTreeMap<AccountId, AccountBalance>>(),
            (*chain_tip_height).into(),
            (*fully_scanned_height).into(),
            scan_progress.map(|x| (*x).into()),
        ))
    }

    /// Returns the balances of accounts in the wallet, keyed by account ID.
    pub fn account_balances(&self) -> HashMap<String, Arc<ZcashAccountBalance>> {
        self.0
            .account_balances()
            .iter()
            .map(|(&x, &y)| {
                (
                    <AccountId as Into<u32>>::into(x).to_string(),
                    Arc::new(y.into()),
                )
            })
            .collect()
    }

    /// Returns the height of the current chain tip.
    pub fn chain_tip_height(&self) -> Arc<ZcashBlockHeight> {
        Arc::new(self.0.chain_tip_height().into())
    }

    /// Returns the height below which all blocks have been scanned by the wallet, ignoring blocks
    /// below the wallet birthday.
    pub fn fully_scanned_height(&self) -> Arc<ZcashBlockHeight> {
        Arc::new(self.0.fully_scanned_height().into())
    }

    /// Returns the progress of scanning shielded outputs, in terms of the ratio between notes
    /// scanned and the total number of notes added to the chain since the wallet birthday.
    ///
    /// This ratio should only be used to compute progress percentages, and the numerator and
    /// denominator should not be treated as authoritative note counts. Returns `None` if the
    /// wallet is unable to determine the size of the note commitment tree.
    pub fn scan_progress(&self) -> Option<Arc<ZcashRatio>> {
        self.0.scan_progress().map(From::from).map(Arc::new)
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
    pub fn numerator(&self) -> u64 {
        *self.0.numerator()
    }

    /// Returns the denominator of the ratio.
    pub fn denominator(&self) -> u64 {
        *self.0.denominator()
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
    pub fn zero() -> Self {
        Self(AccountBalance {
            sapling_balance: Balance::ZERO,
            unshielded: NonNegativeAmount::ZERO,
        })
    }

    /// Returns the total value of funds belonging to the account.
    pub fn total(&self) -> Arc<ZcashNonNegativeAmount> {
        Arc::new(self.0.total().into())
    }

    /// custom function to return unshielded value
    pub fn unshielded(&self) -> Arc<ZcashNonNegativeAmount> {
        Arc::new(self.0.unshielded.into())
    }

    /// custom function to return sapling spendable balance
    pub fn sapling_spendable_value(&self) -> Arc<ZcashNonNegativeAmount> {
        Arc::new(self.0.sapling_balance.spendable_value.into())
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

#[derive(Debug, Clone, Copy)]
pub struct ZcashBlockMetadata(BlockMetadata);

impl ZcashBlockMetadata {
    /// Constructs a new [`BlockMetadata`] value from its constituent parts.
    pub fn from_parts(
        block_height: Arc<ZcashBlockHeight>,
        block_hash: Arc<ZcashBlockHash>,
        sapling_tree_size: u32,
    ) -> Self {
        Self(BlockMetadata::from_parts(
            (*block_height).into(),
            (*block_hash).into(),
            sapling_tree_size,
        ))
    }

    /// Returns the block height.
    pub fn block_height(&self) -> Arc<ZcashBlockHeight> {
        Arc::new(self.0.block_height().into())
    }

    /// Returns the hash of the block
    pub fn block_hash(&self) -> Arc<ZcashBlockHash> {
        Arc::new(self.0.block_hash().into())
    }

    /// Returns the size of the Sapling note commitment tree as of the block that this
    /// [`BlockMetadata`] describes.
    pub fn sapling_tree_size(&self) -> u32 {
        self.0.sapling_tree_size()
    }
}

impl From<ZcashBlockMetadata> for BlockMetadata {
    fn from(inner: ZcashBlockMetadata) -> Self {
        inner.0
    }
}

impl From<BlockMetadata> for ZcashBlockMetadata {
    fn from(e: BlockMetadata) -> Self {
        ZcashBlockMetadata(e)
    }
}

type SaplingFrontier = Frontier<sapling::Node, { sapling::NOTE_COMMITMENT_TREE_DEPTH }>;

pub struct MerkleTreeFrontier(SaplingFrontier);

impl From<MerkleTreeFrontier> for SaplingFrontier {
    fn from(inner: MerkleTreeFrontier) -> Self {
        inner.0
    }
}

impl From<SaplingFrontier> for MerkleTreeFrontier {
    fn from(e: SaplingFrontier) -> Self {
        Self(e)
    }
}

#[derive(Clone, Debug)]
pub struct ZcashAccountBirthday(AccountBirthday);

impl ZcashAccountBirthday {
    /// Constructs a new [`AccountBirthday`] from a [`TreeState`] returned from `lightwalletd`.
    ///
    /// * `treestate`: The tree state corresponding to the last block prior to the wallet's
    ///    birthday height.
    /// * `recover_until`: An optional height at which the wallet should exit "recovery mode". In
    ///    order to avoid confusing shifts in wallet balance and spendability that may temporarily be
    ///    visible to a user during the process of recovering from seed, wallets may optionally set a
    ///    "recover until" height. The wallet is considered to be in "recovery mode" until there
    ///    exist no unscanned ranges between the wallet's birthday height and the provided
    ///    `recover_until` height, exclusive.
    pub fn from_treestate(
        treestate: Arc<ZcashTreeState>,
        recover_until: Option<Arc<ZcashBlockHeight>>,
    ) -> ZcashResult<Self> {
        AccountBirthday::from_treestate(
            (*treestate).clone().into(),
            recover_until.map(|x| (*x).into()),
        )
        .map(Self)
        .map_err(|_| ZcashError::Message {
            error: "Error creating birthday struct".to_string(),
        })
    }

    // /// Returns the Sapling note commitment tree frontier as of the end of the block at
    // /// [`Self::height`].
    pub fn sapling_frontier(&self) -> Arc<MerkleTreeFrontier> {
        Arc::new((*self.0.sapling_frontier()).clone().into())
    }

    /// Returns the birthday height of the account.
    pub fn height(&self) -> Arc<ZcashBlockHeight> {
        Arc::new(self.0.height().into())
    }

    /// Returns the height at which the wallet should exit "recovery mode".
    pub fn recover_until(&self) -> Option<Arc<ZcashBlockHeight>> {
        self.0.recover_until().map(From::from).map(Arc::new)
    }
}

impl From<ZcashAccountBirthday> for AccountBirthday {
    fn from(inner: ZcashAccountBirthday) -> Self {
        inner.0
    }
}

impl From<AccountBirthday> for ZcashAccountBirthday {
    fn from(e: AccountBirthday) -> Self {
        ZcashAccountBirthday(e)
    }
}
