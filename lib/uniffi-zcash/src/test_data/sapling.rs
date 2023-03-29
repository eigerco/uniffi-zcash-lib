use zcash_primitives::zip32::{ChildIndex, ExtendedSpendingKey};

fn extended_spending_key(seed: &[u8]) -> String {
    let data = format_u8(&ExtendedSpendingKey::master(seed).to_bytes());

    println!("Sapling extended spending key bytes: {data}");

    let seed_str = format_u8(&seed);

    format!("ExtendedSpendingKey::master;[{seed_str}];[{data}]")
}

fn extended_spending_key_from_path(seed: &[u8]) -> String {
    let sk = ExtendedSpendingKey::master(seed);
    let path = [
        ChildIndex::Hardened(32),
        ChildIndex::Hardened(133),
        ChildIndex::Hardened(2),
        ChildIndex::NonHardened(3),
    ];
    let derived_key = ExtendedSpendingKey::from_path(&sk, &path);
    let data = format_u8(&derived_key.to_bytes());

    println!("Sapling derived from path extended spending key bytes: {data}");

    format!("ExtendedSpendingKey::from_path;32,133,2,3;[{data}]")
}

fn extended_spending_key_derive_child(seed: &[u8]) -> String {
    let sk = ExtendedSpendingKey::master(seed);
    let derived_key = sk.derive_child(ChildIndex::Hardened(32));
    let data = format_u8(&derived_key.to_bytes());

    println!("Sapling derive child from extended spending key bytes: {data}");

    format!("ExtendedSpendingKey::derive_child;32;[{data}]")
}

fn extended_spending_key_default_address(seed: &[u8]) -> String {
    let sk = ExtendedSpendingKey::master(seed);
    let (_, address) = sk.default_address();

    let data = format_u8(&address.to_bytes());

    println!("Sapling extended spending key default address: {data}");

    format!("ExtendedSpendingKey::default_address;;[{data}]")
}

fn extended_spending_key_default_child_index(seed: &[u8]) -> String {
    let sk = ExtendedSpendingKey::master(seed);
    let (index, _) = sk.default_address();

    let data = format_u8(&index.0);

    println!("Sapling extended spending key default child index: {data}");

    format!("ExtendedSpendingKey::child_index;;[{data}]")
}

fn extended_spending_key_derive_internal(seed: &[u8]) -> String {
    let sk = ExtendedSpendingKey::master(seed);

    let data = format_u8(&sk.derive_internal().to_bytes());

    println!("Sapling spending key derive internal: {data}");

    format!("ExtendedSpendingKey::derive_internal;;[{data}]")
}

fn extended_spending_key_fvk(seed: &[u8]) -> String {
    let sk = ExtendedSpendingKey::master(seed);

    let data = format_u8(&sk.to_diversifiable_full_viewing_key().to_bytes());

    println!("Sapling spending key fvk bytes: {data}");

    format!("ExtendedSpendingKey::to_diversifiable_full_viewing_key;;[{data}]")
}

fn format_u8(arr: &[u8]) -> String {
    arr.iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(",")
}
