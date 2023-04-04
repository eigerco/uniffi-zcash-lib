mod components;
pub use self::components::*;

mod fees;
pub use self::fees::*;

use std::sync::{Arc, RwLock};

use zcash_primitives::transaction::{
    builder::Builder,
    components::{sapling::builder::SaplingMetadata, Amount},
    Transaction,
};

use crate::{
    SecpSecretKey, ZcashBlockHeight, ZcashConsensusParameters, ZcashDiversifier, ZcashError,
    ZcashExtendedSpendingKey, ZcashLocalTxProver, ZcashMemoBytes, ZcashOutgoingViewingKey,
    ZcashPaymentAddress, ZcashResult, ZcashSaplingMerklePath, ZcashSaplingNote,
    ZcashTransparentAddress,
};

pub use self::components::*;

pub struct ZcashTransactionBuilder {
    parameters: ZcashConsensusParameters,
    target_height: Arc<ZcashBlockHeight>,
    sapling_spends: RwLock<
        Vec<(
            Arc<ZcashExtendedSpendingKey>,
            Arc<ZcashDiversifier>,
            Arc<ZcashSaplingNote>,
            Arc<ZcashSaplingMerklePath>,
        )>,
    >,
    sapling_outputs: RwLock<
        Vec<(
            Option<Arc<ZcashOutgoingViewingKey>>,
            Arc<ZcashPaymentAddress>,
            Arc<ZcashAmount>,
            Arc<ZcashMemoBytes>,
        )>,
    >,
    transparent_input: RwLock<Vec<(Arc<SecpSecretKey>, Arc<ZcashOutPoint>, Arc<ZcashTxOut>)>>,
    transparent_output: RwLock<Vec<(Arc<ZcashTransparentAddress>, Arc<ZcashAmount>)>>,
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
                let ovk_p = match ovk {
                    Some(ovk) => Some(ovk.as_ref()),
                    None => None,
                };

                builder.add_sapling_output(
                    ovk_p.map(From::from),
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
