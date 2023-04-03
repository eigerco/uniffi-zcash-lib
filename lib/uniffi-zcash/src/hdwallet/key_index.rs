use hdwallet::KeyIndex;

use crate::ZcashResult;

/// KeyIndex indicates the key type and index of a child key.
pub struct ZcashKeyIndex(KeyIndex);

impl From<KeyIndex> for ZcashKeyIndex {
    fn from(value: KeyIndex) -> Self {
        ZcashKeyIndex(value)
    }
}

impl From<&ZcashKeyIndex> for KeyIndex {
    fn from(value: &ZcashKeyIndex) -> Self {
        value.0
    }
}

impl ZcashKeyIndex {
    /// DUPLICATE ?
    pub fn from_u32(i: u32) -> ZcashResult<Self> {
        KeyIndex::from_index(i).map_err(From::from).map(From::from)
    }

    /// Return raw index value
    pub fn raw_index(&self) -> u32 {
        self.0.raw_index()
    }

    /// Return normalize index, it will return index subtract 2 ** 31 for hardended key.
    pub fn normalize_index(&self) -> u32 {
        self.0.normalize_index()
    }

    /// Check index range.
    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }

    /// Generate Hardened KeyIndex from normalize index value.
    pub fn hardened_from_normalize_index(i: u32) -> ZcashResult<Self> {
        KeyIndex::hardened_from_normalize_index(i)
            .map_err(From::from)
            .map(From::from)
    }

    /// Generate KeyIndex from raw index value.
    pub fn from_index(i: u32) -> ZcashResult<Self> {
        KeyIndex::from_index(i).map_err(From::from).map(From::from)
    }
}
