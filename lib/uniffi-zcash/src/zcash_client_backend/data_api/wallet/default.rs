use std::num::NonZeroU32;
use std::sync::Arc;

use zcash_client_backend::data_api::wallet;
use zcash_client_backend::keys::UnifiedSpendingKey;
use zcash_client_sqlite::WalletDb;
use zcash_primitives::consensus;
use zcash_primitives::legacy::TransparentAddress;
use zcash_proofs::prover::LocalTxProver;

use crate::{
    MainGreedyInputSelector, TestGreedyInputSelector, ZcashConsensusParameters, ZcashError,
    ZcashGreedyInputSelector, ZcashLocalTxProver, ZcashMainGreedyInputSelector, ZcashMemoBytes,
    ZcashNonNegativeAmount, ZcashOvkPolicy, ZcashResult, ZcashTestGreedyInputSelector,
    ZcashTransaction, ZcashTransactionRequest, ZcashTransparentAddress, ZcashTxId,
    ZcashUnifiedSpendingKey, ZcashWalletDb,
};

/// Scans a [`Transaction`] for any information that can be decrypted by the accounts in
/// the wallet, and saves it to the wallet.
pub fn decrypt_and_store_transaction(
    params: ZcashConsensusParameters,
    z_db_data: Arc<ZcashWalletDb>,
    tx: Arc<ZcashTransaction>,
) -> ZcashResult<()> {
    let mut db_data = WalletDb::for_path(&z_db_data.path, params).unwrap();

    match wallet::decrypt_and_store_transaction(&params, &mut db_data, &((*tx).clone().into())) {
        Ok(_) => Ok(()),
        Err(_) => Err(ZcashError::Unknown),
    }
}

// /// [`sapling::TxProver`]: zcash_primitives::sapling::prover::TxProver
// where
//     DbT: WalletWrite + WalletCommitmentTrees,
//     DbT::NoteRef: Copy + Eq + Ord,
//     ParamsT: consensus::Parameters + Clone,
//     InputsT: InputSelector<DataSource = DbT>,
// Error<
//     <DbT as WalletRead>::Error,
//     <DbT as WalletCommitmentTrees>::Error,
//     InputsT::Error,
//     <InputsT::FeeRule as FeeRule>::Error,
//     DbT::NoteRef,
// >,

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn spend(
    z_db_data: ZcashWalletDb,
    params: ZcashConsensusParameters,
    prover: ZcashLocalTxProver,
    input_selector: Arc<dyn ZcashGreedyInputSelector>,
    usk: ZcashUnifiedSpendingKey,
    request: ZcashTransactionRequest,
    ovk_policy: ZcashOvkPolicy,
    min_confirmations: NonZeroU32,
) -> ZcashResult<ZcashTxId> {
    match params {
        ZcashConsensusParameters::MainNetwork => {
            let mut db_data = WalletDb::for_path(&z_db_data.path, consensus::MAIN_NETWORK).unwrap();
            let in_sel: ZcashMainGreedyInputSelector = (*input_selector).into();

            match wallet::spend(
                &mut db_data,
                &params,
                <ZcashLocalTxProver as Into<LocalTxProver>>::into(prover),
                &<ZcashMainGreedyInputSelector as Into<MainGreedyInputSelector>>::into(in_sel),
                &<ZcashUnifiedSpendingKey as Into<UnifiedSpendingKey>>::into(usk),
                request.into(),
                ovk_policy.into(),
                min_confirmations,
            ) {
                Ok(txid) => Ok(txid.into()),
                Err(_) => Err(ZcashError::Unknown),
            }
        }
        ZcashConsensusParameters::TestNetwork => {
            let mut db_data = WalletDb::for_path(&z_db_data.path, consensus::TEST_NETWORK).unwrap();
            let in_sel: ZcashTestGreedyInputSelector = (*input_selector).into();

            match wallet::spend(
                &mut db_data,
                &params,
                <ZcashLocalTxProver as Into<LocalTxProver>>::into(prover),
                &<ZcashTestGreedyInputSelector as Into<TestGreedyInputSelector>>::into(in_sel),
                &<ZcashUnifiedSpendingKey as Into<UnifiedSpendingKey>>::into(usk),
                request.into(),
                ovk_policy.into(),
                min_confirmations,
            ) {
                Ok(txid) => Ok(txid.into()),
                Err(_) => Err(ZcashError::Unknown),
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn shield_transparent_funds(
    z_db_data: ZcashWalletDb,
    params: ZcashConsensusParameters,
    prover: ZcashLocalTxProver,
    input_selector: Arc<dyn ZcashGreedyInputSelector>,
    shielding_threshold: ZcashNonNegativeAmount,
    usk: ZcashUnifiedSpendingKey,
    from_addrs: Vec<ZcashTransparentAddress>,
    memo: ZcashMemoBytes,
    min_confirmations: NonZeroU32,
) -> ZcashResult<ZcashTxId> {
    match params {
        ZcashConsensusParameters::MainNetwork => {
            let mut db_data = WalletDb::for_path(&z_db_data.path, consensus::MAIN_NETWORK).unwrap();
            let in_sel: ZcashMainGreedyInputSelector = (*input_selector).into();

            match wallet::shield_transparent_funds(
                &mut db_data,
                &params,
                <ZcashLocalTxProver as Into<LocalTxProver>>::into(prover),
                &<ZcashMainGreedyInputSelector as Into<MainGreedyInputSelector>>::into(in_sel),
                shielding_threshold.into(),
                &<ZcashUnifiedSpendingKey as Into<UnifiedSpendingKey>>::into(usk),
                &(from_addrs
                    .iter()
                    .map(From::from)
                    .collect::<Vec<TransparentAddress>>())[..],
                &(memo.into()),
                min_confirmations,
            ) {
                Ok(txid) => Ok(txid.into()),
                Err(_) => Err(ZcashError::Unknown),
            }
        }
        ZcashConsensusParameters::TestNetwork => {
            let mut db_data = WalletDb::for_path(&z_db_data.path, consensus::TEST_NETWORK).unwrap();
            let in_sel: ZcashTestGreedyInputSelector = (*input_selector).into();

            match wallet::shield_transparent_funds(
                &mut db_data,
                &params,
                <ZcashLocalTxProver as Into<LocalTxProver>>::into(prover),
                &<ZcashTestGreedyInputSelector as Into<TestGreedyInputSelector>>::into(in_sel),
                shielding_threshold.into(),
                &<ZcashUnifiedSpendingKey as Into<UnifiedSpendingKey>>::into(usk),
                &(from_addrs
                    .iter()
                    .map(From::from)
                    .collect::<Vec<TransparentAddress>>())[..],
                &(memo.into()),
                min_confirmations,
            ) {
                Ok(txid) => Ok(txid.into()),
                Err(_) => Err(ZcashError::Unknown),
            }
        }
    }
}
