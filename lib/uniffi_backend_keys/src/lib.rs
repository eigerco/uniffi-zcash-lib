use zcash_client_backend::keys::{Era, UnifiedFullViewingKey, UnifiedSpendingKey};
use zcash_primitives::consensus::{self, MAIN_NETWORK, TEST_NETWORK};
use zcash_primitives::legacy::keys::AccountPrivKey;
use zcash_primitives::zip32::{AccountId, Scope as ZScope};
// use zcash_primitives::{};
use zcash_primitives::sapling::{Diversifier, SaplingIvk};
// use zcash_primitives::sapling::keys::OutgoingViewingKey;
use ff::PrimeField;
use serde::Serialize;

/// This is a doc comment. ZcashAccountPrivKey
#[derive(Debug, Serialize)]
pub struct ZcashAccountPrivKey {
    data: Vec<u8>,
}

/// This is a doc comment. ZcashExtendedSpendingKey
#[derive(Debug, Serialize)]
pub struct ZcashExtendedSpendingKey {
    data: Vec<u8>,
}

/// This is a doc comment. ZcashSpendingKey
#[derive(Debug, Serialize)]
pub struct ZcashSpendingKey {
    data: Vec<u8>,
}

/// This is a doc comment. ZcashUnifiedSpendingKey
#[derive(Debug, Serialize)]
pub struct ZcashUnifiedSpendingKey {
    transparent: ZcashAccountPrivKey,
    sapling: ZcashExtendedSpendingKey,
    orchard: ZcashSpendingKey,
    binary: Vec<u8>,
    network: Network,
}

pub struct ZcashAccountPubKey {
    data: Vec<u8>,
}

pub struct ZcashDiversifiableFullViewingKey {
    data: Vec<u8>,
}

pub struct ZcashFullViewingKey {
    data: Vec<u8>,
}

/*
unknown_fst: Vec<u32>;
unknown_snd: Vec<Vec<u8>> ;
*/
pub struct ZcashUnifiedFullViewingKey {
    transparent: ZcashAccountPubKey,
    sapling: ZcashDiversifiableFullViewingKey,
    orchard: ZcashFullViewingKey,
    encoded: String,
}

pub struct ZcashOutgoingViewingKey {
    data: Vec<u8>,
}

pub struct ZcashSaplingIvk {
    data: Vec<u8>,
}

#[derive(Debug, Serialize)]
pub enum Network {
    MAIN,
    TEST,
}

pub enum Scope {
    INTERNAL,
    EXTERNAL,
}

pub fn unified_sk_from_seed(
    network: Network,
    seed: Vec<u8>,
    account_id: u32,
) -> Option<ZcashUnifiedSpendingKey> {
    let uusk = match network {
        Network::MAIN => {
            UnifiedSpendingKey::from_seed(&MAIN_NETWORK, &seed, AccountId::from(account_id))
        }
        Network::TEST => {
            UnifiedSpendingKey::from_seed(&TEST_NETWORK, &seed, AccountId::from(account_id))
        }
    }
    .ok();

    uusk.map(|usk| ZcashUnifiedSpendingKey {
        transparent: ZcashAccountPrivKey {
            data: usk.transparent().to_bytes(),
        },
        sapling: ZcashExtendedSpendingKey {
            data: usk.sapling().to_bytes().to_vec(),
        },
        orchard: ZcashSpendingKey {
            data: usk.orchard().to_bytes().to_vec(),
        },
        binary: usk.to_bytes(Era::Orchard).to_vec(),
        network,
    })
}

fn ufvk_to_external<P: consensus::Parameters>(
    ufvk: UnifiedFullViewingKey,
    network: &P,
) -> ZcashUnifiedFullViewingKey {
    ZcashUnifiedFullViewingKey {
        transparent: ZcashAccountPubKey {
            data: ufvk
                .transparent()
                .unwrap_or_else(|| panic!("Error with taddr"))
                .serialize(),
        },
        sapling: ZcashDiversifiableFullViewingKey {
            data: ufvk
                .sapling()
                .unwrap_or_else(|| panic!("Error with sap-addr"))
                .to_bytes()
                .to_vec(),
        },
        orchard: ZcashFullViewingKey {
            data: ufvk
                .orchard()
                .unwrap_or_else(|| panic!("Error with orc-addr"))
                .to_bytes()
                .to_vec(),
        },
        encoded: ufvk.encode(network),
    }
}

pub fn unified_fvk_from_usk(zusk: ZcashUnifiedSpendingKey) -> ZcashUnifiedFullViewingKey {
    let usk_bin: Vec<u8> = zusk.binary;
    let decoded = UnifiedSpendingKey::from_bytes(Era::Orchard, &usk_bin[..]);
    let usk: UnifiedSpendingKey = decoded.unwrap_or_else(|e| panic!("Error decoding USK: {e:?}"));
    let ufvk: UnifiedFullViewingKey = usk.to_unified_full_viewing_key();

    match zusk.network {
        Network::MAIN => ufvk_to_external(ufvk, &MAIN_NETWORK),
        Network::TEST => ufvk_to_external(ufvk, &TEST_NETWORK),
    }
}

