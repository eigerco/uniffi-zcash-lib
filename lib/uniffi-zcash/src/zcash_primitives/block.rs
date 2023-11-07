use zcash_primitives::block::BlockHash;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ZcashBlockHash(BlockHash);

impl ZcashBlockHash {
    pub fn from_slice(bytes: &[u8]) -> Self {
        ZcashBlockHash(BlockHash::from_slice(bytes))
    }
}

impl From<ZcashBlockHash> for BlockHash {
    fn from(inner: ZcashBlockHash) -> Self {
        inner.0
    }
}

impl From<BlockHash> for ZcashBlockHash {
    fn from(e: BlockHash) -> Self {
        Self(e)
    }
}
