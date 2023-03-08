use zcash_primitives::transaction::components::{amount::MAX_MONEY, Amount};

use crate::ZcashError;

#[derive(Clone, Copy)]
pub struct ZcashAmount(Amount);

impl std::ops::Deref for ZcashAmount {
    type Target = Amount;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Amount> for ZcashAmount {
    fn from(value: Amount) -> Self {
        ZcashAmount(value)
    }
}

impl From<ZcashAmount> for Amount {
    fn from(value: ZcashAmount) -> Self {
        value.0
    }
}

impl ZcashAmount {
    pub fn new(value: i64) -> crate::ZcashResult<Self> {
        let amount = Amount::from_i64(value).map_err(|_| ZcashError::ValueOutOfRange {
            val: value,
            from: -MAX_MONEY,
            to: MAX_MONEY,
        })?;

        Ok(ZcashAmount(amount))
    }

    pub fn zero() -> Self {
        ZcashAmount(Amount::zero())
    }
}
