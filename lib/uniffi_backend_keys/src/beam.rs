use rustler::{Binary, ResourceArc, NifResult, Error};



#[rustler::nif]
fn from_bytes(data: Binary) -> NifResult<Option<ResourceArc<crate::ZcashAccountPrivKey>>> {
    if data.len() < 32 { return Err(Error::BadArg) }

    Ok(crate::from_bytes(data.to_vec()).map(ResourceArc::new))
}

#[rustler::nif]
fn from_seed(seed: Binary, account_id: u32) -> NifResult<Option<ResourceArc<crate::ZcashAccountPrivKey>>> {
    if seed.len() < 32 { return Err(Error::BadArg) }

    Ok(crate::from_seed(seed.to_vec(), account_id).map(ResourceArc::new))
}

#[rustler::nif]
fn to_account_pubkey(priv_key: ResourceArc<crate::ZcashAccountPrivKey>) -> ResourceArc<crate::ZcashAccountPubKey> {
    ResourceArc::new(crate::to_account_pubkey(&priv_key))
}

fn load(env: rustler::Env, _: rustler::Term) -> bool {
    rustler::resource!(crate::ZcashAccountPrivKey, env);
    rustler::resource!(crate::ZcashAccountPubKey, env);

    true
}

rustler::init!("Elixir.ZcashBackendKeys", [
    from_bytes,
    from_seed,
    to_account_pubkey
], load = load);