pub fn deserialize_ufvk(encoded: &str, network: Network) -> Option<ZcashUnifiedFullViewingKey> {
    match network {
        Network::MAIN => UnifiedFullViewingKey::decode(&MAIN_NETWORK, encoded)
            .map(|ufvk| ufvk_to_external(ufvk, &MAIN_NETWORK))
            .ok(),
        Network::TEST => UnifiedFullViewingKey::decode(&TEST_NETWORK, encoded)
            .map(|ufvk| ufvk_to_external(ufvk, &TEST_NETWORK))
            .ok(),
    }
}

pub fn ivk_from_ufvk(encoded: &str, network: Network, scope: Scope) -> Option<ZcashSaplingIvk> {
    let some_ufvk = match network {
        Network::MAIN => UnifiedFullViewingKey::decode(&MAIN_NETWORK, encoded).ok(),
        Network::TEST => UnifiedFullViewingKey::decode(&TEST_NETWORK, encoded).ok(),
    };

    some_ufvk.as_ref()?;

    let data: Vec<u8> = match scope {
        Scope::EXTERNAL => some_ufvk.unwrap().sapling()?.to_ivk(ZScope::External),
        Scope::INTERNAL => some_ufvk.unwrap().sapling()?.to_ivk(ZScope::Internal),
    }
    .to_repr()
    .to_vec();

    Some(ZcashSaplingIvk { data })
}

pub fn ovk_from_ufvk(
    encoded: &str,
    network: Network,
    scope: Scope,
) -> Option<ZcashOutgoingViewingKey> {
    let some_ufvk = match network {
        Network::MAIN => UnifiedFullViewingKey::decode(&MAIN_NETWORK, encoded).ok(),
        Network::TEST => UnifiedFullViewingKey::decode(&TEST_NETWORK, encoded).ok(),
    };

    some_ufvk.as_ref()?;

    let data: Vec<u8> = match scope {
        Scope::EXTERNAL => some_ufvk.unwrap().sapling()?.to_ovk(ZScope::External),
        Scope::INTERNAL => some_ufvk.unwrap().sapling()?.to_ovk(ZScope::Internal),
    }
    .0
    .to_vec();

    Some(ZcashOutgoingViewingKey { data })
}

pub fn ivk_to_payment_address(ivk: ZcashSaplingIvk, diversifier_bytes: Vec<u8>) -> Vec<u8> {
    let ivk_data: [u8; 32] = ivk.data[..]
        .try_into()
        .expect("slice ivk.data with incorrect length");
    let ntv_ivk = SaplingIvk(jubjub::Fr::from_repr(ivk_data).unwrap());
    let slc: [u8; 11] = diversifier_bytes[..]
        .try_into()
        .expect("slice diversifier_bytes with incorrect length");
    ntv_ivk
        .to_payment_address(Diversifier(slc))
        .unwrap()
        .to_bytes()
        .to_vec()
}

pub fn from_seed(seed: Vec<u8>, account_id: u32) -> Option<ZcashAccountPrivKey> {
    if seed.len() < 32 {
        panic!("ZIP 32 seeds MUST be at least 32 bytes");
    }

    AccountPrivKey::from_seed(&MAIN_NETWORK, &seed[..32], AccountId::from(account_id))
        .map(|priv_key| {
            Some(ZcashAccountPrivKey {
                data: priv_key.to_bytes(),
            })
        })
        .unwrap()
}

pub fn test_from_seed(seed: Vec<u8>, account_id: u32) -> Option<ZcashAccountPrivKey> {
    if seed.len() < 32 {
        panic!("ZIP 32 seeds MUST be at least 32 bytes");
    }

    AccountPrivKey::from_seed(&TEST_NETWORK, &seed[..32], AccountId::from(account_id))
        .map(|priv_key| {
            Some(ZcashAccountPrivKey {
                data: priv_key.to_bytes(),
            })
        })
        .unwrap()
}

pub fn from_bytes(data: Vec<u8>) -> Option<ZcashAccountPrivKey> {
    AccountPrivKey::from_bytes(&data).map(|data| ZcashAccountPrivKey {
        data: data.to_bytes(),
    })
}

pub fn to_account_pubkey(priv_key: &ZcashAccountPrivKey) -> ZcashAccountPubKey {
    let serialized_data = AccountPrivKey::from_bytes(&priv_key.data)
        .unwrap()
        .to_account_pubkey()
        .serialize();

    ZcashAccountPubKey {
        data: serialized_data,
    }
}

// pub fn derive_external_ivk()

// pub fn derive_sk_from_pk(priv_key: &ZcashAccountPrivKey) -> Option<ZcashAccountSpendingKey> {}

// pub fn create_spending_key() -> ZcashAccountSpendingKey {}

uniffi::include_scaffolding!("backend_keys");
