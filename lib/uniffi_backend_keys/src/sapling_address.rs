/// A Sapling payment address.
///
/// # Invariants
///
/// `pk_d` is guaranteed to be prime-order (i.e. in the prime-order subgroup of Jubjub,
/// and not the identity).
pub struct ZcashPaymentAddress(zcash_primitives::sapling::PaymentAddress);

impl From<zcash_primitives::sapling::PaymentAddress> for ZcashPaymentAddress {
    fn from(inner: zcash_primitives::sapling::PaymentAddress) -> Self {
        ZcashPaymentAddress(inner)
    }
}

impl Into<zcash_primitives::sapling::PaymentAddress> for &ZcashPaymentAddress {
    fn into(self) -> zcash_primitives::sapling::PaymentAddress {
        self.0.clone()
    }
}

impl ZcashPaymentAddress {
    pub fn from_bytes(bytes: &[u8]) -> crate::ZcashResult<Self> {
        if bytes.len() != 43 {
            return Err(crate::ZcashError::ArrayLengthMismatch {
                expected: 43,
                got: bytes.len() as u64,
            });
        }

        zcash_primitives::sapling::PaymentAddress::from_bytes(bytes.try_into().unwrap())
            .map(ZcashPaymentAddress)
            .ok_or(crate::ZcashError::Unknown)
    }

    /// Returns the byte encoding of this `PaymentAddress`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().into()
    }
}
