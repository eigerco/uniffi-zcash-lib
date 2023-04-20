mod hdwallet;
mod orchard;
mod secp256k1;
mod transaction;
mod zcash_client_backend;
mod zcash_primitives;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use self::hdwallet::write_for_hdwallet;
use self::orchard::write_for_orchard;
use self::secp256k1::write_for_secp256k1;
use self::transaction::write_for_transaction;
use self::zcash_client_backend::write_for_zcash_client_backend;
use self::zcash_primitives::write_for_zcash_primitives;

pub fn generate_test_data(force: bool) {
    let base_url = env!("CARGO_MANIFEST_DIR");
    let path = format!("{base_url}/../uniffi-zcash/tests/test_data.csv");

    if force || !Path::new(&path).exists() {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();

        writeln!(file, "test_string:TestString").unwrap();

        let mut seed = vec![0u8; 32];
        seed[0] = 1u8;
        writeln!(file, "{}", format_bytes("seed", &seed)).unwrap();

        write_for_hdwallet(&mut file, &seed);
        write_for_orchard(&mut file, &seed);
        write_for_secp256k1(&mut file, &seed);
        write_for_zcash_client_backend(&mut file, &seed);
        write_for_zcash_primitives(&mut file, &seed);
        write_for_transaction(&mut file, &seed);
    }
}

pub(crate) fn format_bytes(label: &str, bytes: &[u8]) -> String {
    let bytes_arr = bytes
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(",");

    format!("{label}:[{bytes_arr}]")
}
