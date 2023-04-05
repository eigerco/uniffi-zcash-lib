use std::io::Write;

use orchard::keys::{DiversifierIndex, FullViewingKey, Scope, SpendingKey};
use zcash_client_backend::keys::UnifiedSpendingKey;
use zcash_primitives::consensus::MainNetwork;

use super::format_bytes;

#[rustfmt::skip]
pub fn write_for_orchard<W: Write>(mut file: W, seed: &[u8]) {
    let orchard_diversifier = orchard::keys::Diversifier::from_bytes([0; 11]);
    writeln!(file, "{}", format_bytes("orchard_diversifier", orchard_diversifier.as_array())).unwrap();

    let usk = UnifiedSpendingKey::from_seed(&MainNetwork, seed, 0.into()).unwrap();
    writeln!(file, "{}", format_bytes("orchard_spending_key", usk.orchard().to_bytes().as_slice())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_spending_key_fvk", FullViewingKey::from(usk.orchard()).to_bytes().as_slice())).unwrap();

    let coin_type = 234;
    writeln!(file, "coin_type:{coin_type}").unwrap();

    let account_number = 2345;
    writeln!(file, "account:{account_number}").unwrap();

    let orchard_sk = SpendingKey::from_zip32_seed(seed, coin_type, account_number).unwrap();
    writeln!(file, "{}", format_bytes("orchard_spending_key_from_zip32_seed", orchard_sk.to_bytes().as_slice())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_full_viewing_key", FullViewingKey::from(&orchard_sk).to_bytes().as_slice())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_div_idx_address", &FullViewingKey::from(&orchard_sk).address(orchard_diversifier, Scope::External).to_raw_address_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_div_idx_address_at", &FullViewingKey::from(&orchard_sk).address_at(DiversifierIndex::from(4u32), Scope::External).to_raw_address_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_full_viewing_key_ivk", &FullViewingKey::from(&orchard_sk).to_ivk(Scope::External).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("orchard_full_viewing_key_ovk", FullViewingKey::from(&orchard_sk).to_ovk(Scope::External).as_ref())).unwrap();

    let orchard_address = get_orchard_address(&usk);
    writeln!(file, "{}", format_bytes("orchard_address", orchard_address.as_slice())).unwrap();

    let orchard_div_idx_u32 = 4u32;
    writeln!(file, "orchard_diversifier_index_u32:{orchard_div_idx_u32}").unwrap();

    let orchard_div_idx_u32_obj: DiversifierIndex = orchard_div_idx_u32.into();
    writeln!(file, "{}", format_bytes("orchard_diversifier_index_from_u32", orchard_div_idx_u32_obj.to_bytes().as_slice()) ).unwrap();

    let orchard_div_idx_u64 = u32::MAX as u64 + 1;
    writeln!(file, "orchard_diversifier_index_u64:{orchard_div_idx_u64}").unwrap();

    let orchard_div_idx_u64_obj: DiversifierIndex = orchard_div_idx_u64.into();
    writeln!(file, "{}", format_bytes("orchard_diversifier_index_from_u64", orchard_div_idx_u64_obj.to_bytes().as_slice()) ).unwrap();

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
