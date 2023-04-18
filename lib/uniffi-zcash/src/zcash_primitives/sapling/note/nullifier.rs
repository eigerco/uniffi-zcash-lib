use zcash_primitives::sapling::Nullifier;

pub struct ZcashSaplingNullifier(Nullifier);

impl ZcashSaplingNullifier {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl From<&Nullifier> for ZcashSaplingNullifier {
    fn from(inner: &Nullifier) -> Self {
        ZcashSaplingNullifier(*inner)
    }
}