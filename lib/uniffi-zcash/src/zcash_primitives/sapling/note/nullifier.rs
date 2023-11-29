use zcash_primitives::sapling::Nullifier;

use derive_more::{From, Into};

#[derive(Copy, Clone, PartialEq, Eq, From, Into)]
pub struct ZcashSaplingNullifier(Nullifier);

impl ZcashSaplingNullifier {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}
