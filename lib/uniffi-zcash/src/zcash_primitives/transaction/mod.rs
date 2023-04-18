mod components;
pub use self::components::*;

mod fees;
pub use self::fees::*;

use std::sync::{Arc, RwLock};

use hdwallet::rand_core::OsRng;
use orchard::{
    builder::{InProgress, Unauthorized, Unproven},
    bundle::Flags,
    keys::{SpendAuthorizingKey, SpendingKey},
    value::NoteValue,
};
use zcash_primitives::transaction::TxId;
use zcash_primitives::{
    consensus::BranchId,
    transaction::{
        builder::Builder,
        components::{sapling::builder::SaplingMetadata, Amount},
        Transaction, TransactionData, TxVersion,
    },
};

use crate::{
    utils::cast_slice, SecpSecretKey, ZcashAnchor, ZcashBlockHeight, ZcashBranchId,
    ZcashConsensusParameters, ZcashDiversifier, ZcashError, ZcashExtendedSpendingKey,
    ZcashLocalTxProver, ZcashMemoBytes, ZcashOrchardAddress, ZcashOrchardFullViewingKey,
    ZcashOrchardMerklePath, ZcashOrchardNote, ZcashOrchardOutgoingViewingKey,
    ZcashOrchardSpendingKey, ZcashOutgoingViewingKey, ZcashPaymentAddress, ZcashResult,
    ZcashSaplingMerklePath, ZcashSaplingNote, ZcashTransparentAddress,
};

pub use self::components::*;

pub struct ZcashTransactionBuilder {
    parameters: ZcashConsensusParameters,
    target_height: Arc<ZcashBlockHeight>,
    sapling_spends: SaplingSpends,
    sapling_outputs: SaplingOutputs,
    transparent_input: TransparentInput,
    transparent_output: TransparentOutput,
}

impl ZcashTransactionBuilder {
    pub fn new(parameters: ZcashConsensusParameters, target_height: Arc<ZcashBlockHeight>) -> Self {
        Self {
            parameters,
            target_height,
            sapling_spends: RwLock::new(Vec::new()),
            sapling_outputs: RwLock::new(Vec::new()),
            transparent_input: RwLock::new(Vec::new()),
            transparent_output: RwLock::new(Vec::new()),
        }
    }

    pub fn add_sapling_spend(
        &self,
        extsk: Arc<ZcashExtendedSpendingKey>,
        diversifier: Arc<ZcashDiversifier>,
        note: Arc<ZcashSaplingNote>,
        merkle_path: Arc<ZcashSaplingMerklePath>,
    ) {
        self.sapling_spends
            .write()
            .unwrap()
            .push((extsk, diversifier, note, merkle_path))
    }

    pub fn add_sapling_output(
        &self,
        ovk: Option<Arc<ZcashOutgoingViewingKey>>,
        to: Arc<ZcashPaymentAddress>,
        value: Arc<ZcashAmount>,
        memo: Arc<ZcashMemoBytes>,
    ) {
        self.sapling_outputs
            .write()
            .unwrap()
            .push((ovk, to, value, memo))
    }

    pub fn add_transparent_input(
        &self,
        sk: Arc<SecpSecretKey>,
        utxo: Arc<ZcashOutPoint>,
        coin: Arc<ZcashTxOut>,
    ) {
        self.transparent_input
            .write()
            .unwrap()
            .push((sk, utxo, coin))
    }

    pub fn add_transparent_output(
        &self,
        to: Arc<ZcashTransparentAddress>,
        value: Arc<ZcashAmount>,
    ) {
        self.transparent_output.write().unwrap().push((to, value))
    }

