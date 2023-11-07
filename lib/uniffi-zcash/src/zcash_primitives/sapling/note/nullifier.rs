use zcash_primitives::sapling::Nullifier;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ZcashSaplingNullifier(Nullifier);

impl ZcashSaplingNullifier {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl From<ZcashSaplingNullifier> for Nullifier {
    fn from(outer: ZcashSaplingNullifier) -> Self {
        outer.0
    }
}

impl From<Nullifier> for ZcashSaplingNullifier {
    fn from(inner: Nullifier) -> Self {
        Self(inner)
    }
}

impl From<&Nullifier> for ZcashSaplingNullifier {
    fn from(inner: &Nullifier) -> Self {
        Self(*inner)
    }
}
