use zcash_primitives::memo::MemoBytes;

use crate::{ZcashError, ZcashResult};

#[derive(Debug, Clone)]
pub struct ZcashMemoBytes(MemoBytes);

impl ZcashMemoBytes {
    // Creates a `MemoBytes` from a slice, exactly as provided.
    ///
    /// Returns an error if the provided slice is longer than 512 bytes. Slices shorter
    /// than 512 bytes are padded with null bytes.
    ///
    /// Note that passing an empty slice to this API (or an all-zeroes slice) will result
    /// in a memo representing an empty string. What you almost certainly want in this
    /// case is [`MemoBytes::empty`], which uses a specific encoding to indicate that no
    /// memo is present.
    pub fn new(data: &[u8]) -> ZcashResult<Self> {
        let memo = MemoBytes::from_bytes(data).map_err(|_| ZcashError::ArrayLengthMismatch {
            expected: 512,
            got: data.len() as u64,
        })?;

        Ok(ZcashMemoBytes(memo))
    }

    /// Creates a `MemoBytes` indicating that no memo is present.
    pub fn empty() -> Self {
        ZcashMemoBytes(MemoBytes::empty())
    }

    /// Returns a slice of the raw bytes, excluding null padding.
    pub fn data(&self) -> Vec<u8> {
        self.0.as_slice().to_owned()
    }
}

// maybe implement it for all the other types?
impl From<MemoBytes> for ZcashMemoBytes {
    fn from(value: MemoBytes) -> Self {
        ZcashMemoBytes(value)
    }
}

impl From<ZcashMemoBytes> for MemoBytes {
    fn from(value: ZcashMemoBytes) -> Self {
        value.0
    }
}

impl From<&ZcashMemoBytes> for MemoBytes {
    fn from(value: &ZcashMemoBytes) -> Self {
        value.0.clone()
    }
}
