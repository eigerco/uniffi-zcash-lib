use uniffi_backend_keys::{unified_sk_from_seed, Network};

fn main() {
    let mut seed = vec![0u8; 32];
    seed[0] = 1u8;

    let usk = unified_sk_from_seed(Network::MAIN, seed, 0).unwrap();
    let usk_json = serde_json::to_string(&usk).unwrap();

    std::fs::File::create("test_vectors/zusk.json").expect("Unable to create file");
    std::fs::write("test_vectors/zusk.json", usk_json).expect("Unable to write file");
}
