use zcash_primitives::legacy::{self, TransparentAddress::*};

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
        Ok(PublicKey(buf).into())
    }

    /// Create new transparent address corresponding to script
    pub fn script(input: Vec<u8>) -> crate::ZcashResult<Self> {
        let buf = crate::utils::cast_slice(&input)?;
        Ok(Script(buf).into())
    }

    /// Check if it is public key transparent address
    pub fn is_public_key(&self) -> bool {
        match self.0 {
            PublicKey(_) => true,
            _ => false,
        }
    }

    /// Check if it is script transparent address
    pub fn is_script(&self) -> bool {
        match self.0 {
            Script(_) => true,
            _ => false,
        }
    }

    /// Return raw bytes corresponding to given address
    pub fn to_bytes(&self) -> Vec<u8> {
        match self.0 {
            PublicKey(bytes) => bytes.into(),
            Script(bytes) => bytes.into(),
        }
    }
}