    pub fn build(
        &self,
        prover: Arc<ZcashLocalTxProver>,
        fee_rule: ZcashFeeRules,
    ) -> ZcashResult<ZcashTransactionAndSaplingMetadata> {
        let mut builder = Builder::new(self.parameters, (*self.target_height).into());

        self.sapling_spends.read().unwrap().iter().try_for_each(
            |(extsk, diversifier, note, merkle_path)| {
                builder.add_sapling_spend(
                    extsk.as_ref().into(),
                    diversifier.as_ref().into(),
                    note.as_ref().into(),
                    merkle_path.as_ref().into(),
                )
            },
        )?;

        self.sapling_outputs
            .read()
            .unwrap()
            .iter()
            .try_for_each(|(ovk, to, value, memo)| {
                builder.add_sapling_output(
                    ovk.as_ref().map(|ovk| ovk.as_ref()).map(From::from),
                    to.as_ref().into(),
                    value.as_ref().into(),
                    memo.as_ref().into(),
                )
            })?;

        self.transparent_input
            .read()
            .unwrap()
            .iter()
            .try_for_each(|(sk, utxo, coin)| {
                builder.add_transparent_input(
                    sk.as_ref().into(),
                    utxo.as_ref().into(),
                    coin.as_ref().into(),
                )
            })?;

        self.transparent_output
            .read()
            .unwrap()
            .iter()
            .try_for_each(|(to, value)| {
                builder.add_transparent_output(&to.as_ref().into(), value.as_ref().into())
            })?;

        match fee_rule {
            ZcashFeeRules::FixedStandard => {
                let fee = zcash_primitives::transaction::fees::fixed::FeeRule::standard();
                let result = builder.build(&prover.0, &fee).map_err(ZcashError::from)?;
                Ok(result.into())
            }
            ZcashFeeRules::FixedNonStandard { amount } => {
                let amount = Amount::from_u64(amount).or(Err("Error parsing amount"))?;
                let fee = zcash_primitives::transaction::fees::fixed::FeeRule::non_standard(amount);
                let result = builder.build(&prover.0, &fee).map_err(ZcashError::from)?;
                Ok(result.into())
            }
            ZcashFeeRules::Zip317Standard => {
                let fee = zcash_primitives::transaction::fees::zip317::FeeRule::standard();
                let result = builder.build(&prover.0, &fee).map_err(ZcashError::from)?;
                Ok(result.into())
            }
            ZcashFeeRules::Zip317NonStandard {
                marginal_fee,
                grace_actions,
                p2pkh_standard_input_size,
                p2pkh_standard_output_size,
            } => {
                let fee = match zcash_primitives::transaction::fees::zip317::FeeRule::non_standard(
                    Amount::from_u64(marginal_fee).or(Err("Error parsing amount"))?,
                    grace_actions.try_into()?,
                    p2pkh_standard_input_size.try_into()?,
                    p2pkh_standard_output_size.try_into()?
                ) {
                    Some(fee) => fee,
                    None => return Err("p2pkh_standard_input_size and p2pkh_standard_output_size should not be zero".into()),
                };
                let result = builder.build(&prover.0, &fee).map_err(ZcashError::from)?;
                Ok(result.into())
            }
        }
    }
}

type TransparentInput = RwLock<Vec<(Arc<SecpSecretKey>, Arc<ZcashOutPoint>, Arc<ZcashTxOut>)>>;

type TransparentOutput = RwLock<Vec<(Arc<ZcashTransparentAddress>, Arc<ZcashAmount>)>>;

type SaplingSpends = RwLock<
    Vec<(
        Arc<ZcashExtendedSpendingKey>,
        Arc<ZcashDiversifier>,
        Arc<ZcashSaplingNote>,
        Arc<ZcashSaplingMerklePath>,
    )>,
>;

type SaplingOutputs = RwLock<
    Vec<(
        Option<Arc<ZcashOutgoingViewingKey>>,
        Arc<ZcashPaymentAddress>,
        Arc<ZcashAmount>,
        Arc<ZcashMemoBytes>,
    )>,
>;

/// A selector for the desired fee rules for applying to a transaction.
pub enum ZcashFeeRules {
    FixedStandard,
    FixedNonStandard {
        amount: u64,
    },
    Zip317Standard,
    Zip317NonStandard {
        marginal_fee: u64,
        grace_actions: u64,
        p2pkh_standard_input_size: u64,
        p2pkh_standard_output_size: u64,
    },
}

/// A Zcash transaction.
pub struct ZcashTransaction(Transaction);

impl ZcashTransaction {
    pub fn to_bytes(&self) -> ZcashResult<Vec<u8>> {
        let mut data = Vec::new();
        self.0.write(&mut data).map_err(ZcashError::from)?;
        Ok(data)
    }

    pub fn from_bytes(
        data: &[u8],
        consensus_branch_id: ZcashBranchId,
    ) -> ZcashResult<ZcashTransaction> {
        let tx = Transaction::read(data, consensus_branch_id.into())?;
        Ok(tx.into())
    }

    pub fn txid(&self) -> Arc<ZcashTxId> {
        Arc::new(self.0.txid().into())
    }

    pub fn version(&self) -> Arc<ZcashTxVersion> {
        Arc::new(self.0.version().into())
    }

    pub fn consensus_branch_id(&self) -> ZcashBranchId {
        self.0.consensus_branch_id().into()
    }

    pub fn lock_time(&self) -> u32 {
        self.0.lock_time()
    }

    pub fn expiry_height(&self) -> Arc<ZcashBlockHeight> {
        Arc::new(self.0.expiry_height().into())
    }

