use std::io::Write;

use group::GroupEncoding;
use hdwallet::ExtendedPrivKey;
use zcash_client_backend::encoding;
use zcash_primitives::{
    consensus::MainNetwork,
    legacy::keys::{AccountPrivKey, IncomingViewingKey}, memo::MemoBytes, zip32::{ExtendedSpendingKey, Scope}, sapling::{Diversifier, keys::ExpandedSpendingKey},
};

use super::format_bytes;

#[rustfmt::skip]
pub fn write_for_zcash_primitives<W: Write>(mut file: W, seed: &[u8]) {
    let apk = AccountPrivKey::from_seed(&MainNetwork, seed, 0.into()).unwrap();
    writeln!(file, "{}", format_bytes("account_private_key", &apk.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("apk_derive_external_secret_key", &apk.derive_external_secret_key(0).unwrap().serialize_secret())).unwrap();
    writeln!(file, "{}", format_bytes("apk_derive_internal_secret_key", &apk.derive_internal_secret_key(0).unwrap().serialize_secret())).unwrap();

    let ppk = apk.to_account_pubkey();
    writeln!(file, "{}", format_bytes("account_public_key", &ppk.serialize())).unwrap();

    let external_ivk = ppk.derive_external_ivk().unwrap();
    writeln!(file, "{}", format_bytes("ppk_external_ivk", &external_ivk.serialize())).unwrap();

    let (default_address_address, default_address_index) = external_ivk.default_address();
    writeln!(file, "external_ivk_default_address_address:{}", encoding::encode_transparent_address_p(&MainNetwork, &default_address_address)).unwrap();
    writeln!(file, "external_ivk_default_address_index:{}", default_address_index).unwrap();

    let internal_ivk = ppk.derive_internal_ivk().unwrap();
    writeln!(file, "{}", format_bytes("ppk_internal_ivk", &internal_ivk.serialize())).unwrap();

    let (default_address_address, default_address_index) = internal_ivk.default_address();
    writeln!(file, "internal_ivk_default_address_address:{}", encoding::encode_transparent_address_p(&MainNetwork, &default_address_address)).unwrap();
    writeln!(file, "internal_ivk_default_address_index:{}", default_address_index).unwrap();

    writeln!(file, "{}", format_bytes("ppk_external_ovk", &ppk.external_ovk().as_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("ppk_internal_ovk", &ppk.internal_ovk().as_bytes())).unwrap();

    let extended_priv_key = ExtendedPrivKey::with_seed(seed).unwrap();
    let extended_private_key = AccountPrivKey::from_extended_privkey(extended_priv_key);
    writeln!(file, "{}", format_bytes("extended_private_key", &extended_private_key.to_bytes())).unwrap();

    writeln!(file, "t_address_public_key:tm9iMLAuYMzJ6jtFLcA7rzUmfreGuKvr7Ma").unwrap();
    writeln!(file, "t_address_script:t26YoyZ1iPgiMEWL4zGUm74eVWfhyDMXzY2").unwrap();

    let memo_bytes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    writeln!(file, "{}", format_bytes("memo_bytes", &memo_bytes)).unwrap();

    let memo_bytes = MemoBytes::from_bytes(&memo_bytes).unwrap();
    writeln!(file, "{}", format_bytes("memo_data", memo_bytes.as_slice())).unwrap();

    let memo_bytes_too_long = [0; 520];
    writeln!(file, "{}", format_bytes("memo_bytes_too_long", &memo_bytes_too_long)).unwrap();

    let memo_empty = MemoBytes::empty();
    writeln!(file, "{}", format_bytes("memo_empty", &memo_empty.as_slice())).unwrap();

    let extended_spending_key = ExtendedSpendingKey::master(seed);
    let diversifiable_fvk = extended_spending_key.to_diversifiable_full_viewing_key();
    let diversifiable_fvk_fvk = diversifiable_fvk.fvk().to_bytes();
    writeln!(file, "{}", format_bytes("diversifiable_fvk_fvk", &diversifiable_fvk_fvk)).unwrap();

    let diversifiable_fvk_nk = diversifiable_fvk.to_nk(Scope::External).0.to_bytes();
    writeln!(file, "{}", format_bytes("diversifiable_fvk_nk", &diversifiable_fvk_nk)).unwrap();

    let diversifiable_fvk_ivk = diversifiable_fvk.to_ivk(Scope::External).to_repr();
    writeln!(file, "{}", format_bytes("diversifiable_fvk_ivk", &diversifiable_fvk_ivk)).unwrap();

    let diversifiable_fvk_ovk = diversifiable_fvk.to_ovk(Scope::External).0;
    writeln!(file, "{}", format_bytes("diversifiable_fvk_ovk", &diversifiable_fvk_ovk)).unwrap();

    let diversifiable_fvk_address = encoding::encode_payment_address_p(&MainNetwork, &diversifiable_fvk.address(1u32.into()).unwrap());
    writeln!(file, "diversifiable_fvk_address:{}", &diversifiable_fvk_address).unwrap();

    let (dfvk_find_address_index, dfvk_find_address_address)  = diversifiable_fvk.find_address(1u32.into()).unwrap();
    writeln!(file, "{}", format_bytes("dfvk_find_address_index", &dfvk_find_address_index.0)).unwrap();

    let dfvk_find_address_address = encoding::encode_payment_address_p(&MainNetwork, &dfvk_find_address_address);
    writeln!(file, "dfvk_find_address_address:{}", &dfvk_find_address_address).unwrap();

    let (dfvk_default_address_index, dfvk_default_address_address)  = diversifiable_fvk.default_address();
    writeln!(file, "{}", format_bytes("dfvk_default_address_index", &dfvk_default_address_index.0)).unwrap();

    let dfvk_default_address_address = encoding::encode_payment_address_p(&MainNetwork, &dfvk_default_address_address);
    writeln!(file, "dfvk_default_address_address:{}", &dfvk_default_address_address).unwrap();

    let diversifier = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let dfvk_diversified_address = diversifiable_fvk.diversified_address(Diversifier(diversifier)).unwrap();
    let dfvk_diversified_address = encoding::encode_payment_address_p(&MainNetwork, &dfvk_diversified_address);
    writeln!(file, "dfvk_diversified_address:{}", &dfvk_diversified_address).unwrap();

    let (dfvk_change_address_index, dfvk_change_address_address) = diversifiable_fvk.change_address();
    writeln!(file, "{}", format_bytes("dfvk_change_address_index", &dfvk_change_address_index.0)).unwrap();

    let dfvk_change_address_address = encoding::encode_payment_address_p(&MainNetwork, &dfvk_change_address_address);
    writeln!(file, "dfvk_change_address_address:{}", &dfvk_change_address_address).unwrap();

    let dfvk_diversified_change_address = diversifiable_fvk.diversified_change_address(Diversifier(diversifier)).unwrap();
    let dfvk_diversified_change_address = encoding::encode_payment_address_p(&MainNetwork, &dfvk_diversified_change_address);
    writeln!(file, "dfvk_diversified_change_address:{}", &dfvk_diversified_change_address).unwrap();

    let address = diversifiable_fvk.default_address().1;
    let (dfvk_decrypt_diversifier, scope) = diversifiable_fvk.decrypt_diversifier(&address).unwrap();
    writeln!(file, "{}", format_bytes("dfvk_decrypt_diversifier", &dfvk_decrypt_diversifier.0)).unwrap();
    assert_eq!(scope, Scope::External);

    let expanded_sk = ExpandedSpendingKey::from_spending_key(&extended_spending_key.to_bytes());
    writeln!(file, "{}", format_bytes("expanded_spending_key", &expanded_sk.to_bytes())).unwrap();

    expanded_sk.proof_generation_key();
    /*
    let apk = AccountPrivKey::from_seed(&MainNetwork, &seed, 0.into()).unwrap();
    let ppk = apk.to_account_pubkey();



    let diversifier = Diversifier([0; 11]);












    writeln!(file, "{}", format_bytes("sapling_address", &get_sapling_address(&usk).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_diversifier", orchard_diversifier.as_array())).unwrap();

    writeln!(file, "{}", format_bytes("account_private_key", &apk.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("account_public_key", &ppk.serialize())).unwrap();




/*
    writeln!(file, "{}", format_bytes("sapling_full_viewing_key", &sapling_fvk_from_expsk.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("sapling_full_viewing_key_vk", &sapling_fvk_from_expsk.vk.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("sapling_full_viewing_key_vk_ak", &sapling_fvk_from_expsk.vk.ak.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("sapling_full_viewing_key_vk_nk", &sapling_fvk_from_expsk.vk.nk.0.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("sapling_full_viewing_key_ovk", &sapling_fvk_from_expsk.ovk.0)).unwrap();

    writeln!(file, "{}", format_bytes("extended_fvk", &sapling_fvk.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_encoded", &sapling_fvk.encode(&MainNetwork))).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_child", &sapling_fvk.derive_child(ZcashChildIndex::Hardened(32)).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_address", &sapling_fvk.address(ZcashDiversifierIndex::from_u32(4)).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_find_address", &sapling_fvk.find_address(ZcashDiversifierIndex::from_u32(0)).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_default_address", &sapling_fvk.default_address().to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_derive_internal", &sapling_fvk.derive_internal().to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_diversifiable_fvk", &sapling_fvk.to_diversifiable_full_viewing_key().to_bytes())).unwrap();

    writeln!(file, "{}", format_bytes("extended_spending_key_fvk_nk", &diversifiable_fvk.to_nk(ZcashScope::EXTERNAL).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_fvk_ivk", &diversifiable_fvk.to_ivk(ZcashScope::EXTERNAL).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_fvk_ovk", &diversifiable_fvk.to_ovk(ZcashScope::EXTERNAL).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_fvk_addr", &diversifiable_fvk.address(&diversifier).to_bytes())).unwrap();

    writeln!(file, "{}", format_bytes("sapling_outgoing_viewing_key", &get_ovk(&usk))).unwrap();



    */ */
}

/*
fn get_ovk(key: &UnifiedSpendingKey) -> [u8; 32] {
    key.to_unified_full_viewing_key()
        .sapling()
        .unwrap()
        .to_ovk(zcash_primitives::zip32::Scope::External)
        .0
}

fn get_sapling_address(key: &UnifiedSpendingKey) -> PaymentAddress {
    let diversifier = zcash_primitives::sapling::Diversifier([0; 11]);

    key.to_unified_full_viewing_key()
        .sapling()
        .unwrap()
        .to_ivk(zcash_primitives::zip32::Scope::External)
        .to_payment_address(diversifier)
        .unwrap()
}
*/
