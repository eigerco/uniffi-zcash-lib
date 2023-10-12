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
    ZcashLocalTxProver, ZcashMainGreedyInputSelector, ZcashMemoBytes, ZcashNonNegativeAmount,
    ZcashOvkPolicy, ZcashResult, ZcashTestGreedyInputSelector, ZcashTransaction,
    ZcashTransactionRequest, ZcashTransparentAddress, ZcashTxId, ZcashUnifiedSpendingKey,
    ZcashWalletDb,
};

#[derive(Default)]
pub struct WalletDefault;

impl WalletDefault {
    pub fn new() -> Self {
        Self
    }
    /// Scans a [`Transaction`] for any information that can be decrypted by the accounts in
    /// the wallet, and saves it to the wallet.
    pub fn decrypt_and_store_transaction(
        &self,
        params: ZcashConsensusParameters,
        z_db_data: Arc<ZcashWalletDb>,
        tx: Arc<ZcashTransaction>,
    ) -> ZcashResult<()> {
        let mut db_data = WalletDb::for_path(&z_db_data.path, params).unwrap();

        match wallet::decrypt_and_store_transaction(&params, &mut db_data, &((*tx).clone().into()))
        {
            Ok(_) => Ok(()),
            Err(x) => Err(ZcashError::Message {
                error: format!("decrypt and store transaction error: {:?}", x),
            }),
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    pub fn spend_main(
        &self,
        z_db_data: Arc<ZcashWalletDb>,
        params: ZcashConsensusParameters,
        prover: Arc<ZcashLocalTxProver>,
        input_selector: Arc<ZcashMainGreedyInputSelector>,
        usk: Arc<ZcashUnifiedSpendingKey>,
        request: Arc<ZcashTransactionRequest>,
        ovk_policy: ZcashOvkPolicy,
        min_confirmations: u32,
    ) -> ZcashResult<Arc<ZcashTxId>> {
        let min_confirmations = NonZeroU32::new(min_confirmations).unwrap();

        let mut db_data = WalletDb::for_path(&z_db_data.path, consensus::MAIN_NETWORK)
            .expect("Cannot unwrap db_data!");

        match wallet::spend(
            &mut db_data,
            &params,
            <ZcashLocalTxProver as Into<LocalTxProver>>::into((*prover).clone()),
            &<ZcashMainGreedyInputSelector as Into<MainGreedyInputSelector>>::into(
                (*input_selector).clone(),
            ),
            &((*usk).clone().into()),
            (*request).clone().into(),
            ovk_policy.into(),
            min_confirmations,
        ) {
            Ok(txid) => {
                let x: ZcashTxId = txid.into();
                Ok(Arc::new(x))
            }
            Err(x) => Err(ZcashError::Message {
                error: format!("spending error (spend_main): {:?}", x),
            }),
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    pub fn spend_test(
        &self,
        z_db_data: Arc<ZcashWalletDb>,
        params: ZcashConsensusParameters,
        prover: Arc<ZcashLocalTxProver>,
        input_selector: Arc<ZcashTestGreedyInputSelector>,
        usk: Arc<ZcashUnifiedSpendingKey>,
        request: Arc<ZcashTransactionRequest>,
        ovk_policy: ZcashOvkPolicy,
        min_confirmations: u32,
    ) -> ZcashResult<Arc<ZcashTxId>> {
        let min_confirmations = NonZeroU32::new(min_confirmations).unwrap();

        let mut db_data = WalletDb::for_path(&z_db_data.path, consensus::TEST_NETWORK)
            .expect("Cannot unwrap db_data!");

        match wallet::spend(
            &mut db_data,
            &params,
            <ZcashLocalTxProver as Into<LocalTxProver>>::into((*prover).clone()),
            &<ZcashTestGreedyInputSelector as Into<TestGreedyInputSelector>>::into(
                (*input_selector).clone(),
            ),
            &((*usk).clone().into()),
            (*request).clone().into(),
            ovk_policy.into(),
            min_confirmations,
        ) {
            Ok(txid) => {
                let x: ZcashTxId = txid.into();
                Ok(Arc::new(x))
            }
            Err(x) => Err(ZcashError::Message {
                error: format!("spending error (spend test): {:?}", x),
            }),
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    pub fn shield_transparent_funds_main(
        &self,
        z_db_data: Arc<ZcashWalletDb>,
        params: ZcashConsensusParameters,
        prover: Arc<ZcashLocalTxProver>,
        input_selector: Arc<ZcashMainGreedyInputSelector>,
        shielding_threshold: u64,
        usk: Arc<ZcashUnifiedSpendingKey>,
        from_addrs: Vec<Arc<ZcashTransparentAddress>>,
        memo: Arc<ZcashMemoBytes>,
        min_confirmations: u32,
    ) -> ZcashResult<Arc<ZcashTxId>> {
        let min_confirmations = NonZeroU32::new(min_confirmations).unwrap();
        let shielding_threshold = ZcashNonNegativeAmount::from_u64(shielding_threshold).unwrap();
        let addresses = from_addrs
            .iter()
            .map(|x| x.as_ref().into())
            .collect::<Vec<TransparentAddress>>();

        let mut db_data = WalletDb::for_path(&z_db_data.path, consensus::MAIN_NETWORK).unwrap();

        match wallet::shield_transparent_funds(
            &mut db_data,
            &params,
            <ZcashLocalTxProver as Into<LocalTxProver>>::into((*prover).clone()),
            &<ZcashMainGreedyInputSelector as Into<MainGreedyInputSelector>>::into(
                (*input_selector).clone(),
            ),
            shielding_threshold.into(),
            &<ZcashUnifiedSpendingKey as Into<UnifiedSpendingKey>>::into((*usk).clone()),
            &addresses[..],
            &((*memo).clone().into()),
            min_confirmations,
        ) {
            Ok(txid) => {
                let x: ZcashTxId = txid.into();
                Ok(Arc::new(x))
            }
            Err(x) => Err(ZcashError::Message {
                error: format!("spending error: {:?}", x),
            }),
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    pub fn shield_transparent_funds_test(
        &self,
        z_db_data: Arc<ZcashWalletDb>,
        params: ZcashConsensusParameters,
        prover: Arc<ZcashLocalTxProver>,
        input_selector: Arc<ZcashTestGreedyInputSelector>,
        shielding_threshold: u64,
        usk: Arc<ZcashUnifiedSpendingKey>,
        from_addrs: Vec<Arc<ZcashTransparentAddress>>,
        memo: Arc<ZcashMemoBytes>,
        min_confirmations: u32,
    ) -> ZcashResult<Arc<ZcashTxId>> {
        let min_confirmations = NonZeroU32::new(min_confirmations).unwrap();
        let shielding_threshold = ZcashNonNegativeAmount::from_u64(shielding_threshold).unwrap();
        let addresses = from_addrs
            .iter()
            .map(|x| x.as_ref().into())
            .collect::<Vec<TransparentAddress>>();

        let mut db_data = WalletDb::for_path(&z_db_data.path, consensus::TEST_NETWORK).unwrap();

        match wallet::shield_transparent_funds(
            &mut db_data,
            &params,
            <ZcashLocalTxProver as Into<LocalTxProver>>::into((*prover).clone()),
            &<ZcashTestGreedyInputSelector as Into<TestGreedyInputSelector>>::into(
                (*input_selector).clone(),
            ),
            shielding_threshold.into(),
            &<ZcashUnifiedSpendingKey as Into<UnifiedSpendingKey>>::into((*usk).clone()),
            &addresses[..],
            &((*memo).clone().into()),
            min_confirmations,
        ) {
            Ok(txid) => {
                let x: ZcashTxId = txid.into();
                Ok(Arc::new(x))
            }
            Err(x) => Err(ZcashError::Message {
                error: format!("spending error: {:?}", x),
            }),
        }
    }
}
