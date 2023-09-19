use zcash_primitives::transaction::components::{
    amount::{NonNegativeAmount, MAX_MONEY},
    Amount,
};

use zcash_client_backend::data_api::Balance;

use crate::{ZcashError, ZcashResult};

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

    // pub fn from_u64(value: u64) -> Self {
    //     ZcashAmount(self.0.from_u64(value))
    // }

    // pub fn from_nonnegative_i64(value: i64) -> Self {
    //     ZcashAmount(self.0.from_nonnegative_i64(value))
    // }

    /// Returns a zero-valued Amount.
    pub fn zero() -> Self {
        ZcashAmount(Amount::zero())
    }

    pub const fn is_negative(self) -> bool {
        self.0.is_negative()
    }

    /// Returns the value of the amount as i64.
    pub fn value(&self) -> i64 {
        self.0.into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct ZcashNonNegativeAmount(NonNegativeAmount);

impl ZcashNonNegativeAmount {
    /// Returns the identity `NonNegativeAmount`
    pub const ZERO: Self = Self(NonNegativeAmount::ZERO);

    pub fn from_u64(amount: u64) -> ZcashResult<Self> {
        NonNegativeAmount::from_u64(amount)
            .map(Self)
            .map_err(|_| ZcashError::Message {
                error: "maybe it is negative".to_string(),
            })
    }

    /// Creates a NonNegativeAmount from an i64.
    ///
    /// Returns an error if the amount is outside the range `{0..MAX_MONEY}`.
    pub fn from_nonnegative_i64(amount: i64) -> ZcashResult<Self> {
        NonNegativeAmount::from_nonnegative_i64(amount)
            .map(Self)
            .map_err(|_| ZcashError::Message {
                error: "maybe it is negative".to_string(),
            })
    }

    pub fn value(&self) -> u64 {
        let amount: Amount = self.0.into();
        amount.into()
    }
}

// converters

impl From<ZcashNonNegativeAmount> for NonNegativeAmount {
    fn from(inner: ZcashNonNegativeAmount) -> Self {
        inner.0
    }
}

impl From<NonNegativeAmount> for ZcashNonNegativeAmount {
    fn from(e: NonNegativeAmount) -> Self {
        ZcashNonNegativeAmount(e)
    }
}

impl TryFrom<ZcashAmount> for ZcashNonNegativeAmount {
    type Error = ();

    fn try_from(value: ZcashAmount) -> Result<Self, Self::Error> {
        if value.is_negative() {
            Err(())
        } else {
            Ok(
                NonNegativeAmount::from_u64(value.value().try_into().unwrap())
                    .unwrap()
                    .into(),
            )
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZcashBalance(Balance);

impl ZcashBalance {
    /// The [`Balance`] value having zero values for all its fields.
    pub const ZERO: Self = Self(Balance {
        spendable_value: NonNegativeAmount::ZERO,
        change_pending_confirmation: NonNegativeAmount::ZERO,
        value_pending_spendability: NonNegativeAmount::ZERO,
    });

    /// Returns the total value of funds represented by this [`Balance`].
    pub fn total(&self) -> ZcashNonNegativeAmount {
        self.0.total().into()
    }
}
