use zcash_primitives::transaction::components::OutPoint;

use crate::{utils::cast_slice, ZcashResult};

pub struct ZcashOutPoint(OutPoint);

impl ZcashOutPoint {
    pub fn new(hash: &[u8], n: u32) -> ZcashResult<Self> {
        let casted_data = cast_slice(hash)?;
        Ok(OutPoint::new(casted_data, n).into())
    }
}

impl From<OutPoint> for ZcashOutPoint {
    fn from(inner: OutPoint) -> Self {
        ZcashOutPoint(inner)
    }
}
