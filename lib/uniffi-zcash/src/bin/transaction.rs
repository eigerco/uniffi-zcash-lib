use zcash_client_backend::keys::UnifiedSpendingKey;
use zcash_primitives::{
    consensus::{BlockHeight, MainNetwork},
    legacy::keys::IncomingViewingKey,
    memo::MemoBytes,
    merkle_tree::{CommitmentTree, IncrementalWitness},
    sapling::Node,
    transaction::{
        builder::Builder,
        components::{Amount, OutPoint, TxOut},
        fees::fixed,
    },
    zip32::Scope,
};
use zcash_proofs::prover::LocalTxProver;

const BLOCK_HEIGHT: u32 = 2030820;

fn main() {
    let mut seed = vec![0u8; 32];
    seed[0] = 1u8;
    let key = UnifiedSpendingKey::from_seed(&MainNetwork, &seed, 0.into()).unwrap();

    transparent_transaction_general_builder_example(&key);
    println!();
    sapling_transaction_general_builder_example(&key);
}

pub fn transparent_transaction_general_builder_example(key: &UnifiedSpendingKey) {
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
    println!("Transparent transaction data (from general builder)");
    println!("___________________________________________________");
    println!(
        "[{}]",
        data.iter()
            .map(|byte| byte.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
}

pub fn sapling_transaction_general_builder_example(key: &UnifiedSpendingKey) {
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
                    .to_ovk(Scope::Internal),
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
    println!("Sapling transaction data (from general builder)");
    println!("_______________________________________________");
    println!(
        "[{}]",
        data.iter()
            .map(|byte| byte.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
}
