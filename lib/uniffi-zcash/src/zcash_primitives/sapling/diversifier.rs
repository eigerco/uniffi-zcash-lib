use zcash_primitives::sapling::Diversifier;

pub struct ZcashDiversifier(Diversifier);

impl From<&ZcashDiversifier> for Diversifier {
    fn from(value: &ZcashDiversifier) -> Self {
        value.0
    }
}

impl From<Diversifier> for ZcashDiversifier {
    fn from(diversifier: Diversifier) -> Self {
        ZcashDiversifier(diversifier)
    }
}

impl ZcashDiversifier {
    pub fn new(bytes: Vec<u8>) -> crate::ZcashResult<Self> {
        let array = crate::utils::cast_slice(&bytes)?;
        Ok(Diversifier(array).into())
    }
}
