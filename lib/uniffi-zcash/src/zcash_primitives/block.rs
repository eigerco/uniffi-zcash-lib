use zcash_primitives::block::BlockHash;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ZcashBlockHash(BlockHash);

impl ZcashBlockHash {
    pub fn from_slice(bytes: &[u8]) -> Self {
        ZcashBlockHash(BlockHash::from_slice(bytes))
    }
}
