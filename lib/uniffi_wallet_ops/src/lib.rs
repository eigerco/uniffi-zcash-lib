// use orchard::Address as OrchardAddress;
// use zcash_address::unified::{Address, Encoding};
use zcash_client_backend::address::UnifiedAddress;
use zcash_client_backend::encoding::{decode_payment_address, decode_transparent_address};
use zcash_client_backend::zip321::{memo_from_base64, memo_to_base64, TransactionRequest};
use zcash_primitives::consensus::{Parameters, MAIN_NETWORK, TEST_NETWORK};
use zcash_primitives::constants;
use zcash_primitives::legacy::TransparentAddress;
use zcash_primitives::sapling::PaymentAddress as SaplingAddress;
// use zcash_primitives::zip32::AccountId;

uniffi::include_scaffolding!("wallet_ops");

pub struct ZcashOrchardAddress {
    data: Vec<u8>,
}

pub struct ZcashSaplingAddress {
    data: Vec<u8>,
}

pub struct ZcashTransparentAddress {
    data: Vec<u8>,
}

pub struct ZcashAccount {
    account_id: u32,
}

pub struct ZcashTransactionRequest {
    payments: Vec<ZcashPayment>,
}

pub struct ZcashUnifiedAddress {
    orchard: Option<ZcashOrchardAddress>,
    sapling: Option<ZcashSaplingAddress>,
    transparent: Option<ZcashTransparentAddress>,
    network: Network,
    // unknown?
}

pub struct ZcashKeyValuePair {
    key: String,
    value: String,
}

pub struct ZcashMemoBytes {
    data: Vec<u8>,
}

pub struct ZcashPayment {
    label: Option<String>,
    message: Option<String>,
    recipient_address: ZcashUnifiedAddress, // RecipientAddress
    other_params: Vec<ZcashKeyValuePair>,
    memo: Option<ZcashMemoBytes>,
    amount: i64,
}

pub enum ZcashParsedAddress {
    Shielded { z_address: ZcashSaplingAddress }, // PaymentAddress
    Transparent { t_address: ZcashTransparentAddress },
    Unified { u_address: ZcashUnifiedAddress },
}

pub enum Network {
    MAIN,
    TEST,
}

pub enum AddrType {
    ZSAPLING,
    ZORCHARD,
    TRANSPARENT,
}

// Create a new Account
// - create_account from data_api::wallet.rs
//   but it's abstract, so we cannot use it as a trait.
// we have to force the user to introduce on their own id
// for the input integer, since we cannot make any assumption
// about the Data Model in usage
pub fn create_account(account_id: u32) -> ZcashAccount {
    ZcashAccount { account_id }
}

// Create a new Unified Address from an Account
// (account, _usk)
// For this it's necessary to access the data model

// pub fn unified_address_from_account(_zaccount: ZcashAccount) -> Option<ZcashUnifiedAddress> {
// zaccount.account_id

// let data_file = NamedTempFile::new().unwrap();
// let mut db_data = WalletDb::for_path(data_file.path(), tests::network()).unwrap();
// let mut ops = db_data.get_update_ops().unwrap();
// let (account, _usk) = ops.create_account(&Secret::new(seed.to_vec())).unwrap();

// construct sapling :: PaymentAddress
// from_parts(diversifier: Diversifier, pk_d: jubjub::SubgroupPoint)

// construct orchard::Address

// construct TransparentAddress

// construct UnifiedAddress::from_receivers(
//     orchard: Option<orchard::Address>,
//     sapling: Option<PaymentAddress>,
//     transparent: Option<TransparentAddress>,
// )
// None
// }

pub fn make_unified_address(
    network: Network,
    orchard: Option<ZcashOrchardAddress>,
    sapling: Option<ZcashSaplingAddress>,
    transparent: Option<ZcashTransparentAddress>,
) -> Option<ZcashUnifiedAddress> {
    if orchard.is_some() || sapling.is_some() {
        Some(ZcashUnifiedAddress {
            orchard,
            sapling,
            transparent,
            network,
        })
    } else {
        None
    }
}

