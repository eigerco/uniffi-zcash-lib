use zcash_client_backend::data_api::wallet;

use std::num::NonZeroU32;

use std::sync::Arc;

use crate::{
    ZcashConsensusParameters, ZcashError, ZcashLocalTxProver, ZcashOvkPolicy, ZcashResult,
    ZcashTransaction, ZcashTransactionRequest, ZcashTxId, ZcashUnifiedSpendingKey, ZcashWalletDb,
};

use zcash_client_backend::keys::UnifiedSpendingKey;
use zcash_proofs::prover::LocalTxProver;

pub mod input_selection;

/// Scans a [`Transaction`] for any information that can be decrypted by the accounts in
/// the wallet, and saves it to the wallet.
pub fn decrypt_and_store_transaction(
    params: ZcashConsensusParameters,
    z_db_data: Arc<ZcashWalletDb>,
    tx: Arc<ZcashTransaction>,
) -> ZcashResult<()> {
    match params {
        ZcashConsensusParameters::MainNetwork => {
            let mut db_data = z_db_data.sup.main.lock().unwrap();

            match wallet::decrypt_and_store_transaction(
                &params,
                &mut (*db_data),
                &((*tx).clone().into()),
            ) {
                Ok(_) => Ok(()),
                Err(_) => Err(ZcashError::Unknown),
            }
        }

        ZcashConsensusParameters::TestNetwork => {
            let mut db_data = z_db_data.sup.test.lock().unwrap();

            match wallet::decrypt_and_store_transaction(
                &params,
                &mut (*db_data),
                &((*tx).clone().into()),
            ) {
                Ok(_) => Ok(()),
                Err(_) => Err(ZcashError::Unknown),
            }
        }
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

use crate::input_selection::ZcashGreedyInputSelector;
use crate::input_selection::{
    MainGreedyInputSelector, TestGreedyInputSelector, ZcashMainGreedyInputSelector,
    ZcashTestGreedyInputSelector,
};

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
            let mut db_data = z_db_data.sup.main.lock().unwrap();

            let a: ZcashMainGreedyInputSelector = (*input_selector).into();

            match wallet::spend(
                &mut (*db_data),
                &params,
                <ZcashLocalTxProver as Into<LocalTxProver>>::into(prover),
                &<ZcashMainGreedyInputSelector as Into<MainGreedyInputSelector>>::into(a),
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
            let mut db_data = z_db_data.sup.test.lock().unwrap();

            let a: ZcashTestGreedyInputSelector = (*input_selector).into();

            match wallet::spend(
                &mut (*db_data),
                &params,
                <ZcashLocalTxProver as Into<LocalTxProver>>::into(prover),
                &<ZcashTestGreedyInputSelector as Into<TestGreedyInputSelector>>::into(a),
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

// #[cfg(feature = "transparent-inputs")]
// #[allow(clippy::too_many_arguments)]
// #[allow(clippy::type_complexity)]
// pub fn shield_transparent_funds<DbT, ParamsT, InputsT>(
//     wallet_db: &mut DbT,
//     params: &ParamsT,
//     prover: impl SaplingProver,
//     input_selector: &InputsT,
//     shielding_threshold: NonNegativeAmount,
//     usk: &UnifiedSpendingKey,
//     from_addrs: &[TransparentAddress],
//     memo: &MemoBytes,
//     min_confirmations: NonZeroU32,
// ) -> Result<
//     TxId,
//     Error<
//         <DbT as WalletRead>::Error,
//         <DbT as WalletCommitmentTrees>::Error,
//         InputsT::Error,
//         <InputsT::FeeRule as FeeRule>::Error,
//         DbT::NoteRef,
//     >,
// >
// where
//     ParamsT: consensus::Parameters,
//     DbT: WalletWrite + WalletCommitmentTrees,
//     DbT::NoteRef: Copy + Eq + Ord,
//     InputsT: InputSelector<DataSource = DbT>,
// {
