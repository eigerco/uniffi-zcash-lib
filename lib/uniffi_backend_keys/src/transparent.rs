use zcash_primitives::legacy;

/// A transparent address corresponding to either a public key or a script.
pub struct ZcashTransparentAddress(legacy::TransparentAddress);

impl From<legacy::TransparentAddress> for ZcashTransparentAddress {
    fn from(value: legacy::TransparentAddress) -> Self {
        ZcashTransparentAddress(value)
    }
}

impl From<ZcashTransparentAddress> for legacy::TransparentAddress {
    fn from(value: ZcashTransparentAddress) -> Self {
        value.0
    }
}

impl ZcashTransparentAddress {
    /// Create new transparent address corresponding to public key
    pub fn public_key(input: Vec<u8>) -> crate::ZcashResult<Self> {
        let buf = crate::utils::cast_slice(&input)?;
        Ok(legacy::TransparentAddress::PublicKey(buf).into())
    }

    /// Create new transparent address corresponding to script
    pub fn script(input: Vec<u8>) -> crate::ZcashResult<Self> {
        let buf = crate::utils::cast_slice(&input)?;
        Ok(legacy::TransparentAddress::Script(buf).into())
    }
}
