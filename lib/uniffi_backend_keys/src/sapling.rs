use std::sync::Arc;

use zcash_primitives::sapling;

/// A Sapling payment address.
///
/// # Invariants
///
/// `pk_d` is guaranteed to be prime-order (i.e. in the prime-order subgroup of Jubjub,
/// and not the identity).
#[derive(Clone)]
pub struct ZcashPaymentAddress(sapling::PaymentAddress);

impl From<sapling::PaymentAddress> for ZcashPaymentAddress {
    fn from(inner: sapling::PaymentAddress) -> Self {
        ZcashPaymentAddress(inner)
    }
}

impl From<ZcashPaymentAddress> for sapling::PaymentAddress {
    fn from(value: ZcashPaymentAddress) -> Self {
        value.0
    }
}

impl ZcashPaymentAddress {
    pub fn from_bytes(bytes: &[u8]) -> crate::ZcashResult<Self> {
        let bytes = crate::utils::cast_slice(bytes)?;
        sapling::PaymentAddress::from_bytes(&bytes)
            .map(ZcashPaymentAddress)
            .ok_or(crate::ZcashError::Unknown)
    }

    /// Returns the byte encoding of this `PaymentAddress`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().into()
    }

    pub fn diversifier(&self) -> Arc<crate::ZcashDiversifier> {
        Arc::new(self.0.diversifier().clone().into())
    }
}

pub struct ZcashDiversifier(sapling::Diversifier);

impl From<&ZcashDiversifier> for sapling::Diversifier {
    fn from(value: &ZcashDiversifier) -> Self {
        value.0
    }
}

impl From<sapling::Diversifier> for ZcashDiversifier {
    fn from(inner: zcash_primitives::sapling::Diversifier) -> Self {
        ZcashDiversifier(inner)
    }
}

impl ZcashDiversifier {
    pub fn new(bytes: Vec<u8>) -> crate::ZcashResult<Self> {
        let array = crate::utils::cast_slice(&bytes)?;

        Ok(sapling::Diversifier(array).into())
    }
}
