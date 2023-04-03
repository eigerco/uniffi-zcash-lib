use std::sync::Arc;

use zcash_client_backend::encoding;

use crate::{
    ZcashConsensusParameters, ZcashError, ZcashExtendedFullViewingKey, ZcashExtendedSpendingKey,
    ZcashPaymentAddress, ZcashResult, ZcashTransparentAddress,
};

// TODO at least some of the below functions exist somewhere else, like encode_extended_full_viewing_key

/// Writes an [`ExtendedSpendingKey`] as a Bech32-encoded string.
pub fn encode_extended_spending_key(hrp: &str, extsk: Arc<ZcashExtendedSpendingKey>) -> String {
    encoding::encode_extended_spending_key(hrp, &extsk.as_ref().into())
}

/// Decodes an [`ExtendedSpendingKey`] from a Bech32-encoded string.
pub fn decode_extended_spending_key(
    hrp: &str,
    s: &str,
) -> ZcashResult<Arc<ZcashExtendedSpendingKey>> {
    encoding::decode_extended_spending_key(hrp, s)
        .map_err(From::from)
        .map(From::from)
        .map(Arc::new)
}

/// Writes an [`ExtendedFullViewingKey`] as a Bech32-encoded string.
pub fn encode_extended_full_viewing_key(
    hrp: &str,
    extfvk: Arc<ZcashExtendedFullViewingKey>,
) -> String {
    encoding::encode_extended_full_viewing_key(hrp, &extfvk.as_ref().into())
}

/// Decodes an [`ExtendedFullViewingKey`] from a Bech32-encoded string.
pub fn decode_extended_full_viewing_key(
    hrp: &str,
    s: &str,
) -> ZcashResult<Arc<ZcashExtendedFullViewingKey>> {
    encoding::decode_extended_full_viewing_key(hrp, s)
        .map_err(From::from)
        .map(From::from)
        .map(Arc::new)
}

/// Decodes an [`ExtendedFullViewingKey`] from a Bech32-encoded string.
pub fn encode_payment_address(hrp: &str, addr: Arc<ZcashPaymentAddress>) -> String {
    encoding::encode_payment_address(hrp, &addr.as_ref().into())
}

/// Writes a [`PaymentAddress`] as a Bech32-encoded string
/// using the human-readable prefix values defined in the specified
/// network parameters.
pub fn encode_payment_address_p(
    params: ZcashConsensusParameters,
    addr: Arc<ZcashPaymentAddress>,
) -> String {
    encoding::encode_payment_address_p(&params, &addr.as_ref().into())
}

/// Decodes a [`PaymentAddress`] from a Bech32-encoded string.
pub fn decode_payment_address(hrp: &str, s: &str) -> ZcashResult<Arc<ZcashPaymentAddress>> {
    encoding::decode_payment_address(hrp, s)
        .map_err(From::from)
        .map(From::from)
        .map(Arc::new)
}

/// Writes a [`TransparentAddress`] as a Base58Check-encoded string.
pub fn encode_transparent_address(
    pubkey_version: &[u8],
    script_version: &[u8],
    addr: Arc<ZcashTransparentAddress>,
) -> String {
    encoding::encode_transparent_address(pubkey_version, script_version, &addr.as_ref().into())
}

/// Writes a [`TransparentAddress`] as a Base58Check-encoded string.
/// using the human-readable prefix values defined in the specified
/// network parameters.
pub fn encode_transparent_address_p(
    params: ZcashConsensusParameters,
    addr: Arc<ZcashTransparentAddress>,
) -> String {
    encoding::encode_transparent_address_p(&params, &addr.as_ref().into())
}

/// Decodes a [`TransparentAddress`] from a Base58Check-encoded string.
pub fn decode_transparent_address(
    pubkey_version: &[u8],
    script_version: &[u8],
    s: &str,
) -> ZcashResult<Arc<ZcashTransparentAddress>> {
    encoding::decode_transparent_address(pubkey_version, script_version, s)
        .map_err(ZcashError::from)?
        .map(From::from)
        .map(Arc::new)
        .ok_or("error ocurred while decoding transparent address".into())
}
