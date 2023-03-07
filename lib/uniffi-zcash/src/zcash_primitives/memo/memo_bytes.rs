use zcash_primitives::memo::MemoBytes;

use crate::{ZcashError, ZcashResult};

#[derive(Clone)]
pub struct ZcashMemoBytes(MemoBytes);

impl ZcashMemoBytes {
    pub fn new(data: &[u8]) -> ZcashResult<Self> {
        let memo = MemoBytes::from_bytes(data).map_err(|_| ZcashError::Unknown)?;

        Ok(ZcashMemoBytes(memo))
    }

    pub fn data(&self) -> Vec<u8> {
        self.0.as_slice().to_owned()
    }
}
