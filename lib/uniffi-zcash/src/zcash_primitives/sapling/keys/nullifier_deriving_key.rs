use zcash_primitives::sapling::NullifierDerivingKey;

pub struct ZcashNullifierDerivingKey(NullifierDerivingKey);

impl From<NullifierDerivingKey> for ZcashNullifierDerivingKey {
    fn from(key: NullifierDerivingKey) -> Self {
        ZcashNullifierDerivingKey(key)
    }
}

// todo
