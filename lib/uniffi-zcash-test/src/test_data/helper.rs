use std::io::Write;

use zcash_primitives::transaction::{components::transparent::fees::OutputView, Transaction};

pub fn store_tx_t_id<W: Write>(mut file: W, tx: &Transaction) {
    let mut data = Vec::new();
    tx.txid().write(&mut data).unwrap();
    super::store_bytes(&mut file, "transaction_standard_fee_id", &data).unwrap();
}

pub fn store_tx_t_version<W: Write>(mut file: W, tx: &Transaction) {
    let mut data = Vec::new();
    tx.version().write(&mut data).unwrap();
    super::store_bytes(&mut file, "transaction_standard_fee_version", &data).unwrap();
}

pub fn store_tx_t_out<W: Write>(mut file: W, tx: &Transaction, idx: usize) {
    let mut data = Vec::new();
    tx.transparent_bundle()
        .unwrap()
        .vout
        .get(idx)
        .unwrap()
        .write(&mut data)
        .unwrap();
    let label = format!("transaction_standard_fee_vout_{}", idx);
    super::store_bytes(&mut file, &label, &data).unwrap();
}

pub fn store_tx_t_out_address<W: Write>(mut file: W, tx: &Transaction, idx: usize) {
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
    let label = format!("transaction_standard_fee_vout_{}_recipient_address", idx);
    super::store_bytes(&mut file, &label, &data).unwrap();
}

pub fn store_tx_t_out_script_pubkey<W: Write>(mut file: W, tx: &Transaction, idx: usize) {
    let mut data = Vec::new();
    tx.transparent_bundle()
        .unwrap()
        .vout
        .get(idx)
        .unwrap()
        .script_pubkey()
        .write(&mut data)
        .unwrap();
    let label = format!("transaction_standard_fee_vout_{}_script", idx);
    super::store_bytes(&mut file, &label, &data).unwrap();
}

pub fn store_tx_t_vin<W: Write>(mut file: W, tx: &Transaction, idx: usize) {
    let mut data = Vec::new();
    tx.transparent_bundle()
        .unwrap()
        .vin
        .to_vec()
        .get(idx)
        .unwrap()
        .write(&mut data)
        .unwrap();
    let label = format!("transaction_standard_fee_vin_{}", idx);
    super::store_bytes(&mut file, &label, &data).unwrap();
}

pub fn store_tx_sapling_spend_cv<W: Write>(mut file: W, tx: &Transaction, idx: usize) {
    let data = tx
        .sapling_bundle()
        .unwrap()
        .shielded_spends()
        .get(idx)
        .unwrap()
        .cv()
        .to_bytes();
    let label = format!("transaction_sapling_spend_{}_cv", idx);
    super::store_bytes(&mut file, &label, &data).unwrap();
}

pub fn store_tx_sapling_spend_anchor<W: Write>(mut file: W, tx: &Transaction, idx: usize) {
    let data = tx
        .sapling_bundle()
        .unwrap()
        .shielded_spends()
        .get(idx)
        .unwrap()
        .anchor()
        .to_bytes();
    let label = format!("transaction_sapling_spend_{}_anchor", idx);
    super::store_bytes(&mut file, &label, &data).unwrap();
}

pub fn store_tx_sapling_spend_nullifier<W: Write>(mut file: W, tx: &Transaction, idx: usize) {
    let data = tx
        .sapling_bundle()
        .unwrap()
        .shielded_spends()
        .get(idx)
        .unwrap()
        .nullifier()
        .to_vec();
    let label = format!("transaction_sapling_spend_{}_nullifier", idx);
    super::store_bytes(&mut file, &label, &data).unwrap();
}

pub fn store_tx_sapling_spend_rk<W: Write>(mut file: W, tx: &Transaction, idx: usize) {
    let mut data = Vec::new();
    tx.sapling_bundle()
        .unwrap()
        .shielded_spends()
        .get(idx)
        .unwrap()
        .rk()
        .write(&mut data)
        .unwrap();
    let label = format!("transaction_sapling_spend_{}_rk", idx);
    super::store_bytes(&mut file, &label, &data).unwrap();
}

