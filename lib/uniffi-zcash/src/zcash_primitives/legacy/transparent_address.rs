use zcash_client_backend::encoding;
use zcash_primitives::{consensus::Parameters, legacy::TransparentAddress};

use crate::{utils, ZcashConsensusParameters, ZcashError, ZcashResult};

/// A transparent address corresponding to either a public key or a script.
#[derive(Clone, Copy)]
pub struct ZcashTransparentAddress(TransparentAddress);

impl From<TransparentAddress> for ZcashTransparentAddress {
    fn from(address: TransparentAddress) -> Self {
        ZcashTransparentAddress(address)
    }
}

impl From<ZcashTransparentAddress> for TransparentAddress {
    fn from(value: ZcashTransparentAddress) -> Self {
        value.0
    }
}

impl ZcashTransparentAddress {
    pub fn parse(params: ZcashConsensusParameters, input: &str) -> ZcashResult<Self> {
        encoding::decode_transparent_address(
            &params.b58_pubkey_address_prefix(),
            &params.b58_script_address_prefix(),
            input,
        )
        .map_err(|_| ZcashError::Unknown)?
        .ok_or(ZcashError::Unknown)
        .map(Into::into)
    }

    /// Create new transparent address corresponding to public key
    pub fn public_key(input: Vec<u8>) -> ZcashResult<Self> {
        let buf = utils::cast_slice(&input)?;
        Ok(TransparentAddress::PublicKey(buf).into())
    }

    /// Create new transparent address corresponding to script
    pub fn script(input: Vec<u8>) -> ZcashResult<Self> {
        let buf = utils::cast_slice(&input)?;
        Ok(TransparentAddress::Script(buf).into())
    }

    pub fn encode(&self, params: ZcashConsensusParameters) -> String {
        encoding::encode_transparent_address_p(&params, &self.0)
    }

    /// Check if it is public key transparent address
    pub fn is_public_key(&self) -> bool {
        matches!(self.0, TransparentAddress::PublicKey(_))
    }

    /// Check if it is script transparent address
    pub fn is_script(&self) -> bool {
        matches!(self.0, TransparentAddress::Script(_))
    }

    /// Return raw bytes corresponding to given address
    pub fn to_bytes(&self) -> Vec<u8> {
        match self.0 {
            TransparentAddress::PublicKey(bytes) => bytes.into(),
            TransparentAddress::Script(bytes) => bytes.into(),
        }
    }
}
