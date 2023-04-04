mod parameters;
pub use self::parameters::*;

use zcash_primitives::consensus::BlockHeight;

#[derive(Clone, Copy)]
pub struct ZcashBlockHeight(BlockHeight);

impl ZcashBlockHeight {
    pub fn new(v: u32) -> Self {
        ZcashBlockHeight(BlockHeight::from_u32(v))
    }
}

impl From<ZcashBlockHeight> for BlockHeight {
    fn from(value: ZcashBlockHeight) -> Self {
        value.0
    }
}
