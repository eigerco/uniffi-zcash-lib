use std::io::Write;

use hdwallet::rand_core::OsRng;
use orchard::{
    builder::{InProgress, Unauthorized, Unproven},
    bundle::Flags,
    keys::Diversifier,
    note::{Nullifier, RandomSeed},
    tree::{MerkleHashOrchard, MerklePath},
    value::NoteValue,
    Note,
};
use zcash_client_backend::keys::UnifiedSpendingKey;
use zcash_primitives::{
    consensus::{BlockHeight, BranchId, MainNetwork},
    legacy::keys::IncomingViewingKey,
    memo::MemoBytes,
    merkle_tree::{CommitmentTree, IncrementalWitness},
    sapling::Node,
    transaction::{
        builder::Builder,
        components::{transparent::fees::OutputView, Amount, OutPoint, TxOut},
        fees::{fixed, zip317},
        Authorized, Transaction, TransactionData, TxVersion,
    },
};

use zcash_proofs::prover::LocalTxProver;

const BLOCK_HEIGHT: u32 = 2030820;

pub fn write_for_transaction<W: Write>(mut file: W, seed: &[u8]) {
    let key = UnifiedSpendingKey::from_seed(&MainNetwork, seed, 0.into()).unwrap();
    transparent_builder_with_nonstandard_fee_example(&mut file, &key);
    transparent_builder_with_standard_fee_example(&mut file, &key);
    transparent_builder_with_zip317_standard_fee_example(&mut file, &key);
    transparent_builder_with_zip317_non_standard_fee_example(&mut file, &key);
    sapling_transaction_general_builder_example(&mut file, &key);
    orchard_transaction(&mut file, &key);
}

pub fn transparent_builder_with_nonstandard_fee_example<W: Write>(
    mut file: W,
    key: &UnifiedSpendingKey,
) {
    let mut builder = Builder::new(MainNetwork, BlockHeight::from_u32(BLOCK_HEIGHT));

    // Transparent data
    let address = key
        .transparent()
        .to_account_pubkey()
        .derive_external_ivk()
        .unwrap()
        .derive_address(0)
        .unwrap();
    let prev_coin = TxOut {
        value: Amount::from_u64(200).unwrap(),
        script_pubkey: address.script(),
    };
    let secret_key = key.transparent().derive_external_secret_key(0).unwrap();
    builder
        .add_transparent_input(secret_key, OutPoint::new([0u8; 32], 1), prev_coin)
        .unwrap();

    builder
        .add_transparent_output(&address, Amount::from_u64(200).unwrap())
        .unwrap();

    let prover = LocalTxProver::bundled(); // This can increase binary size byb 50MB.
    let fee_rule = fixed::FeeRule::non_standard(Amount::zero());

    let (transaction, _) = builder.build(&prover, &fee_rule).unwrap();
    let mut data = Vec::new();
    transaction.write(&mut data).unwrap();
    super::store_bytes(&mut file, "transaction_non_standard_fee", &data).unwrap();
}

pub fn transparent_builder_with_standard_fee_example<W: Write>(
    mut file: W,
    key: &UnifiedSpendingKey,
) {
    let mut builder = Builder::new(MainNetwork, BlockHeight::from_u32(BLOCK_HEIGHT));

    // Transparent data
    let address = key
        .transparent()
        .to_account_pubkey()
        .derive_external_ivk()
        .unwrap()
        .derive_address(0)
        .unwrap();
    let prev_coin = TxOut {
        value: Amount::from_u64(1200).unwrap(),
        script_pubkey: address.script(),
    };
    let secret_key = key.transparent().derive_external_secret_key(0).unwrap();
    builder
        .add_transparent_input(secret_key, OutPoint::new([0u8; 32], 1), prev_coin)
        .unwrap();

    builder
        .add_transparent_output(&address, Amount::from_u64(200).unwrap())
        .unwrap();

    let prover = LocalTxProver::bundled();
    let fee_rule = fixed::FeeRule::standard();

    let (transaction, _) = builder.build(&prover, &fee_rule).unwrap();

    store_tx_id(&mut file, &transaction, "transaction_standard_fee_id");
    store_tx_version(&mut file, &transaction, "transaction_standard_fee_version");
    store_tx_t_out(
        &mut file,
        &transaction,
        0,
        "transaction_standard_fee_vout_0",
    );
    store_tx_t_out_address(
        &mut file,
        &transaction,
        0,
        "transaction_standard_fee_vout_0_recipient_address",
    );
    store_tx_t_out_script_pubkey(
        &mut file,
        &transaction,
        0,
        "transaction_standard_fee_vout_0_script",
    );
    store_tx_t_vin(&mut file, &transaction, 0, "transaction_standard_fee_vin_0");

    let mut data = Vec::new();
    transaction.write(&mut data).unwrap();
    super::store_bytes(&mut file, "transaction_standard_fee", &data).unwrap();
}

