use std::io::Write;

use hdwallet::ExtendedPrivKey;
use zcash_primitives::{
    consensus::MainNetwork,
    legacy::keys::{AccountPrivKey, IncomingViewingKey},
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
    writeln!(file, "{}", format_bytes("ppk_external_ivk", &ppk.derive_external_ivk().unwrap().serialize())).unwrap();
    writeln!(file, "{}", format_bytes("ppk_internal_ivk", &ppk.derive_internal_ivk().unwrap().serialize())).unwrap();
    writeln!(file, "{}", format_bytes("ppk_external_ovk", &ppk.external_ovk().as_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("ppk_internal_ovk", &ppk.internal_ovk().as_bytes())).unwrap();

    let extended_priv_key = ExtendedPrivKey::with_seed(seed).unwrap();
    let extended_private_key = AccountPrivKey::from_extended_privkey(extended_priv_key);
    writeln!(file, "{}", format_bytes("extended_private_key", &extended_private_key.to_bytes())).unwrap();

    writeln!(file, "t_address_public_key:tm9iMLAuYMzJ6jtFLcA7rzUmfreGuKvr7Ma").unwrap();
    writeln!(file, "t_address_script:t26YoyZ1iPgiMEWL4zGUm74eVWfhyDMXzY2").unwrap();
    /*
    let apk = AccountPrivKey::from_seed(&MainNetwork, &seed, 0.into()).unwrap();
    let ppk = apk.to_account_pubkey();



    let diversifier = Diversifier([0; 11]);




    let expanded_sk = ExpandedSpendingKey::from_spending_key(&extended_spending_key.to_bytes());



    let sapling_fvk_from_expsk = FullViewingKey::from_expanded_spending_key(&expanded_sk);

    let sapling_fvk = diversifiable_fvk.fvk();




    writeln!(file, "{}", format_bytes("sapling_address", &get_sapling_address(&usk).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_diversifier", orchard_diversifier.as_array())).unwrap();

    writeln!(file, "{}", format_bytes("account_private_key", &apk.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("account_public_key", &ppk.serialize())).unwrap();




    writeln!(file, "{}", format_bytes("expanded_spending_key", &expanded_sk.to_bytes())).unwrap();

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
