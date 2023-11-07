use std::sync::Arc;

use zcash_primitives::transaction::fees::fixed::FeeRule;
use zcash_primitives::transaction::fees::zip317::MINIMUM_FEE;

use crate::ZcashAmount;

/// A fee rule that always returns a fixed fee, irrespective of the structure of
/// the transaction being constructed.
#[derive(Clone)]
pub struct ZcashFixedFeeRule(pub(crate) FeeRule);

impl ZcashFixedFeeRule {
    /// Creates a new nonstandard fixed fee rule with the specified fixed fee.
    pub fn non_standard(fixed_fee: Arc<ZcashAmount>) -> Self {
        FeeRule::non_standard((*fixed_fee).into()).into()
    }

    /// Creates a new fixed fee rule with the standard default fee.
    pub fn standard() -> Self {
        FeeRule::non_standard(MINIMUM_FEE).into()
    }

    /// Returns the fixed fee amount which which this rule was configured.
    pub fn fixed_fee(&self) -> Arc<ZcashAmount> {
        Arc::new(self.0.fixed_fee().into())
    }
}

impl From<FeeRule> for ZcashFixedFeeRule {
    fn from(inner: FeeRule) -> Self {
        ZcashFixedFeeRule(inner)
    }
}

impl From<ZcashFixedFeeRule> for FeeRule {
    fn from(value: ZcashFixedFeeRule) -> Self {
        value.0
    }
}

impl From<&ZcashFixedFeeRule> for FeeRule {
    fn from(value: &ZcashFixedFeeRule) -> Self {
        value.0
    }
}
