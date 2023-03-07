use hdwallet::ExtendedPrivKey;
use zcash_client_backend::keys::{Era, UnifiedSpendingKey};
use zcash_primitives::{
    consensus::MainNetwork, legacy::keys::AccountPrivKey, sapling::Diversifier,
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
}
