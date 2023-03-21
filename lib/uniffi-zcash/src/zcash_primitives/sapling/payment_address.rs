use std::sync::Arc;

use zcash_client_backend::encoding;
use zcash_primitives::{consensus::Parameters, sapling::PaymentAddress};

/// A Sapling payment address.
///
/// # Invariants
///
/// `pk_d` is guaranteed to be prime-order (i.e. in the prime-order subgroup of Jubjub,
/// and not the identity).
#[derive(Clone)]
pub struct ZcashPaymentAddress(PaymentAddress);

impl From<PaymentAddress> for ZcashPaymentAddress {
    fn from(address: PaymentAddress) -> Self {
        ZcashPaymentAddress(address)
    }
}

impl From<&ZcashPaymentAddress> for PaymentAddress {
    fn from(value: &ZcashPaymentAddress) -> Self {
        value.0
    }
}

impl ZcashPaymentAddress {
    /// Parse the input string into `ZcashPaymentAddress`
    pub fn parse(
        params: crate::ZcashConsensusParameters,
        string: &str,
    ) -> crate::ZcashResult<Self> {
        let address =
            encoding::decode_payment_address(params.hrp_sapling_payment_address(), string)
                // TODO: This is just mock we should use `zcash_client_backend::encoding::Bech32DecodeError` there,
                // but for whatever reason, that enum is currently not implementing the `Error` trait.
                .map_err(|_| crate::ZcashError::Unknown)?;

        Ok(address.into())
    }

    pub fn from_bytes(bytes: &[u8]) -> crate::ZcashResult<Self> {
        let bytes = crate::utils::cast_slice(bytes)?;
        PaymentAddress::from_bytes(&bytes)
            .map(ZcashPaymentAddress)
            .ok_or(crate::ZcashError::Unknown)
    }

    /// Encode payment address into string
    pub fn encode(&self, params: crate::ZcashConsensusParameters) -> String {
        encoding::encode_payment_address_p(&params, &self.0)
    }

    /// Returns the byte encoding of this `PaymentAddress`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().into()
    }

    pub fn diversifier(&self) -> Arc<crate::ZcashDiversifier> {
        Arc::new((*self.0.diversifier()).into())
    }
}
