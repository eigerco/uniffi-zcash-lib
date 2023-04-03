use hdwallet::ExtendedPrivKey;
use orchard::keys::SpendingKey;
use std::fs::OpenOptions;
use std::io::Write;
use zcash_client_backend::keys::{Era, UnifiedSpendingKey};
use zcash_primitives::sapling::PaymentAddress;
use zcash_primitives::zip32::{ChildIndex, ExtendedSpendingKey};
use zcash_primitives::{consensus::MainNetwork, legacy::keys::AccountPrivKey};

#[rustfmt::skip]
fn main() {
    let base_url = env!("CARGO_MANIFEST_DIR");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!("{base_url}/tests/test_data.csv"))
        .unwrap();

    let mut seed = vec![0u8; 32];
    seed[0] = 1u8;

    let usk = UnifiedSpendingKey::from_seed(&MainNetwork, &seed, 0.into()).unwrap();

    // Obtaining from original API expected byte array results
    // for derivation from ExtendedPrivKey to AccountPrivKey.
    let extended_priv_key = ExtendedPrivKey::with_seed(&seed).unwrap();

    let apk = AccountPrivKey::from_seed(&MainNetwork, &seed, 0.into()).unwrap();
    let ppk = apk.to_account_pubkey();
    let secp_secret_key = apk.derive_external_secret_key(0);
    let extended_private_key = AccountPrivKey::from_extended_privkey(extended_priv_key);

    let encoded = usk.to_unified_full_viewing_key().encode(&MainNetwork);

    // https://github.com/eigerco/uniffi-zcash-lib/blob/main/STATUS.md#zcashextendedprivkey

    // https://github.com/eigerco/uniffi-zcash-lib/blob/main/STATUS.md#zcashaccountpubkey

    // https://github.com/eigerco/uniffi-zcash-lib/blob/main/STATUS.md#zcashunifiedspendingkey

    // https://github.com/eigerco/uniffi-zcash-lib/blob/main/STATUS.md#zcashunifiedfullviewingkey

    // https://github.com/eigerco/uniffi-zcash-lib/blob/main/STATUS.md#zcashunifiedaddress

    // https://github.com/eigerco/uniffi-zcash-lib/blob/main/STATUS.md#zcashtransparentaddress

    // https://github.com/eigerco/uniffi-zcash-lib/blob/main/STATUS.md#zcashdiversifiablefullviewingkey-sapling

    // https://github.com/eigerco/uniffi-zcash-lib/blob/main/STATUS.md#zcashextendedspendingkey-sapling

    let extended_spending_key = ExtendedSpendingKey::master(&seed);
    let (ext_sk_child_index, ext_sk_default_address) = extended_spending_key.default_address();

    let coin_type = 234;
    let account_number = 2345;
    let orchard_sk = SpendingKey::from_zip32_seed(&seed, coin_type, account_number).unwrap();
    let diversifier = ZcashDiversifier::new([0; 11]);
    let orchard_diversifier = orchard::keys::Diversifier::from_bytes([0; 11]);


    let diversifiable_fvk = extended_spending_key.to_diversifiable_full_viewing_key();
    let sapling_fvk = diversifiable_fvk.fvk();

    let expanded_sk = ZcashExpandedSpendingKey::from_spending_key(&extended_spending_key.to_bytes());

    let orchard_div_idx_u32 = 4;
    let orchard_div_idx_u64 = u32::MAX + 1;
    let orchard_div_idx_u32_obj = ZcashOrchardDiversifierIndex::from_u32(orchard_div_idx_u32);
    let orchard_div_idx_u64_obj = ZcashOrchardDiversifierIndex::from_u64(orchard_div_idx_u64);

    let orchard_address = get_orchard_address(&usk);

    let sapling_fvk_from_expsk = FullViewingKey::from_expanded_spending_key(&expanded_sk)

    writeln!(file, "{}", format_bytes("seed", &seed)).unwrap();
    writeln!(file, "coin_type:{coin_type}").unwrap();
    writeln!(file, "account:{account_number}").unwrap();
    writeln!(file, "scope:External").unwrap();
    writeln!(file, "t_address_public_key:tm9iMLAuYMzJ6jtFLcA7rzUmfreGuKvr7Ma").unwrap();
    writeln!(file, "t_address_script:t26YoyZ1iPgiMEWL4zGUm74eVWfhyDMXzY2").unwrap();
    writeln!(file, "unified_full_viewing_key_encoded:{encoded}").unwrap();
    writeln!(file, "{}", format_bytes("sapling_address", &get_sapling_address(&usk).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_address", &orchard_address.as_slice())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_diversifier", &orchard_diversifier.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("unified_spending_key", &usk.to_bytes(Era::Orchard))).unwrap();
    writeln!(file, "{}", format_bytes("account_private_key", &apk.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("account_public_key", &ppk.serialize())).unwrap();
    writeln!(file, "{}", format_bytes("ppk_external_ivk", &ppk.derive_external_ivk().as_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("ppk_internal_ivk", &ppk.derive_internal_ivk().as_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("ppk_external_ovk", &ppk.external_ovk().as_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("ppk_internal_ovk", &ppk.internal_ovk().as_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_private_key", &extended_private_key.to_bytes())).unwrap();

    writeln!(file, "{}", format_bytes("extended_spending_key", &extended_spending_key.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_from_path", &get_ext_sk_from_path(&extended_spending_key).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_derived_child", &extended_spending_key.derive_child(ChildIndex::Hardened(32)).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_default_address", &ext_sk_default_address.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_child_index", &ext_sk_child_index.0)).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_internal_sk", &extended_spending_key.derive_internal().to_bytes())).unwrap();

    writeln!(file, "{}", format_bytes("expanded_spending_key", &expanded_sk.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("diversifiable_fvk", &diversifiable_fvk.to_bytes())).unwrap();

    writeln!(file, "{}", format_bytes("sapling_full_viewing_key", &sapling_fvk_from_expsk.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("sapling_full_viewing_key_vk", &sapling_fvk_from_expsk.vk().to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("sapling_full_viewing_key_ovk", &sapling_fvk_from_expsk.ovk().to_bytes())).unwrap();

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
    writeln!(file, "{}", format_bytes("orchard_spending_key", &usk.orchard().to_bytes().as_slice())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_spending_key_from_zip32_seed", &orchard_sk.to_bytes().as_slice())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_full_viewing_key", &orchard_sk.to_fvk().to_bytes().as_slice())).unwrap();

    writeln!(file, "{}", format_bytes("orchard_div_idx_address", &orchard_sk.to_fvk().address(&orchard_diversifier).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_div_idx_address_at", &orchard_sk.to_fvk().address_at(ZcashDiversifierIndex::from_u32(4), ZcashOrchardScope::EXTERNAL).to_bytes() )).unwrap();
    writeln!(file, "{}", format_bytes("orchard_full_viewing_key_ivk", &orchard_sk.to_fvk().to_ivk(ZcashOrchardScope::EXTERNAL).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_full_viewing_key_ovk", &orchard_sk.to_fvk().to_ovk(ZcashOrchardScope::EXTERNAL).to_bytes() )).unwrap();

    writeln!(file, "{}", format_bytes("secp_secret_key", &secp_secret_key.to_bytes())).unwrap();
    writeln!(file, "orchard_diversifier_index_u32:{orchard_div_idx_u32}").unwrap();
    writeln!(file, "orchard_diversifier_index_u64:{orchard_div_idx_u64}").unwrap();

    writeln!(file, "{}", format_bytes("orchard_diversifier_index_from_u32", &orchard_div_idx_u32_obj.to_bytes().as_slice()) ).unwrap();
    writeln!(file, "{}", format_bytes("orchard_diversifier_index_from_u64", &orchard_div_idx_u64_obj.to_bytes().as_slice()) ).unwrap();
}

fn format_bytes(label: &str, bytes: &[u8]) -> String {
    let bytes_arr = bytes
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(",");

    format!("{label}:[{bytes_arr}]")
}

fn get_ext_sk_from_path(ext_sk: &ExtendedSpendingKey) -> ExtendedSpendingKey {
    let path = [
        ChildIndex::Hardened(32),
        ChildIndex::Hardened(133),
        ChildIndex::Hardened(2),
        ChildIndex::NonHardened(3),
    ];

    ExtendedSpendingKey::from_path(ext_sk, &path)
}

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

fn get_orchard_address(key: &UnifiedSpendingKey) -> [u8; 43] {
    let diversifier = orchard::keys::Diversifier::from_bytes([0; 11]);

    key.to_unified_full_viewing_key()
        .orchard()
        .unwrap()
        .to_ivk(orchard::keys::Scope::External)
        .address(diversifier)
        .to_raw_address_bytes()
}