    /// Returns the total fees paid by the transaction, given a function that can be used to
    /// retrieve the value of previous transactions' transparent outputs that are being spent in
    /// this transaction.
    // pub fn fee_paid(&self) -> ZcashResult<Arc<ZcashAmount>> {
    //     let amount = self.0.fee_paid::<BalanceError, _>(|_| Ok(Amount::zero()))?;
    //     Ok(Arc::new(amount.into()))
    // }

    pub fn transparent_bundle(&self) -> Option<Arc<ZcashTransparentBundle>> {
        self.0.transparent_bundle().map(|b| Arc::new(b.into()))  
    }
}

impl From<Transaction> for ZcashTransaction {
    fn from(inner: Transaction) -> Self {
        ZcashTransaction(inner)
    }
}
pub struct ZcashTransactionAndSaplingMetadata {
    pub transaction: Arc<ZcashTransaction>,
    pub sapling_metadata: Arc<ZcashSaplingMetadata>,
}

impl From<(Transaction, SaplingMetadata)> for ZcashTransactionAndSaplingMetadata {
    fn from((transaction, sapling_metadata): (Transaction, SaplingMetadata)) -> Self {
        ZcashTransactionAndSaplingMetadata {
            transaction: Arc::new(transaction.into()),
            sapling_metadata: Arc::new(sapling_metadata.into()),
        }
    }
}

pub struct ZcashTxId(TxId);

impl ZcashTxId {
    pub fn from_bytes(data: &[u8]) -> ZcashResult<Self> {
        Ok(TxId::from_bytes(cast_slice(data)?).into())
    }

    pub fn to_bytes(&self) -> ZcashResult<Vec<u8>> {
        let mut data = Vec::with_capacity(32);
        self.0.write(&mut data)?;
        Ok(data)
    }
}

impl From<TxId> for ZcashTxId {
    fn from(inner: TxId) -> Self {
        ZcashTxId(inner)
    }
}

pub struct ZcashOrchardTransactionBuilder {
    parameters: ZcashConsensusParameters,
    target_height: Arc<ZcashBlockHeight>,
    expiry_height: Arc<ZcashBlockHeight>,
    anchor: Arc<ZcashAnchor>,
    spends: OrchardSpends,
    outputs: OrchardOutputs,
}

impl ZcashOrchardTransactionBuilder {
    pub fn new(
        parameters: ZcashConsensusParameters,
        target_height: Arc<ZcashBlockHeight>,
        expiry_height: Arc<ZcashBlockHeight>,
        anchor: Arc<ZcashAnchor>,
    ) -> Self {
        Self {
            parameters,
            target_height,
            expiry_height,
            anchor,
            spends: RwLock::new(Vec::new()),
            outputs: RwLock::new(Vec::new()),
        }
    }

    pub fn add_spend(
        &self,
        fvk: Arc<ZcashOrchardFullViewingKey>,
        note: Arc<ZcashOrchardNote>,
        merkle_path: Arc<ZcashOrchardMerklePath>,
    ) {
        self.spends.write().unwrap().push((fvk, note, merkle_path))
    }

    pub fn add_output(
        &self,
        ovk: Arc<ZcashOrchardOutgoingViewingKey>,
        recipient: Arc<ZcashOrchardAddress>,
        value: u64,
        memo: Option<Vec<u8>>,
    ) -> ZcashResult<()> {
        let m = match memo {
            Some(m) => Some(cast_slice(m.as_slice())?),
            None => None,
        };

        self.outputs
            .write()
            .unwrap()
            .push((ovk, recipient, value, m));
        Ok(())
    }

