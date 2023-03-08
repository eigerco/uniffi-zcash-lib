use zcash_primitives::zip32::ChildIndex;

/// A child index for a derived key
pub enum ZcashChildIndex {
    NonHardened { v: u32 },
    Hardened { v: u32 },
}

impl From<ZcashChildIndex> for ChildIndex {
    fn from(value: ZcashChildIndex) -> Self {
        match value {
            ZcashChildIndex::NonHardened { v } => ChildIndex::NonHardened(v),
            ZcashChildIndex::Hardened { v } => ChildIndex::Hardened(v),
        }
    }
}