pub fn encode_unified_address(zua: ZcashUnifiedAddress) -> Option<String> {
    let transparent = zua
        .transparent
        .and_then(|addr| prv_decode_transparent_address(&zua.network, &bytes_to_string(addr.data)));

    let sapling = zua
        .sapling
        .and_then(|addr| prv_decode_sapling_address(&zua.network, &bytes_to_string(addr.data)));

    let orchard = None;

    if let Some(ua) = UnifiedAddress::from_receivers(orchard, sapling, transparent) {
        match zua.network {
            Network::MAIN => Some(ua.encode(&MAIN_NETWORK)),
            Network::TEST => Some(ua.encode(&TEST_NETWORK)),
        }
    } else {
        None
    }
}

// Parse and validate a Unified Address
pub fn parse_unified_address(_u_address: String) -> Option<ZcashUnifiedAddress> {
    // Address::decode(address)
    // unified::Address::decode(address)
    // .map_err(|e| format!("{}", e))
    // .and_then(|(network, addr)| {
    //     if params.address_network() == Some(network) {
    //         UnifiedAddress::try_from(addr).map_err(|e| e.to_owned())
    //     } else {
    //         Err(format!(
    //             "Address {} is for a different network: {:?}",
    //             address, network
    //         ))
    //     }
    // })
    None
}

/// Parse and validate a Sapling or Orchard Z-address
pub fn parse_sapling_address(network: Network, z_address: String) -> Option<ZcashSaplingAddress> {
    if prv_decode_sapling_address(&network, &z_address).is_some() {
        Some(ZcashSaplingAddress {
            data: z_address.into_bytes(),
        })
    } else {
        None
    }
}

/// Parse and validate a Sapling or Orchard Z-address
pub fn parse_orchard_address(_network: Network, _z_address: String) -> Option<ZcashOrchardAddress> {
    // decode_payment_address(z_address)
    None
}

/// Parse and validate a Transparent T-address
pub fn parse_transparent_address(
    network: Network,
    t_address: String,
) -> Option<ZcashTransparentAddress> {
    if prv_decode_transparent_address(&network, &t_address).is_some() {
        Some(ZcashTransparentAddress {
            data: t_address.into_bytes(),
        })
    } else {
        None
    }
}

pub fn encode_transparent_address(t_address: ZcashTransparentAddress) -> String {
    bytes_to_string(t_address.data)
}

pub fn encode_sapling_address(z_address: ZcashSaplingAddress) -> String {
    bytes_to_string(z_address.data)
}

pub fn encode_orchard_address(z_address: ZcashOrchardAddress) -> String {
    bytes_to_string(z_address.data)
}

fn prv_decode_sapling_address(network: &Network, z_address: &str) -> Option<SaplingAddress> {
    match network {
        Network::MAIN => {
            decode_payment_address(constants::mainnet::HRP_SAPLING_PAYMENT_ADDRESS, z_address)
        }
        Network::TEST => {
            decode_payment_address(constants::testnet::HRP_SAPLING_PAYMENT_ADDRESS, z_address)
        }
    }
    .ok()
}

fn prv_decode_transparent_address(
    network: &Network,
    t_address: &str,
) -> Option<TransparentAddress> {
    match network {
        Network::MAIN => decode_transparent_address(
            &MAIN_NETWORK.b58_pubkey_address_prefix(),
            &MAIN_NETWORK.b58_script_address_prefix(),
            t_address,
        ),
        Network::TEST => decode_transparent_address(
            &TEST_NETWORK.b58_pubkey_address_prefix(),
            &TEST_NETWORK.b58_script_address_prefix(),
            t_address,
        ),
    }
    .ok()
    .flatten()
}

fn bytes_to_string(buf: Vec<u8>) -> String {
    match std::str::from_utf8(&buf) {
        Ok(v) => v.to_owned(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}

pub fn is_address_valid(_address: String) -> bool {
    false
}

// /// Parse and validate Payment disclosures
// pub fn validate_payment_disclosures() {

// }

// /// Verify Payment disclosures
// pub fn verify_payment_disclosure() {

// }

pub fn get_address_type(_address: String) -> Option<AddrType> {
    Some(AddrType::TRANSPARENT)
}

/// Parse Payment requests
pub fn transaction_request_from_uri(_uri: String) -> Option<ZcashTransactionRequest> {
    None
}

/// Produce Payment requests
pub fn transaction_request_to_uri(_ztr: ZcashTransactionRequest) -> Option<String> {
    Some("".to_owned())
}

/// Get a fee rate
pub fn get_fee_rate() -> f32 {
    0.0
}