fn store_tx_id<W: Write>(mut file: W, tx: &Transaction, label: &str) {
    let mut data = Vec::new();
    tx.txid().write(&mut data).unwrap();
    super::store_bytes(&mut file, label, &data).unwrap();
}

fn store_tx_version<W: Write>(mut file: W, tx: &Transaction, label: &str) {
    let mut data = Vec::new();
    tx.version().write(&mut data).unwrap();
    super::store_bytes(&mut file, label, &data).unwrap();
}

fn store_tx_t_out<W: Write>(mut file: W, tx: &Transaction, idx: usize, label: &str) {
    let mut data = Vec::new();
    tx.transparent_bundle()
        .unwrap()
        .vout
        .get(idx)
        .unwrap()
        .write(&mut data)
        .unwrap();
    super::store_bytes(&mut file, label, &data).unwrap();
}

fn store_tx_t_out_address<W: Write>(mut file: W, tx: &Transaction, idx: usize, label: &str) {
    let data = match tx
        .transparent_bundle()
        .unwrap()
        .vout
        .get(idx)
        .unwrap()
        .recipient_address()
        .unwrap()
    {
        zcash_primitives::legacy::TransparentAddress::PublicKey(pubkey) => pubkey,
        zcash_primitives::legacy::TransparentAddress::Script(script) => script,
    };
    super::store_bytes(&mut file, label, &data).unwrap();
}

fn store_tx_t_out_script_pubkey<W: Write>(mut file: W, tx: &Transaction, idx: usize, label: &str) {
    let mut data = Vec::new();
    tx.transparent_bundle()
        .unwrap()
        .vout
        .get(idx)
        .unwrap()
        .script_pubkey()
        .write(&mut data)
        .unwrap();
    super::store_bytes(&mut file, label, &data).unwrap();
}

fn store_tx_t_vin<W: Write>(mut file: W, tx: &Transaction, idx: usize, label: &str) {
    let mut data = Vec::new();
    tx.transparent_bundle()
        .unwrap()
        .vin
        .to_vec()
        .get(idx)
        .unwrap()
        .write(&mut data)
        .unwrap();
    super::store_bytes(&mut file, label, &data).unwrap();
}

pub fn transparent_builder_with_zip317_standard_fee_example<W: Write>(
    mut file: W,
    key: &UnifiedSpendingKey,
) {
    let mut builder = Builder::new(MainNetwork, BlockHeight::from_u32(BLOCK_HEIGHT));

    // Transparent data
    let address = key
        .transparent()
        .to_account_pubkey()
        .derive_external_ivk()
        .unwrap()
        .derive_address(0)
        .unwrap();
    let prev_coin = TxOut {
        value: Amount::from_u64(19200).unwrap(),
        script_pubkey: address.script(),
    };
    let secret_key = key.transparent().derive_external_secret_key(0).unwrap();
    builder
        .add_transparent_input(secret_key, OutPoint::new([0u8; 32], 1), prev_coin)
        .unwrap();

    builder
        .add_transparent_output(&address, Amount::from_u64(9200).unwrap())
        .unwrap();

    let prover = LocalTxProver::bundled();
    let fee_rule = zip317::FeeRule::standard();

    let (transaction, _) = builder.build(&prover, &fee_rule).unwrap();
    let mut data = Vec::new();
    transaction.write(&mut data).unwrap();
    super::store_bytes(&mut file, "transaction_zip317_standard_fee", &data).unwrap();
}

pub fn transparent_builder_with_zip317_non_standard_fee_example<W: Write>(
    mut file: W,
    key: &UnifiedSpendingKey,
) {
    let mut builder = Builder::new(MainNetwork, BlockHeight::from_u32(BLOCK_HEIGHT));

    // Transparent data
    let address = key
        .transparent()
        .to_account_pubkey()
        .derive_external_ivk()
        .unwrap()
        .derive_address(0)
        .unwrap();
    let prev_coin = TxOut {
        value: Amount::from_u64(19200).unwrap(),
        script_pubkey: address.script(),
    };
    let secret_key = key.transparent().derive_external_secret_key(0).unwrap();
    builder
        .add_transparent_input(secret_key, OutPoint::new([0u8; 32], 1), prev_coin)
        .unwrap();

    builder
        .add_transparent_output(&address, Amount::from_u64(9200).unwrap())
        .unwrap();

    let prover = LocalTxProver::bundled();
    let fee_rule =
        zip317::FeeRule::non_standard(Amount::from_u64(5000).unwrap(), 2, 150, 34).unwrap();

    let (transaction, _) = builder.build(&prover, &fee_rule).unwrap();
    let mut data = Vec::new();
    transaction.write(&mut data).unwrap();
    super::store_bytes(&mut file, "transaction_zip317_non_standard_fee", &data).unwrap();
}

