mod chain;
pub use self::chain::*;

mod wallet;
pub use self::wallet::*;

mod scanning;
pub use self::scanning::*;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;

use zcash_client_backend::address::UnifiedAddress;
use zcash_client_backend::data_api::{
    AccountBalance, AccountBirthday, Balance, BlockMetadata, DecryptedTransaction, NullifierQuery,
    PoolType, Ratio, Recipient, ScannedBlock, SentTransaction, SentTransactionOutput,
    ShieldedProtocol, WalletSummary,
};
use zcash_client_backend::encoding::AddressCodec;
use zcash_client_backend::wallet::{WalletSaplingOutput, WalletSaplingSpend, WalletTx};

use zcash_primitives::consensus::BlockHeight;
use zcash_primitives::legacy::TransparentAddress;
use zcash_primitives::sapling::{self, PaymentAddress};
use zcash_primitives::transaction::{components::amount::NonNegativeAmount, TxId};
use zcash_primitives::zip32::AccountId;

use crate::{
    ZcashAccountId, ZcashAmount, ZcashBlockHash, ZcashBlockHeight, ZcashBranchId,
    ZcashConsensusParameters, ZcashError, ZcashMemoBytes, ZcashNonNegativeAmount, ZcashOutPoint,
    ZcashResult, ZcashSaplingNode, ZcashSaplingNote, ZcashSaplingNullifier, ZcashTransaction,
    ZcashTreeState, ZcashTxId, ZcashWalletTx,
};

use incrementalmerkletree::frontier::Frontier;
use incrementalmerkletree::Retention;

// experiment
pub fn clone_orig<P: Clone, T: std::convert::From<P>>(x: &P) -> T {
    (*x).clone().into()
}

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

#[derive(Copy, Clone)]
pub struct MerkleTreeRetention(Retention<BlockHeight>);

impl From<MerkleTreeRetention> for Retention<BlockHeight> {
    fn from(inner: MerkleTreeRetention) -> Self {
        inner.0
    }
}

impl From<Retention<BlockHeight>> for MerkleTreeRetention {
    fn from(e: Retention<BlockHeight>) -> Self {
        Self(e)
    }
}

type SaplingFrontier = Frontier<sapling::Node, { sapling::NOTE_COMMITMENT_TREE_DEPTH }>;

pub enum ZcashNullifierQuery {
    Unspent,
    All,
}

impl From<ZcashNullifierQuery> for NullifierQuery {
    fn from(e: ZcashNullifierQuery) -> Self {
        match e {
            ZcashNullifierQuery::Unspent => Self::Unspent,
            ZcashNullifierQuery::All => Self::All,
        }
    }
}

