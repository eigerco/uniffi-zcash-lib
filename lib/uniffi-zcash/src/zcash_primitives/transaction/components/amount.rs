use zcash_primitives::transaction::components::{amount::MAX_MONEY, Amount};

use crate::ZcashError;

/// A type-safe representation of some quantity of Zcash.
///
/// An Amount can only be constructed from an integer that is within the valid monetary
/// range of `{-MAX_MONEY..MAX_MONEY}` (where `MAX_MONEY` = 21,000,000 × 10⁸ zatoshis).
/// However, this range is not preserved as an invariant internally; it is possible to
/// add two valid Amounts together to obtain an invalid Amount. It is the user's
/// responsibility to handle the result of serializing potentially-invalid Amounts. In
/// particular, a [`Transaction`] containing serialized invalid Amounts will be rejected
/// by the network consensus rules.
///
/// [`Transaction`]: crate::transaction::Transaction
#[derive(Debug, Clone, Copy)]
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

impl From<&Amount> for ZcashAmount {
    fn from(value: &Amount) -> Self {
        ZcashAmount(*value)
    }
}

impl From<ZcashAmount> for Amount {
    fn from(value: ZcashAmount) -> Self {
        value.0
    }
}

impl From<&ZcashAmount> for Amount {
    fn from(value: &ZcashAmount) -> Self {
        value.0
    }
}

impl ZcashAmount {
    /// Creates an Amount from an i64.
    ///
    /// Returns an error if the amount is outside the range `{-MAX_MONEY..MAX_MONEY}`.
    pub fn new(value: i64) -> crate::ZcashResult<Self> {
        let amount = Amount::from_i64(value).map_err(|_| ZcashError::ValueOutOfRange {
            val: value,
            from: -MAX_MONEY,
            to: MAX_MONEY,
        })?;

        Ok(ZcashAmount(amount))
    }

    /// Returns a zero-valued Amount.
    pub fn zero() -> Self {
        ZcashAmount(Amount::zero())
    }

    /// Returns the value of the amount as i64.
    pub fn value(&self) -> i64 {
        self.0.into()
    }
}
