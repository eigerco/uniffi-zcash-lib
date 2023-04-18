mod note;
use zcash_primitives::sapling::value::ValueCommitment;

pub use self::note::*;

pub struct ZcashSaplingValueCommitment(ValueCommitment);

impl ZcashSaplingValueCommitment {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

impl From<&ValueCommitment> for ZcashSaplingValueCommitment {
    fn from(inner: &ValueCommitment) -> Self {
        ZcashSaplingValueCommitment(inner.clone())
    }
}