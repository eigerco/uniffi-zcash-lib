use jubjub::Fr;

use crate::{utils::cast_slice, ZcashResult};

use derive_more::{From, Into};

/// Represents an element of the scalar field $\mathbb{F}_r$ of the Jubjub elliptic
/// curve construction.
// The internal representation of this type is four 64-bit unsigned
// integers in little-endian order. Elements of Fr are always in
// Montgomery form; i.e., Fr(a) = aR mod r, with R = 2^256.
#[derive(From, Into)]
pub struct ZcashJubjubFr(Fr);

impl ZcashJubjubFr {
    pub fn from_bytes(data: &[u8]) -> ZcashResult<Self> {
        let casted_data = cast_slice(data)?;
        let opt: Option<Fr> = Fr::from_bytes(&casted_data).into();
        match opt {
            Some(fr) => Ok(fr.into()),
            None => Err("Error parsing data.".into()),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}