impl From<NullifierQuery> for ZcashNullifierQuery {
    fn from(e: NullifierQuery) -> Self {
        match e {
            NullifierQuery::Unspent => Self::Unspent,
            NullifierQuery::All => Self::All,
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

#[derive(Copy, Clone)]
pub struct TimeOffsetDateTime(time::OffsetDateTime);

impl From<time::OffsetDateTime> for TimeOffsetDateTime {
    fn from(e: time::OffsetDateTime) -> Self {
        Self(e)
    }
}

impl From<TimeOffsetDateTime> for time::OffsetDateTime {
    fn from(inner: TimeOffsetDateTime) -> Self {
        inner.0
    }
}

/// A transaction that was constructed and sent by the wallet.
///
/// The purpose of this struct is to permit atomic updates of the
/// wallet database when transactions are created and submitted
/// to the network.
#[derive(Clone)]
pub struct ZcashSentTransaction {
    pub tx: Arc<ZcashTransaction>,
    pub created: Arc<TimeOffsetDateTime>,
    pub account: ZcashAccountId,
    pub outputs: Vec<Arc<ZcashSentTransactionOutput>>,
    pub fee_amount: Arc<ZcashAmount>,
    pub utxos_spent: Vec<Arc<ZcashOutPoint>>,
}

impl<'a> From<&'a ZcashSentTransaction> for SentTransaction<'a> {
    fn from(e: &'a ZcashSentTransaction) -> Self {
        SentTransaction {
            tx: e.tx.as_ref().into(),
            created: (*e.created).into(),
            account: e.account.into(),
            outputs: e
                .outputs
                .clone()
                .into_iter()
                .map(|x| (*x).clone().into())
                .collect(),
            fee_amount: (*e.fee_amount).into(),
            utxos_spent: e
                .utxos_spent
                .clone()
                .into_iter()
                .map(|x| (*x).clone().into())
                .collect(),
        }
    }
}

impl From<SentTransaction<'static>> for ZcashSentTransaction {
    fn from(e: SentTransaction<'static>) -> Self {
        let convert_outputs = |x: &SentTransactionOutput| -> ZcashSentTransactionOutput {
            ZcashSentTransactionOutput::from_parts(
                x.output_index().try_into().unwrap(),
                (*x.recipient()).clone().into(),
                Arc::new(x.value().into()),
                x.memo().map(clone_orig).map(Arc::new),
                x.sapling_change_to().map(clone_orig),
            )
        };

        let mut txdata = Vec::new();
        let _ = e.tx.write(&mut txdata).map_err(ZcashError::from);

        // NOTE need to find a way not to hardcode the BranchId
        let ztx: ZcashTransaction = ZcashTransaction::from_bytes(&txdata[..], ZcashBranchId::Nu5)
            .expect("error converting tx data!");

        Self {
            tx: Arc::new(ztx),
            created: Arc::new(e.created.into()),
            account: e.account.into(),
            outputs: e
                .outputs
                .iter()
                .map(convert_outputs)
                .map(Arc::new)
                .collect(),
            fee_amount: Arc::new(e.fee_amount.into()),
            utxos_spent: e.utxos_spent.iter().map(clone_orig).map(Arc::new).collect(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ZcashShieldedProtocol {
    // Orchard
    Sapling,
}

impl From<ZcashShieldedProtocol> for ShieldedProtocol {
    fn from(e: ZcashShieldedProtocol) -> Self {
        match e {
            ZcashShieldedProtocol::Sapling => Self::Sapling,
        }
    }
}

impl From<ShieldedProtocol> for ZcashShieldedProtocol {
    fn from(e: ShieldedProtocol) -> Self {
        match e {
            ShieldedProtocol::Sapling => Self::Sapling,
        }
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

#[derive(Clone)]
pub struct TripleSaplingNullifierMap {
    pub txid: Arc<ZcashTxId>,
    pub tx_idx: u16,
    pub nullifiers: Vec<Arc<ZcashSaplingNullifier>>,
}

impl From<TripleSaplingNullifierMap> for (TxId, u16, Vec<sapling::Nullifier>) {
    fn from(triple: TripleSaplingNullifierMap) -> Self {
        (
            (*triple.txid).into(),
            triple.tx_idx,
            triple.nullifiers.into_iter().map(|x| (*x).into()).collect(),
        )
    }
}

impl From<(TxId, u16, Vec<sapling::Nullifier>)> for TripleSaplingNullifierMap {
    fn from(triple: (TxId, u16, Vec<sapling::Nullifier>)) -> Self {
        TripleSaplingNullifierMap {
            txid: Arc::new(triple.0.into()),
            tx_idx: triple.1,
            nullifiers: triple.2.into_iter().map(From::from).map(Arc::new).collect(),
        }
    }
}

#[derive(Clone)]
pub struct TupleSaplingCommitments {
    pub node: Arc<ZcashSaplingNode>,
    pub retention: Arc<MerkleTreeRetention>,
}

impl From<TupleSaplingCommitments> for (sapling::Node, Retention<BlockHeight>) {
    fn from(tuple: TupleSaplingCommitments) -> Self {
        ((*tuple.node).into(), (*tuple.retention).into())
    }
}

impl From<(sapling::Node, Retention<BlockHeight>)> for TupleSaplingCommitments {
    fn from(tuple: (sapling::Node, Retention<BlockHeight>)) -> Self {
        TupleSaplingCommitments {
            node: Arc::new(tuple.0.into()),
            retention: Arc::new(tuple.1.into()),
        }
    }
}

/// The subset of information that is relevant to this wallet that has been
/// decrypted and extracted from a [`CompactBlock`].
///
/// [`CompactBlock`]: crate::proto::compact_formats::CompactBlock
pub struct ZcashScannedBlock(ScannedBlock<sapling::Nullifier>);

impl ZcashScannedBlock {
    /// Constructs a new `ScannedBlock`
    pub fn from_parts(
        metadata: Arc<ZcashBlockMetadata>,
        block_time: u32,
        transactions: Vec<Arc<ZcashWalletTx>>,
        sapling_nullifier_map: Vec<TripleSaplingNullifierMap>,
        sapling_commitments: Vec<TupleSaplingCommitments>,
    ) -> Self {
        Self(ScannedBlock::from_parts(
            (*metadata).into(),
            block_time,
            transactions
                .into_iter()
                .map(|x| (*x).clone().into())
                .collect::<Vec<WalletTx<sapling::Nullifier>>>(),
            sapling_nullifier_map.into_iter().map(From::from).collect(),
            sapling_commitments.into_iter().map(From::from).collect(),
        ))
    }

    /// Returns the height of the block that was scanned.
    pub fn height(&self) -> Arc<ZcashBlockHeight> {
        Arc::new(self.0.height().into())
    }

    /// Returns the block hash of the block that was scanned.
    pub fn block_hash(&self) -> Arc<ZcashBlockHash> {
        Arc::new(self.0.block_hash().into())
    }

    /// Returns the block time of the block that was scanned, as a Unix timestamp in seconds.
    pub fn block_time(&self) -> u32 {
        self.0.block_time()
    }

    /// Returns the metadata describing the state of the note commitment trees as of the end of the
    /// scanned block.
    ///
    /// The metadata returned from this method is guaranteed to be consistent with what is returned
    /// by [`Self::height`] and [`Self::block_hash`].
    pub fn metadata(&self) -> Arc<ZcashBlockMetadata> {
        Arc::new((*self.0.metadata()).into())
    }

    // /// Returns the list of transactions from the block that are relevant to the wallet.
    pub fn transactions(&self) -> Vec<Arc<ZcashWalletTx>> {
        let output_from_parts = |x: &WalletSaplingOutput<sapling::Nullifier>| -> WalletSaplingOutput<sapling::Nullifier> {
            WalletSaplingOutput::from_parts(
                x.index(),
                *x.cmu(),
                x.ephemeral_key().clone(),
                x.account(),
                x.note().clone(),
                x.is_change(),
                x.note_commitment_tree_position(),
                *x.nf(),
            )
        };

        self.0
            .transactions()
            .iter()
            .map(|x| {
                let sapling_outputs = x
                    .sapling_outputs
                    .iter()
                    .map(output_from_parts)
                    .collect::<Vec<WalletSaplingOutput<sapling::Nullifier>>>();

                let sapling_spends = x
                    .sapling_spends
                    .iter()
                    .map(|x| WalletSaplingSpend::from_parts(x.index(), *x.nf(), x.account()))
                    .collect::<Vec<WalletSaplingSpend>>();

                WalletTx {
                    txid: x.txid,
                    index: x.index,
                    sapling_spends,
                    sapling_outputs,
                }
            })
            .map(From::from)
            .map(Arc::new)
            .collect()
    }

    /// Returns the vector of Sapling nullifiers for each transaction in the block.
    ///
    /// The returned tuple is keyed by both transaction ID and the index of the transaction within
    /// the block, so that either the txid or the combination of the block hash available from
    /// [`Self::block_hash`] and returned transaction index may be used to uniquely identify the
    /// transaction, depending upon the needs of the caller.
    pub fn sapling_nullifier_map(&self) -> Vec<TripleSaplingNullifierMap> {
        self.0
            .sapling_nullifier_map()
            .iter()
            .map(|x| (*x).clone().into())
            .collect()
    }

    /// Returns the ordered list of Sapling note commitments to be added to the note commitment
    /// tree.
    pub fn sapling_commitments(&self) -> Vec<TupleSaplingCommitments> {
        self.0
            .sapling_commitments()
            .iter()
            .map(|x| (*x).into())
            .collect()
    }
}

impl Clone for ZcashScannedBlock {
    fn clone(&self) -> Self {
        Self(ScannedBlock::from_parts(
            *self.0.metadata(),
            self.0.block_time(),
            self.transactions()
                .into_iter()
                .map(|x| (*x).clone().into())
                .collect(),
            self.0.sapling_nullifier_map().to_vec(),
            self.0.sapling_commitments().to_vec(),
        ))
    }
}

impl From<ZcashScannedBlock> for ScannedBlock<sapling::Nullifier> {
    fn from(inner: ZcashScannedBlock) -> Self {
        inner.0
    }
}

impl From<ScannedBlock<sapling::Nullifier>> for ZcashScannedBlock {
    fn from(e: ScannedBlock<sapling::Nullifier>) -> Self {
        Self(e)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ZcashPoolType {
    /// The transparent value pool
    Transparent,
    /// A shielded value pool.
    Shielded { v: ZcashShieldedProtocol },
}

impl From<ZcashPoolType> for PoolType {
    fn from(e: ZcashPoolType) -> Self {
        match e {
            ZcashPoolType::Transparent => Self::Transparent,
            ZcashPoolType::Shielded { v } => Self::Shielded(v.into()),
        }
    }
}

impl From<PoolType> for ZcashPoolType {
    fn from(e: PoolType) -> Self {
        match e {
            PoolType::Transparent => Self::Transparent,
            PoolType::Shielded(v) => Self::Shielded { v: v.into() },
        }
    }
}

/// A type that represents the recipient of a transaction output; a recipient address (and, for
/// unified addresses, the pool to which the payment is sent) in the case of outgoing output, or an
/// internal account ID and the pool to which funds were sent in the case of a wallet-internal
/// output.
#[derive(Clone)]
pub enum ZcashRecipient {
    // Transparent {
    //     transparent_address_encoded: String,
    //     params: ZcashConsensusParameters,
    // },
    Transparent {
        script: Vec<u8>,
    },
    Sapling {
        payment_address_bytes: Vec<u8>,
    },
    Unified {
        uae: String,
        params: ZcashConsensusParameters,
        zpt: ZcashPoolType,
    },
    InternalAccount {
        aid: ZcashAccountId,
        zpt: ZcashPoolType,
    },
}

impl From<ZcashRecipient> for Recipient {
    fn from(e: ZcashRecipient) -> Self {
        match e {
            ZcashRecipient::Transparent { script } => {
                let bytes: [u8; 20] = script.try_into().unwrap();

                Self::Transparent(TransparentAddress::Script(bytes))
            }
            ZcashRecipient::Sapling {
                payment_address_bytes,
            } => {
                let bytes: [u8; 43] = payment_address_bytes.try_into().unwrap();
                let address = PaymentAddress::from_bytes(&bytes).unwrap();
                Self::Sapling(address)
            }
            ZcashRecipient::Unified { uae, params, zpt } => {
                let address = UnifiedAddress::decode(&params, &uae).unwrap();
                Self::Unified(address, zpt.into())
            }
            ZcashRecipient::InternalAccount { aid, zpt } => {
                Self::InternalAccount(aid.into(), zpt.into())
            }
        }
    }
}

impl From<Recipient> for ZcashRecipient {
    fn from(e: Recipient) -> Self {
        match e {
            Recipient::Transparent(t_addr) => Self::Transparent {
                script: t_addr.script().0,
            },
            Recipient::Sapling(payment_address) => Self::Sapling {
                payment_address_bytes: payment_address.to_bytes().to_vec(),
            },
            Recipient::Unified(address, zpt) => {
                // NOTE encode has an expect call and I don't know how else
                // to handle it properly
                let (uae, params) = if std::panic::catch_unwind(|| {
                    address.encode(&ZcashConsensusParameters::TestNetwork)
                })
                .is_ok()
                {
                    (
                        address.encode(&ZcashConsensusParameters::TestNetwork),
                        ZcashConsensusParameters::TestNetwork,
                    )
                } else if std::panic::catch_unwind(|| {
                    address.encode(&ZcashConsensusParameters::MainNetwork)
                })
                .is_ok()
                {
                    (
                        address.encode(&ZcashConsensusParameters::MainNetwork),
                        ZcashConsensusParameters::MainNetwork,
                    )
                } else {
                    panic!("This address belongs to an unrecognized network")
                };

                Self::Unified {
                    uae,
                    params,
                    zpt: zpt.into(),
                }
            }
            Recipient::InternalAccount(aid, zpt) => Self::InternalAccount {
                aid: aid.into(),
                zpt: zpt.into(),
            },
        }
    }
}

pub struct TupleAccountIdAndSaplingNote {
    pub account_id: ZcashAccountId,
    pub sapling_note: Arc<ZcashSaplingNote>,
}

impl From<TupleAccountIdAndSaplingNote> for (AccountId, sapling::Note) {
    fn from(tuple: TupleAccountIdAndSaplingNote) -> Self {
        (
            tuple.account_id.into(),
            (*tuple.sapling_note).clone().into(),
        )
    }
}

impl From<(AccountId, sapling::Note)> for TupleAccountIdAndSaplingNote {
    fn from(tuple: (AccountId, sapling::Note)) -> Self {
        TupleAccountIdAndSaplingNote {
            account_id: tuple.0.into(),
            sapling_note: Arc::new(tuple.1.into()),
        }
    }
}

/// A type that represents an output (either Sapling or transparent) that was sent by the wallet.
pub struct ZcashSentTransactionOutput(SentTransactionOutput);

impl Clone for ZcashSentTransactionOutput {
    fn clone(&self) -> Self {
        Self(SentTransactionOutput::from_parts(
            self.0.output_index(),
            self.0.recipient().clone(),
            self.0.value(),
            self.0.memo().cloned(),
            self.0.sapling_change_to().cloned(),
        ))
    }
}

impl ZcashSentTransactionOutput {
    pub fn from_parts(
        output_index: u32,
        recipient: ZcashRecipient,
        value: Arc<ZcashAmount>,
        memo: Option<Arc<ZcashMemoBytes>>,
        sapling_change_to: Option<TupleAccountIdAndSaplingNote>,
    ) -> Self {
        Self(SentTransactionOutput::from_parts(
            output_index.try_into().unwrap(),
            recipient.into(),
            (*value).into(),
            memo.map(|x| (*x).clone().into()),
            sapling_change_to.map(|x| x.into()),
        ))
    }

    /// Returns the index within the transaction that contains the recipient output.
    ///
    /// - If `recipient_address` is a Sapling address, this is an index into the Sapling
    ///   outputs of the transaction.
    /// - If `recipient_address` is a transparent address, this is an index into the
    ///   transparent outputs of the transaction.
    pub fn output_index(&self) -> u32 {
        self.0.output_index().try_into().unwrap()
    }

    // /// Returns the recipient address of the transaction, or the account id for wallet-internal
    // /// transactions.
    pub fn recipient(&self) -> ZcashRecipient {
        (*self.0.recipient()).clone().into()
    }

    /// Returns the value of the newly created output.
    pub fn value(&self) -> Arc<ZcashAmount> {
        Arc::new(self.0.value().into())
    }

    /// Returns the memo that was attached to the output, if any. This will only be `None`
    /// for transparent outputs.
    pub fn memo(&self) -> Option<Arc<ZcashMemoBytes>> {
        self.0.memo().map(|x| (*x).clone().into()).map(Arc::new)
    }

    /// Returns the account to which change (or wallet-internal value in the case of a shielding
    /// transaction) was sent, along with the change note.
    pub fn sapling_change_to(&self) -> Option<TupleAccountIdAndSaplingNote> {
        self.0.sapling_change_to().map(|x| (*x).clone().into())
    }
}

impl From<ZcashSentTransactionOutput> for SentTransactionOutput {
    fn from(inner: ZcashSentTransactionOutput) -> Self {
        inner.0
    }
}

impl From<SentTransactionOutput> for ZcashSentTransactionOutput {
    fn from(e: SentTransactionOutput) -> Self {
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