pub fn store_tx_sapling_output_cv<W: Write>(mut file: W, tx: &Transaction, idx: usize) {
    let data = tx
        .sapling_bundle()
        .unwrap()
        .shielded_outputs()
        .get(idx)
        .unwrap()
        .cv()
        .to_bytes();
    let label = format!("transaction_sapling_output_{}_cv", idx);
    super::store_bytes(&mut file, &label, &data).unwrap();
}

pub fn store_tx_sapling_output_cmu<W: Write>(mut file: W, tx: &Transaction, idx: usize) {
    let data = tx
        .sapling_bundle()
        .unwrap()
        .shielded_outputs()
        .get(idx)
        .unwrap()
        .cmu()
        .to_bytes();
    let label = format!("transaction_sapling_output_{}_cmu", idx);
    super::store_bytes(&mut file, &label, &data).unwrap();
}

pub fn store_tx_orchard_action_nullifier<W: Write>(mut file: W, tx: &Transaction, idx: usize) {
    let data = tx
        .orchard_bundle()
        .unwrap()
        .actions()
        .get(idx)
        .unwrap()
        .nullifier()
        .to_bytes();
    let label = format!("transaction_orchard_action_{}_nullifier", idx);
    super::store_bytes(&mut file, &label, &data).unwrap();
}

pub fn store_tx_orchard_action_cmx<W: Write>(mut file: W, tx: &Transaction, idx: usize) {
    let data = tx
        .orchard_bundle()
        .unwrap()
        .actions()
        .get(idx)
        .unwrap()
        .cmx()
        .to_bytes();
    let label = format!("transaction_orchard_action_{}_cmx", idx);
    super::store_bytes(&mut file, &label, &data).unwrap();
}

pub fn store_tx_orchard_action_encrypted_note<W: Write>(mut file: W, tx: &Transaction, idx: usize) {
    let encrypted_note = tx
        .orchard_bundle()
        .unwrap()
        .actions()
        .get(idx)
        .unwrap()
        .encrypted_note();

    super::store_bytes(
        &mut file,
        format!("transaction_orchard_action_{}_encrypted_note_epk_bytes", idx).as_str(),
        &encrypted_note.epk_bytes,
    )
    .unwrap();
    super::store_bytes(
        &mut file,
        format!(
            "transaction_orchard_action_{}_encrypted_note_enc_ciphertext",
            idx
        )
        .as_str(),
        &encrypted_note.enc_ciphertext,
    )
    .unwrap();
    super::store_bytes(
        &mut file,
        format!(
            "transaction_orchard_action_{}_encrypted_note_out_ciphertext",
            idx
        )
        .as_str(),
        &encrypted_note.out_ciphertext,
    )
    .unwrap();
}

pub fn store_tx_orchard_action_cv_net<W: Write>(mut file: W, tx: &Transaction, idx: usize) {
    let data = tx
        .orchard_bundle()
        .unwrap()
        .actions()
        .get(idx)
        .unwrap()
        .cv_net()
        .to_bytes();
    let label = format!("transaction_orchard_action_{}_cv_net", idx);
    super::store_bytes(&mut file, &label, &data).unwrap();
}

pub fn store_tx_orchard_flags<W: Write>(mut file: W, tx: &Transaction) {
    let data = [tx.orchard_bundle().unwrap().flags().to_byte()];
    super::store_bytes(&mut file, "transaction_orchard_flags", &data).unwrap();
}

pub fn store_tx_orchard_anchor<W: Write>(mut file: W, tx: &Transaction) {
    let data = tx.orchard_bundle().unwrap().anchor().to_bytes();
    super::store_bytes(&mut file, "transaction_orchard_anchor", &data).unwrap();
}