pub fn sapling_transaction_general_builder_example<W: Write>(
    mut file: W,
    key: &UnifiedSpendingKey,
) {
    let mut builder = Builder::new(MainNetwork, BlockHeight::from_u32(BLOCK_HEIGHT));

    let extsk = key.sapling().clone();
    let (_, payment_address) = extsk.default_address();
    let rseed = zcash_primitives::sapling::Rseed::AfterZip212([0u8; 32]);
    let note = payment_address.create_note(200, rseed);
    let mut tree = CommitmentTree::empty();
    tree.append(Node::from_cmu(&note.cmu())).unwrap();
    let witness = IncrementalWitness::from_tree(&tree);

    builder
        .add_sapling_spend(
            extsk,
            *payment_address.diversifier(),
            note,
            witness.path().unwrap(),
        )
        .unwrap();

    builder
        .add_sapling_output(
            Some(
                key.sapling()
                    .to_diversifiable_full_viewing_key()
                    .to_ovk(zcash_primitives::zip32::Scope::Internal),
            ),
            payment_address,
            Amount::from_u64(200).unwrap(),
            MemoBytes::empty(),
        )
        .unwrap();

    let prover = LocalTxProver::bundled();
    let fee_rule = fixed::FeeRule::non_standard(Amount::zero());
    let (transaction, _) = builder.build(&prover, &fee_rule).unwrap();

    let mut data = Vec::new();
    transaction.write(&mut data).unwrap();
    super::store_bytes(&mut file, "transaction_sapling", &data).unwrap();
}

pub fn orchard_transaction<W: Write>(mut file: W, key: &UnifiedSpendingKey) {
    // Key derivation
    let ufvk = key.to_unified_full_viewing_key();
    let fvk = ufvk.orchard().unwrap();
    let ovk = fvk.to_ovk(orchard::keys::Scope::External);
    let address = fvk
        .to_ivk(orchard::keys::Scope::Internal)
        .address(Diversifier::from_bytes([0u8; 11]));

    // Note construction
    let note_value = NoteValue::from_raw(15);
    let nullifier = Nullifier::from_bytes(&[0u8; 32]).unwrap();
    let rseed = RandomSeed::from_bytes([0u8; 32], &nullifier).unwrap();
    let note = Note::from_parts(address, note_value, nullifier, rseed).unwrap();

    let mut auth_path = [MerkleHashOrchard::from_bytes(&[0u8; 32]).unwrap(); 32];
    auth_path[1] = MerkleHashOrchard::from_bytes(&[0u8; 32]).unwrap();

    let merkle_path = MerklePath::from_parts(0, auth_path);
    let anchor = merkle_path.root(note.commitment().into());

    let spends_enabled = true;
    let outputs_enabled = true;

    let mut builder =
        orchard::builder::Builder::new(Flags::from_parts(spends_enabled, outputs_enabled), anchor);

    builder
        .add_recipient(Some(ovk), address, note_value, None)
        .unwrap();

    builder.add_spend(fvk.clone(), note, merkle_path).unwrap();

    let bundle: orchard::Bundle<InProgress<Unproven, Unauthorized>, Amount> =
        builder.build(OsRng).unwrap();

    let pk = orchard::circuit::ProvingKey::build();
    let sighash = [0u8; 32]; // External param
    let proved_bundle = bundle.create_proof(&pk, OsRng).unwrap();
    let authorized_bundle = proved_bundle
        .apply_signatures(OsRng, sighash, &[key.orchard().into()])
        .unwrap();

    let consensus_branch_id =
        BranchId::for_height(&MainNetwork, BlockHeight::from_u32(BLOCK_HEIGHT));
    let version = TxVersion::suggested_for_branch(consensus_branch_id);

    let transaction_data: TransactionData<Authorized> = TransactionData::from_parts(
        version,
        consensus_branch_id,
        23,
        BlockHeight::from_u32(BLOCK_HEIGHT + 100),
        None,
        None,
        None,
        Some(authorized_bundle),
    );

    let transaction = transaction_data.freeze().unwrap();

    let mut data = Vec::new();
    transaction.write(&mut data).unwrap();
    super::store_bytes(&mut file, "transaction_orchard", &data).unwrap();
}
