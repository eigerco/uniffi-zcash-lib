use zcash_client_backend::encoding;
use zcash_primitives::consensus::Parameters;
use zcash_primitives::legacy::{self, TransparentAddress::*};

/// A transparent address corresponding to either a public key or a script.
#[derive(Clone, Copy)]
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
    pub fn parse(params: crate::ZcashConsensusParameters, input: &str) -> crate::ZcashResult<Self> {
        encoding::decode_transparent_address(
            &params.b58_pubkey_address_prefix(),
            &params.b58_script_address_prefix(),
            input,
        )
        .map_err(|_| crate::ZcashError::Unknown)?
        .ok_or(crate::ZcashError::Unknown)
        .map(Into::into)
    }

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

    pub fn encode(&self, params: crate::ZcashConsensusParameters) -> String {
        encoding::encode_transparent_address_p(&params, &self.0)
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
