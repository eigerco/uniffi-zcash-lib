use group::GroupEncoding;
use jubjub::SubgroupPoint;
use zcash_primitives::sapling::NullifierDerivingKey;

use crate::{utils, ZcashResult};

pub struct ZcashNullifierDerivingKey(NullifierDerivingKey);

impl From<NullifierDerivingKey> for ZcashNullifierDerivingKey {
    fn from(key: NullifierDerivingKey) -> Self {
        ZcashNullifierDerivingKey(key)
    }
}

impl ZcashNullifierDerivingKey {
    pub fn from_bytes(bytes: &[u8]) -> ZcashResult<Self> {
        let array = utils::cast_slice(bytes)?;
        let group: Option<_> = SubgroupPoint::from_bytes(&array).into();
        let group = group.ok_or("unable to parse nullifier deriving key")?;
        Ok(ZcashNullifierDerivingKey(NullifierDerivingKey(group)))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0 .0.to_bytes().to_vec()
    }
}
