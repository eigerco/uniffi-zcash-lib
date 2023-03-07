use rustler::{Binary, Error, NifRecord, NifResult, ResourceArc};

#[derive(NifRecord)]
#[tag = "zcash_priv_key"]
struct PrivKey(ResourceArc<crate::ZcashAccountPrivKey>);

#[derive(NifRecord)]
#[tag = "zcash_pub_key"]
struct PubKey(ResourceArc<crate::ZcashAccountPubKey>);

#[rustler::nif]
fn from_bytes(data: Binary) -> NifResult<Option<PrivKey>> {
    if data.len() < 32 {
        return Err(Error::BadArg);
    }

    Ok(crate::from_bytes(data.to_vec())
        .map(ResourceArc::new)
        .map(PrivKey))
}

#[rustler::nif]
fn from_seed(seed: Binary, account_id: u32) -> NifResult<Option<PrivKey>> {
    if seed.len() < 32 {
        return Err(Error::BadArg);
    }

    Ok(crate::from_seed(seed.to_vec(), account_id)
        .map(ResourceArc::new)
        .map(PrivKey))
}

#[rustler::nif]
fn test_from_seed(seed: Binary, account_id: u32) -> NifResult<Option<PrivKey>> {
    if seed.len() < 32 {
        return Err(Error::BadArg);
    }

    Ok(crate::test_from_seed(seed.to_vec(), account_id)
        .map(ResourceArc::new)
        .map(PrivKey))
}

#[rustler::nif]
fn to_account_pubkey(priv_key: PrivKey) -> PubKey {
    PubKey(ResourceArc::new(crate::to_account_pubkey(&priv_key.0)))
}

fn load(env: rustler::Env, _: rustler::Term) -> bool {
    rustler::resource!(crate::ZcashAccountPrivKey, env);
    rustler::resource!(crate::ZcashAccountPubKey, env);

    true
}

rustler::init!(
    "Elixir.Zcash",
    [from_bytes, from_seed, to_account_pubkey],
    load = load
);
