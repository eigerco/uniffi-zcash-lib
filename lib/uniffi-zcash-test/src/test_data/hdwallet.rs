use std::io::Write;

use hdwallet::{traits::Serialize, ExtendedPrivKey};

use super::format_bytes;

#[rustfmt::skip]
pub fn write_for_hdwallet<W: Write>(mut file: W, seed: &[u8]) {
    let key = ExtendedPrivKey::with_seed(seed).unwrap();
    writeln!(file, "{}", format_bytes("hdwallet_epk", &key.serialize())).unwrap();

    let index = 3.into();
    let derived = key.derive_private_key(index).unwrap();
    writeln!(file, "{}", format_bytes("hdwallet_epk_derive_private_key", &derived.serialize())).unwrap();
}
