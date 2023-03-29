use orchard::keys::SpendingKey;
use zcash_client_backend::keys::UnifiedSpendingKey;

fn spending_key(unified_key: &UnifiedSpendingKey) -> String {
    let data = unified_key
        .orchard()
        .to_bytes()
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("Orchard spending key bytes: {data}");

    format!("OrchardSpendingKey.fromBytes;{data};")
}

fn spending_key_from_zip32_seed(seed: &[u8]) -> String {
    let in1 = 234;
    let in2 = 2345;

    let data = SpendingKey::from_zip32_seed(seed, in1, in2)
        .unwrap()
        .to_bytes()
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("Orchard spending key (from zip32 seed) bytes: {data}");

    let seed_str = format_u8(seed);

    format!("SpendingKey::from_zip32_seed;[{seed_str}]${in1}${in2};[{data}]")
}

fn format_u8(arr: &[u8]) -> String {
    arr.iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(",")
}