    pub fn build(
        &self,
        keys: Vec<Arc<ZcashOrchardSpendingKey>>,
        sighash: Vec<u8>,
    ) -> ZcashResult<Arc<ZcashTransaction>> {
        let mut builder = orchard::builder::Builder::new(
            Flags::from_parts(true, true),
            self.anchor.as_ref().into(),
        );

        self.spends
            .read()
            .unwrap()
            .iter()
            .try_for_each(|(fvk, note, merkle_path)| {
                builder.add_spend(
                    fvk.as_ref().into(),
                    note.as_ref().into(),
                    merkle_path.as_ref().into(),
                )
            })?;

        self.outputs
            .read()
            .unwrap()
            .iter()
            .try_for_each(|(ovk, recipient, value, memo)| {
                builder.add_recipient(
                    Some(ovk.as_ref().into()),
                    recipient.as_ref().into(),
                    NoteValue::from_raw(*value),
                    *memo,
                )
            })?;

        let bundle: orchard::Bundle<InProgress<Unproven, Unauthorized>, Amount> =
            builder.build(OsRng).unwrap();

        let pk = orchard::circuit::ProvingKey::build();
        let casted_sighash: [u8; 32] = cast_slice(sighash.as_slice())?;
        let proved_bundle = bundle.create_proof(&pk, OsRng)?;

        let inner_keys = keys
            .iter()
            .map(|k| k.as_ref())
            .map(From::from)
            .collect::<Vec<SpendingKey>>()
            .iter()
            .map(From::from)
            .collect::<Vec<SpendAuthorizingKey>>();

        let authorized_bundle =
            proved_bundle.apply_signatures(OsRng, casted_sighash, inner_keys.as_slice())?;

        let consensus_branch_id =
            BranchId::for_height(&self.parameters, self.target_height.as_ref().into());

        let transaction = TransactionData::from_parts(
            TxVersion::suggested_for_branch(consensus_branch_id),
            consensus_branch_id,
            0,
            self.expiry_height.as_ref().into(),
            None,
            None,
            None,
            Some(authorized_bundle),
        )
        // The unwrap() here is safe because the txid hashing
        // of freeze() should be infalliable.
        .freeze()
        .unwrap();

        Ok(Arc::new(transaction.into()))
    }
}

type OrchardSpends = RwLock<
    Vec<(
        Arc<ZcashOrchardFullViewingKey>,
        Arc<ZcashOrchardNote>,
        Arc<ZcashOrchardMerklePath>,
    )>,
>;

type OrchardOutputs = RwLock<
    Vec<(
        Arc<ZcashOrchardOutgoingViewingKey>,
        Arc<ZcashOrchardAddress>,
        u64,
        Option<[u8; 512]>,
    )>,
>;

pub enum ZcashTxVersionSelection {
    Sprout { v: u32 },
    Overwinter,
    Sapling,
    Zip225,
}

impl From<TxVersion> for ZcashTxVersionSelection {
    fn from(value: TxVersion) -> Self {
        match value {
            TxVersion::Sprout(u) => ZcashTxVersionSelection::Sprout { v: u },
            TxVersion::Overwinter => ZcashTxVersionSelection::Overwinter,
            TxVersion::Sapling => ZcashTxVersionSelection::Sapling,
            TxVersion::Zip225 => ZcashTxVersionSelection::Zip225,
        }
    }
}

impl From<ZcashTxVersionSelection> for TxVersion {
    fn from(value: ZcashTxVersionSelection) -> Self {
        match value {
            ZcashTxVersionSelection::Sprout { v } => TxVersion::Sprout(v),
            ZcashTxVersionSelection::Overwinter => TxVersion::Overwinter,
            ZcashTxVersionSelection::Sapling => TxVersion::Sapling,
            ZcashTxVersionSelection::Zip225 => TxVersion::Zip225,
        }
    }
}

/// The set of defined transaction format versions.
///
/// This is serialized in the first four or eight bytes of the transaction format, and
/// represents valid combinations of the `(overwintered, version, version_group_id)`
/// transaction fields. Note that this is not dependent on epoch, only on transaction encoding.
/// For example, if a particular epoch defines a new transaction version but also allows the
/// previous version, then only the new version would be added to this enum.
pub struct ZcashTxVersion(TxVersion);

impl ZcashTxVersion {
    pub fn selection(&self) -> ZcashTxVersionSelection {
        self.0.into()
    }

    pub fn from_bytes(data: &[u8]) -> ZcashResult<Self> {
        Ok(TxVersion::read(data)?.into())
    }

    pub fn header(&self) -> u32 {
        self.0.header()
    }

    pub fn version_group_id(&self) -> u32 {
        self.0.version_group_id()
    }

    pub fn to_bytes(&self) -> ZcashResult<Vec<u8>> {
        let mut buffer = Vec::new();
        self.0.write(&mut buffer)?;
        Ok(buffer)
    }

    pub fn has_sprout(&self) -> bool {
        self.0.has_sprout()
    }

    pub fn has_overwinter(&self) -> bool {
        self.0.has_overwinter()
    }

    pub fn has_sapling(&self) -> bool {
        self.0.has_sapling()
    }

    pub fn has_orchard(&self) -> bool {
        self.0.has_orchard()
    }

    pub fn suggested_for_branch(consensus_branch_id: ZcashBranchId) -> Self {
        TxVersion::suggested_for_branch(consensus_branch_id.into()).into()
    }
}

impl From<TxVersion> for ZcashTxVersion {
    fn from(inner: TxVersion) -> Self {
        ZcashTxVersion(inner)
    }
}
