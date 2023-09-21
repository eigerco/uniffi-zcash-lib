use std::fmt;
use std::sync::Arc;
use zcash_client_backend::data_api::scanning::{ScanPriority, ScanRange};
use zcash_primitives::consensus::BlockHeight;

use crate::ZcashBlockHeight;

/// Scanning range priority levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ZcashScanPriority {
    /// Block ranges that are ignored have lowest priority.
    Ignored,
    /// Block ranges that have already been scanned will not be re-scanned.
    Scanned,
    /// Block ranges to be scanned to advance the fully-scanned height.
    Historic,
    /// Block ranges adjacent to heights at which the user opened the wallet.
    OpenAdjacent,
    /// Blocks that must be scanned to complete note commitment tree shards adjacent to found notes.
    FoundNote,
    /// Blocks that must be scanned to complete the latest note commitment tree shard.
    ChainTip,
    /// A previously scanned range that must be verified to check it is still in the
    /// main chain, has highest priority.
    Verify,
}

impl From<ZcashScanPriority> for ScanPriority {
    fn from(zsp: ZcashScanPriority) -> Self {
        match zsp {
            ZcashScanPriority::Ignored => Self::Ignored,
            _ => todo!(),
        }
    }
}

impl From<ScanPriority> for ZcashScanPriority {
    fn from(zsp: ScanPriority) -> Self {
        match zsp {
            ScanPriority::Ignored => Self::Ignored,
            _ => todo!(),
        }
    }
}

/// A range of blocks to be scanned, along with its associated priority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZcashScanRange(ScanRange);

impl fmt::Display for ZcashScanRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}({}..{})",
            self.0.priority(),
            self.0.block_range().start,
            self.0.block_range().end,
        )
    }
}

impl ZcashScanRange {
    /// Constructs a scan range from its constituent parts.
    /// vector of two elements, start and end
    pub fn from_parts(
        start_block: Arc<ZcashBlockHeight>,
        end_block: Arc<ZcashBlockHeight>,
        priority: ZcashScanPriority,
    ) -> Self {
        let start: BlockHeight = (*start_block).into();
        let end: BlockHeight = (*end_block).into();

        Self(ScanRange::from_parts(start..end, priority.into()))
    }

    /// Returns the range of block heights to be scanned.
    pub fn block_range(&self) -> Vec<Arc<ZcashBlockHeight>> {
        let range = self.0.block_range();
        let start: ZcashBlockHeight = range.start.into();
        let end: ZcashBlockHeight = range.end.into();
        vec![Arc::new(start), Arc::new(end)]
    }

    /// Returns the priority with which the scan range should be scanned.
    pub fn priority(&self) -> ZcashScanPriority {
        self.0.priority().into()
    }

    /// Returns whether or not the scan range is empty.
    pub fn is_empty(&self) -> bool {
        self.0.block_range().is_empty()
    }

    /// Returns the number of blocks in the scan range.
    pub fn len(&self) -> u32 {
        u32::try_from(self.0.len()).unwrap()
    }

    // truncate_start, truncate_end, split_at left
}

impl From<ZcashScanRange> for ScanRange {
    fn from(outer: ZcashScanRange) -> Self {
        outer.0
    }
}

impl From<ScanRange> for ZcashScanRange {
    fn from(inner: ScanRange) -> Self {
        ZcashScanRange(inner)
    }
}
