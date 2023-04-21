use zcash_primitives::sapling::redjubjub::PublicKey;

use crate::ZcashResult;

pub struct ZcashSaplingPublicKey(PublicKey);

impl ZcashSaplingPublicKey {
    pub fn to_bytes(&self) -> ZcashResult<Vec<u8>> {
        let mut data = Vec::with_capacity(32);
        self.0.write(&mut data)?;
        Ok(data)
    }
}

impl From<&PublicKey> for ZcashSaplingPublicKey {
    fn from(inner: &PublicKey) -> Self {
        ZcashSaplingPublicKey(inner.clone())
    }
}
