use std::sync::Arc;

use zcash_client_backend::encoding;
use zcash_primitives::{consensus::Parameters, sapling::PaymentAddress};

use crate::{
    utils, ZcashConsensusParameters, ZcashDiversifier, ZcashError, ZcashResult, ZcashRseed,
    ZcashSaplingDiversifiedTransmissionKey, ZcashSaplingNote,
};

/// A Sapling payment address.
///
/// # Invariants
///
/// `pk_d` is guaranteed to be prime-order (i.e. in the prime-order subgroup of Jubjub,
/// and not the identity).
#[derive(Clone)]
pub struct ZcashPaymentAddress(PaymentAddress);

impl ZcashPaymentAddress {
    /// Decodes a [`PaymentAddress`] from a Bech32-encoded string.
    pub fn decode(params: ZcashConsensusParameters, string: &str) -> ZcashResult<Self> {
        encoding::decode_payment_address(params.hrp_sapling_payment_address(), string)
            .map_err(From::from)
            .map(From::from)
    }

    /// Parses a PaymentAddress from bytes.
    pub fn from_bytes(bytes: &[u8]) -> ZcashResult<Self> {
        let bytes = utils::cast_slice(bytes)?;
        PaymentAddress::from_bytes(&bytes)
            .map(ZcashPaymentAddress)
            .ok_or(ZcashError::Unknown)
    }

    /// Encode payment address into string
    pub fn encode(&self, params: ZcashConsensusParameters) -> String {
        encoding::encode_payment_address_p(&params, &self.0)
    }

    /// Returns the byte encoding of this `PaymentAddress`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().into()
    }

    /// Returns the [`Diversifier`] for this `PaymentAddress`.
    pub fn diversifier(&self) -> Arc<ZcashDiversifier> {
        Arc::new((*self.0.diversifier()).into())
    }

    /// Returns `pk_d` for this `PaymentAddress`.
    pub fn pk_d(&self) -> Arc<ZcashSaplingDiversifiedTransmissionKey> {
        Arc::new((*self.0.pk_d()).into())
    }

    pub fn create_note(&self, value: u64, rseed: ZcashRseed) -> ZcashResult<Arc<ZcashSaplingNote>> {
        Ok(Arc::new(
            self.0.create_note(value, rseed.try_into()?).into(),
        ))
    }
}

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

impl From<ZcashPaymentAddress> for PaymentAddress {
    fn from(value: ZcashPaymentAddress) -> Self {
        value.0
    }
}
