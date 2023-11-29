mod parameters;
pub use self::parameters::*;

use derive_more::{From, Into};
use zcash_primitives::consensus::{BlockHeight, BranchId};

#[derive(Clone, Copy, From, Into)]
pub struct ZcashBlockHeight(BlockHeight);

impl ZcashBlockHeight {
    pub fn new(v: u32) -> Self {
        ZcashBlockHeight(BlockHeight::from_u32(v))
    }

    pub fn value(&self) -> u32 {
        self.0.into()
    }
}

impl From<&ZcashBlockHeight> for BlockHeight {
    fn from(value: &ZcashBlockHeight) -> Self {
        value.0
    }
}

pub enum ZcashBranchId {
    /// The consensus rules at the launch of Zcash.
    Sprout,
    /// The consensus rules deployed by [`NetworkUpgrade::Overwinter`].
    Overwinter,
    /// The consensus rules deployed by [`NetworkUpgrade::Sapling`].
    Sapling,
    /// The consensus rules deployed by [`NetworkUpgrade::Blossom`].
    Blossom,
    /// The consensus rules deployed by [`NetworkUpgrade::Heartwood`].
    Heartwood,
    /// The consensus rules deployed by [`NetworkUpgrade::Canopy`].
    Canopy,
    /// The consensus rules deployed by [`NetworkUpgrade::Nu5`].
    Nu5,
}

impl From<ZcashBranchId> for BranchId {
    fn from(value: ZcashBranchId) -> Self {
        match value {
            ZcashBranchId::Sprout => BranchId::Sprout,
            ZcashBranchId::Overwinter => BranchId::Overwinter,
            ZcashBranchId::Sapling => BranchId::Sapling,
            ZcashBranchId::Blossom => BranchId::Blossom,
            ZcashBranchId::Heartwood => BranchId::Heartwood,
            ZcashBranchId::Canopy => BranchId::Canopy,
            ZcashBranchId::Nu5 => BranchId::Nu5,
        }
    }
}

impl From<BranchId> for ZcashBranchId {
    fn from(value: BranchId) -> Self {
        match value {
            BranchId::Sprout => ZcashBranchId::Sprout,
            BranchId::Overwinter => ZcashBranchId::Overwinter,
            BranchId::Sapling => ZcashBranchId::Sapling,
            BranchId::Blossom => ZcashBranchId::Blossom,
            BranchId::Heartwood => ZcashBranchId::Heartwood,
            BranchId::Canopy => ZcashBranchId::Canopy,
            BranchId::Nu5 => ZcashBranchId::Nu5,
        }
    }
}
