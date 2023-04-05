use std::io::Write;

use zcash_client_backend::keys::{Era, UnifiedSpendingKey};
use zcash_primitives::{
    consensus::MainNetwork,
    zip32::{ChildIndex, ExtendedSpendingKey, DiversifierIndex},
};

use super::format_bytes;

#[rustfmt::skip]
pub fn write_for_zcash_client_backend<W: Write>(mut file: W, seed: &[u8]) {
    let usk = UnifiedSpendingKey::from_seed(&MainNetwork, seed, 0.into()).unwrap();
    let encoded = usk.to_unified_full_viewing_key().encode(&MainNetwork);
    writeln!(file, "unified_full_viewing_key_encoded:{encoded}").unwrap();
    writeln!(file, "{}", format_bytes("unified_spending_key", &usk.to_bytes(Era::Orchard))).unwrap();

    let extended_spending_key = ExtendedSpendingKey::master(seed);
    let (ext_sk_child_index, ext_sk_default_address) = extended_spending_key.default_address();
    writeln!(file, "{}", format_bytes("extended_spending_key", &extended_spending_key.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_from_path", &get_ext_sk_from_path(&extended_spending_key).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_derived_child", &extended_spending_key.derive_child(ChildIndex::Hardened(32)).to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_default_address", &ext_sk_default_address.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_child_index", &ext_sk_child_index.0)).unwrap();
    writeln!(file, "{}", format_bytes("extended_spending_key_internal_sk", &extended_spending_key.derive_internal().to_bytes())).unwrap();

    let diversifiable_fvk = extended_spending_key.to_diversifiable_full_viewing_key();
    writeln!(file, "{}", format_bytes("diversifiable_fvk", &diversifiable_fvk.to_bytes())).unwrap();

    let diversifier = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    writeln!(file, "{}", format_bytes("diversifier", &diversifier)).unwrap();

    let transparent_address_public_key = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    writeln!(file, "{}", format_bytes("transparent_address_public_key", &transparent_address_public_key)).unwrap();

    let seed = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f
    ];
    let ufvk = UnifiedSpendingKey::from_seed(
        &MainNetwork,
        &seed,
        0u32.into(),
    ).unwrap().to_unified_full_viewing_key();
    let encoded = ufvk.encode(&MainNetwork);
    writeln!(file, "unified_full_viewing_key_encoded_2:{encoded}").unwrap();

    let index = DiversifierIndex::from(0u32);

    let ufvk_address = ufvk.address(index).unwrap().encode(&MainNetwork);
    writeln!(file, "unified_full_viewing_key_address_encoded:{ufvk_address}").unwrap();

    let (ufvk_find_address_address, ufvk_find_address_index) = ufvk.find_address(index).unwrap();
    let ufvk_find_address_address_encoded = ufvk_find_address_address.encode(&MainNetwork);
    let ufvk_find_address_index_bytes = ufvk_find_address_index.0.to_vec();
    writeln!(file, "unified_full_viewing_key_find_address_address_encoded:{ufvk_find_address_address_encoded}").unwrap();
    writeln!(file, "{}", format_bytes("unified_full_viewing_key_find_address_index", &ufvk_find_address_index_bytes)).unwrap();

    let (ufvk_default_address_address, ufvk_default_address_index) = ufvk.default_address();
    let ufvk_default_address_address_encoded = ufvk_default_address_address.encode(&MainNetwork);
    let ufvk_default_address_index_bytes = ufvk_default_address_index.0.to_vec();
    writeln!(file, "unified_full_viewing_key_default_address_address_encoded:{ufvk_default_address_address_encoded}").unwrap();
    writeln!(file, "{}", format_bytes("unified_full_viewing_key_default_address_index", &ufvk_default_address_index_bytes)).unwrap();
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
