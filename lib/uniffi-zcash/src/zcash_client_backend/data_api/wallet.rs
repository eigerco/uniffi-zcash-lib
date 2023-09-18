// /// Scans a [`Transaction`] for any information that can be decrypted by the accounts in
// /// the wallet, and saves it to the wallet.
// pub fn decrypt_and_store_transaction<ParamsT, DbT>(
//     params: &ParamsT,
//     data: &mut DbT,
//     tx: &Transaction,
// ) -> Result<(), DbT::Error>
// where
//     ParamsT: consensus::Parameters,
//     DbT: WalletWrite,
// {

// /// [`sapling::TxProver`]: zcash_primitives::sapling::prover::TxProver
// #[allow(clippy::too_many_arguments)]
// #[allow(clippy::type_complexity)]
// pub fn spend<DbT, ParamsT, InputsT>(
//     wallet_db: &mut DbT,
//     params: &ParamsT,
//     prover: impl SaplingProver,
//     input_selector: &InputsT,
//     usk: &UnifiedSpendingKey,
//     request: zip321::TransactionRequest,
//     ovk_policy: OvkPolicy,
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
//     DbT: WalletWrite + WalletCommitmentTrees,
//     DbT::NoteRef: Copy + Eq + Ord,
//     ParamsT: consensus::Parameters + Clone,
//     InputsT: InputSelector<DataSource = DbT>,
// {

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
