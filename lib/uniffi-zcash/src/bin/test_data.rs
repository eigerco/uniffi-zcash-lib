use hdwallet::ExtendedPrivKey;
use orchard::keys::SpendingKey;
use zcash_client_backend::keys::{Era, UnifiedSpendingKey};
use zcash_primitives::{
    consensus::MainNetwork,
    legacy::keys::AccountPrivKey,
    sapling::Diversifier,
    zip32::{ChildIndex, ExtendedSpendingKey},
};

fn main() {
    let mut seed = vec![0u8; 32];
    seed[0] = 1u8;

    let key = UnifiedSpendingKey::from_seed(&MainNetwork, &seed, 0.into()).unwrap();
    let usk_bytes = key
        .to_bytes(Era::Orchard)
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    let encoded = key.to_unified_full_viewing_key().encode(&MainNetwork);

    let sapling_diversifier = Diversifier([0; 11]);
    let sapling_address = key
        .to_unified_full_viewing_key()
        .sapling()
        .unwrap()
        .to_ivk(zcash_primitives::zip32::Scope::External)
        .to_payment_address(sapling_diversifier)
        .unwrap()
        .to_bytes()
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    let sapling_outgoing_viewing_key = key
        .to_unified_full_viewing_key()
        .sapling()
        .unwrap()
        .to_ovk(zcash_primitives::zip32::Scope::External)
        .0
        .map(|byte| byte.to_string())
        .to_vec()
        .join(", ");

    let orchard_diversifier = orchard::keys::Diversifier::from_bytes([0; 11]);
    let orchard_address = key
        .to_unified_full_viewing_key()
        .orchard()
        .unwrap()
        .to_ivk(orchard::keys::Scope::External)
        .address(orchard_diversifier)
        .to_raw_address_bytes()
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    let account_priv_key = AccountPrivKey::from_seed(&MainNetwork, &seed, 0.into()).unwrap();
    let apk_bytes = account_priv_key
        .to_bytes()
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("UnifiedSpendingKey bytes: {usk_bytes}");
    println!();
    println!("UnifiedFullViewingKey encoded: {encoded}");
    println!();
    println!("SaplinkIvk PaymentAddress bytes: {sapling_address}");
    println!();
    println!("SaplingOvk bytes: {sapling_outgoing_viewing_key}");
    println!();
    println!("OrchardIvk PaymentAddress bytes: {orchard_address}");
    println!();
    println!("AccountPrivateKey bytes: {apk_bytes}");

    // Obtaining from original API expected byte array results
    // for derivation from ExtendedPrivKey to AccountPrivKey.
    let extended_priv_key = ExtendedPrivKey::with_seed(&seed).unwrap();
    let extended_private_key = AccountPrivKey::from_extended_privkey(extended_priv_key);
    let extended_private_key_bytes = extended_private_key
        .to_bytes()
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    println!("AccountPrivateKey from ExtendedPrivKey bytes: {extended_private_key_bytes}");
    println!();

    sapling_extended_spending_key(&seed);
    sapling_extended_spending_key_from_path(&seed);
    sapling_extended_spending_key_derive_child(&seed);
    sapling_extended_spending_key_default_address(&seed);
    sapling_extended_spending_key_derive_internal(&seed);
    sapling_extended_spending_key_fvk(&seed);

    orchard_spending_key(&key);
    orchard_spending_key_from_zip32_seed(&seed);
}

fn sapling_extended_spending_key(seed: &[u8]) {
    let data = ExtendedSpendingKey::master(seed)
        .to_bytes()
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("Sapling extended spending key bytes: {data}");
}

fn sapling_extended_spending_key_from_path(seed: &[u8]) {
    let sk = ExtendedSpendingKey::master(seed);
    let path = [
        ChildIndex::Hardened(32),
        ChildIndex::Hardened(133),
        ChildIndex::Hardened(2),
        ChildIndex::NonHardened(3),
    ];
    let derived_key = ExtendedSpendingKey::from_path(&sk, &path);
    let data = derived_key
        .to_bytes()
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("Sapling derived from path extended spending key bytes: {data}");
}

fn sapling_extended_spending_key_derive_child(seed: &[u8]) {
    let sk = ExtendedSpendingKey::master(seed);
    let derived_key = sk.derive_child(ChildIndex::Hardened(32));
    let data = derived_key
        .to_bytes()
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("Sapling derive child from extended spending key bytes: {data}");
}

fn sapling_extended_spending_key_default_address(seed: &[u8]) {
    let sk = ExtendedSpendingKey::master(seed);
    let (index, address) = sk.default_address();

    let address_data = address
        .to_bytes()
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    let index_data = index
        .0
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("Sapling extended spending key default address: {address_data}");
    println!("Sapling extended spending key default child index: {index_data}");
}

fn sapling_extended_spending_key_derive_internal(seed: &[u8]) {
    let sk = ExtendedSpendingKey::master(seed);

    let data = sk
        .derive_internal()
        .to_bytes()
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("Sapling spending key derive internal: {data}");
}

fn sapling_extended_spending_key_fvk(seed: &[u8]) {
    let sk = ExtendedSpendingKey::master(seed);

    let data = sk
        .to_diversifiable_full_viewing_key()
        .to_bytes()
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("Sapling spending key fvk bytes: {data}");
}

fn orchard_spending_key(unified_key: &UnifiedSpendingKey) {
    let data = unified_key
        .orchard()
        .to_bytes()
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("Orchard spending key bytes: {data}");
}

fn orchard_spending_key_from_zip32_seed(seed: &[u8]) {
    let data = SpendingKey::from_zip32_seed(seed, 234, 2345)
        .unwrap()
        .to_bytes()
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("Orchard spending key (from zip32 seed) bytes: {data}");
}
