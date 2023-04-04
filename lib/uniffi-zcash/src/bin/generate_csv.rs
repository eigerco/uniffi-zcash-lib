use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
// issue with deprecation
use zcash_primitives::zip32::ExtendedSpendingKey;
use uniffi_zcash::{
    ZcashAccountId,
    ZcashAccountPrivKey,
    ZcashConsensusParameters::MainNetwork,
    ZcashChildIndex,
    ZcashDiversifier,
    ZcashDiversifierIndex,
    ZcashDiversifierIndexAndPaymentAddress,
    ZcashKeysEra,
    ZcashFullViewingKey,
    ZcashExtendedFullViewingKey,
    ZcashExtendedPrivKey,
    ZcashExtendedSpendingKey,
    ZcashExpandedSpendingKey,
    ZcashOrchardDiversifier,
    ZcashOrchardDiversifierIndex,
    ZcashOrchardScope,
    ZcashOrchardSpendingKey,
    ZcashOutgoingViewingKey,
    ZcashScope,
    ZcashUnifiedSpendingKey,
    ZcashPaymentAddress
};

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

    let usk = ZcashUnifiedSpendingKey::from_seed(MainNetwork, seed.to_vec(), ZcashAccountId { id: 0 }).unwrap();

    // Obtaining from original API expected byte array results
    // for derivation from ExtendedPrivKey to AccountPrivKey.
    let extended_priv_key = ZcashExtendedPrivKey::with_seed(seed.to_vec()).unwrap();

    let apk = ZcashAccountPrivKey::from_seed(MainNetwork, seed.to_vec(), ZcashAccountId { id: 0 }).unwrap();
    let ppk = apk.to_account_pubkey();
    let secp_secret_key = apk.derive_external_secret_key(0);
    let extended_private_key = ZcashAccountPrivKey::from_extended_privkey(Arc::new(extended_priv_key));

    let encoded = usk.to_unified_full_viewing_key().encode(MainNetwork);

    let extended_spending_key = ZcashExtendedSpendingKey::master(seed.to_vec());
    let extended_spending_key_orig = ExtendedSpendingKey::master(&seed);
    let tup: ZcashDiversifierIndexAndPaymentAddress = extended_spending_key.default_address();
    let ext_sk_child_index = tup.diversifier_index;
    let ext_sk_default_address = tup.address;

    let coin_type = 234;
    let account_number = 2345;
    let orchard_sk = ZcashOrchardSpendingKey::from_zip32_seed(seed.to_vec(), coin_type, account_number).unwrap();
    let diversifier = ZcashDiversifier::new(vec![0; 11]).unwrap();
    let orchard_diversifier = ZcashOrchardDiversifier::from_bytes(vec![0; 11]).unwrap();


    let diversifiable_fvk = extended_spending_key.to_diversifiable_full_viewing_key();
    #[allow(deprecated)]
    let sapling_fvk: ZcashExtendedFullViewingKey = extended_spending_key_orig.to_extended_full_viewing_key().into();

    // let expanded_sk = ZcashExpandedSpendingKey::from_spending_key(&extended_spending_key.to_bytes());
    let expanded_sk = ZcashExpandedSpendingKey::from_spending_key(extended_spending_key.to_bytes());
    let expanded_sk_vk = expanded_sk.proof_generation_key().to_viewing_key();

    let orchard_div_idx_u32 = 4;
    let orchard_div_idx_u64: u64 = (u32::MAX as u64) + 1u64;
    let orchard_div_idx_u32_obj = ZcashOrchardDiversifierIndex::from_u32(orchard_div_idx_u32);
    let orchard_div_idx_u64_obj = ZcashOrchardDiversifierIndex::from_u64(orchard_div_idx_u64);

    let orchard_address = get_orchard_address(&usk);

    let sapling_fvk_from_expsk = ZcashFullViewingKey::from_expanded_spending_key(Arc::new(expanded_sk.clone()));
    let sapling_fvk_encoded = sapling_fvk.encode(MainNetwork);

    writeln!(file, "{}", format_bytes("seed", &seed)).unwrap();
    writeln!(file, "coin_type:{coin_type}").unwrap();
    writeln!(file, "account:{account_number}").unwrap();
    writeln!(file, "scope:External").unwrap();
    writeln!(file, "t_address_public_key:tm9iMLAuYMzJ6jtFLcA7rzUmfreGuKvr7Ma").unwrap();
    writeln!(file, "t_address_script:t26YoyZ1iPgiMEWL4zGUm74eVWfhyDMXzY2").unwrap();
    writeln!(file, "unified_full_viewing_key_encoded:{encoded}").unwrap();
    writeln!(file, "extended_fvk_encoded:{sapling_fvk_encoded}").unwrap();
    writeln!(file, "{}", format_bytes("sapling_address", &get_sapling_address(&usk).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_address", &orchard_address.as_slice())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_diversifier", &orchard_diversifier.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("unified_spending_key", &usk.to_bytes(ZcashKeysEra::Orchard))).unwrap();
    writeln!(file, "{}", format_bytes("account_private_key", &apk.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("account_public_key", &ppk.serialize())).unwrap();
    // writeln!(file, "{}", format_bytes("ppk_external_ivk", &ppk.derive_external_ivk().unwrap().as_bytes())).unwrap();
    // writeln!(file, "{}", format_bytes("ppk_internal_ivk", &ppk.derive_internal_ivk().unwrap().as_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("ppk_external_ovk", &ppk.external_ovk().as_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("ppk_internal_ovk", &ppk.internal_ovk().as_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_private_key", &extended_private_key.to_bytes())).unwrap();

    writeln!(file, "{}", format_bytes("extended_spending_key", &extended_spending_key.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_from_path", &get_ext_sk_from_path(&extended_spending_key).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_derived_child", &extended_spending_key.derive_child(ZcashChildIndex::Hardened { v: 32 }).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_default_address", &ext_sk_default_address.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_child_index", &ext_sk_child_index.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_internal_sk", &extended_spending_key.derive_internal().to_bytes())).unwrap();

    writeln!(file, "{}", format_bytes("expanded_spending_key", &expanded_sk.to_bytes())).unwrap();
    // writeln!(file, "{}", format_bytes("expanded_spending_key_viewing_key", &expanded_sk_vk.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("diversifiable_fvk", &diversifiable_fvk.to_bytes())).unwrap();

    writeln!(file, "{}", format_bytes("sapling_full_viewing_key", &sapling_fvk_from_expsk.to_bytes())).unwrap();
    // writeln!(file, "{}", format_bytes("sapling_full_viewing_key_vk", &sapling_fvk_from_expsk.vk().to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("sapling_full_viewing_key_vk_payment_address", &sapling_fvk_from_expsk.vk().to_payment_address(Arc::new(diversifier)).expect("").to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("sapling_full_viewing_key_vk_ivk", &(*sapling_fvk_from_expsk.vk().ivk().as_ref().clone()).to_repr())).unwrap();
    writeln!(file, "{}", format_bytes("sapling_full_viewing_key_ovk", &sapling_fvk_from_expsk.ovk().to_bytes())).unwrap();

    writeln!(file, "{}", format_bytes("extended_fvk", &sapling_fvk.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_child", &sapling_fvk.derive_child(ZcashChildIndex::NonHardened { v: 32 }).expect("REASON").to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_address", &sapling_fvk.address(Arc::new(ZcashDiversifierIndex::from_u32(4))).expect("REASON").to_bytes())).unwrap();
    // this is a weird thing
    //writeln!(file, "{}", format_bytes("extended_fvk_find_address", &sapling_fvk.find_address(Arc::new(ZcashDiversifierIndex::from_u32(0))).unwrap().to_bytes())).unwrap();
    //writeln!(file, "{}", format_bytes("extended_fvk_default_address", &sapling_fvk.default_address().to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_derive_internal", &sapling_fvk.derive_internal().to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_diversifiable_fvk", &sapling_fvk.to_diversifiable_full_viewing_key().to_bytes())).unwrap();

    // TODO no to_bytes
    // writeln!(file, "{}", format_bytes("extended_spending_key_fvk_nk", (*diversifiable_fvk.to_nk(ZcashScope::External)).to_bytes())).unwrap();
    // writeln!(file, "{}", format_bytes("extended_spending_key_fvk_ivk", &diversifiable_fvk.to_ivk(ZcashScope::External).0.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_fvk_ovk", &diversifiable_fvk.to_ovk(ZcashScope::External).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_fvk_addr", &diversifiable_fvk.address(Arc::new(ZcashDiversifierIndex::from_u32(4))).unwrap().to_bytes())).unwrap();

    writeln!(file, "{}", format_bytes("sapling_outgoing_viewing_key", &get_ovk(&usk).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_spending_key", &usk.orchard().to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_spending_key_from_zip32_seed", &orchard_sk.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_full_viewing_key", &orchard_sk.to_fvk().to_bytes())).unwrap();

    writeln!(file, "{}", format_bytes("orchard_div_idx_address", &(*orchard_sk.to_fvk().address(Arc::new(orchard_diversifier), ZcashOrchardScope::External).as_ref().clone()).to_raw_address_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_div_idx_address_at", &(*orchard_sk.to_fvk().address_at(Arc::new(ZcashOrchardDiversifierIndex::from_u32(4)), ZcashOrchardScope::External).as_ref().clone()).to_raw_address_bytes() )).unwrap();
    writeln!(file, "{}", format_bytes("orchard_full_viewing_key_ivk", &orchard_sk.to_fvk().to_ivk(ZcashOrchardScope::External).to_bytes())).unwrap();
    // ZcashOrchardOutgoingViewingKey` no to_bytes
    // writeln!(file, "{}", format_bytes("orchard_full_viewing_key_ovk", &(*orchard_sk.to_fvk().to_ovk(ZcashOrchardScope::External).as_ref().clone()).to_bytes() )).unwrap();

    // writeln!(file, "{}", format_bytes("secp_secret_key", &(*secp_secret_key.unwrap().as_ref().clone()).to_bytes())).unwrap();
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

fn get_ext_sk_from_path(ext_sk: &ZcashExtendedSpendingKey) -> ZcashExtendedSpendingKey {
    let path = vec![
        ZcashChildIndex::Hardened { v: 32 },
        ZcashChildIndex::Hardened { v: 133 },
        ZcashChildIndex::Hardened { v: 2 },
        ZcashChildIndex::NonHardened { v: 3 }
    ];

    ZcashExtendedSpendingKey::from_path(ext_sk, path)
}

fn get_ovk(key: &ZcashUnifiedSpendingKey) -> ZcashOutgoingViewingKey {
    key.to_unified_full_viewing_key()
        .sapling()
        .unwrap()
        .to_ovk(ZcashScope::External)
        .as_ref().clone()
}

fn get_sapling_address(key: &ZcashUnifiedSpendingKey) -> ZcashPaymentAddress {
    let diversifier = ZcashDiversifier::new(vec![0; 11]).unwrap();

    key.to_unified_full_viewing_key()
        .sapling()
        .unwrap()
        .to_ivk(ZcashScope::External)
        .to_payment_address(diversifier.into())
        .unwrap()
        .as_ref().clone()

    // Arc::try_unwrap(arc).unwrap()
}

fn get_orchard_address(key: &ZcashUnifiedSpendingKey) -> Vec<u8> {
    let diversifier = ZcashOrchardDiversifier::from_bytes(vec![0; 11]).unwrap();

    key.to_unified_full_viewing_key()
        .orchard()
        .unwrap()
        .to_ivk(ZcashOrchardScope::External)
        .address(Arc::new(diversifier))
        .to_raw_address_bytes()
}
// TODO should we use these below to track tests?

    // https://github.com/eigerco/uniffi-zcash-lib/blob/main/STATUS.md#zcashextendedprivkey

    // https://github.com/eigerco/uniffi-zcash-lib/blob/main/STATUS.md#zcashaccountpubkey

    // https://github.com/eigerco/uniffi-zcash-lib/blob/main/STATUS.md#zcashunifiedspendingkey

    // https://github.com/eigerco/uniffi-zcash-lib/blob/main/STATUS.md#zcashunifiedfullviewingkey

    // https://github.com/eigerco/uniffi-zcash-lib/blob/main/STATUS.md#zcashunifiedaddress

    // https://github.com/eigerco/uniffi-zcash-lib/blob/main/STATUS.md#zcashtransparentaddress

    // https://github.com/eigerco/uniffi-zcash-lib/blob/main/STATUS.md#zcashdiversifiablefullviewingkey-sapling

    // https://github.com/eigerco/uniffi-zcash-lib/blob/main/STATUS.md#zcashextendedspendingkey-sapling
