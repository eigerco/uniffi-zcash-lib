use std::sync::Arc;

use zcash_primitives::transaction::fees::zip317::FeeRule;

use crate::{ZcashAmount, ZcashResult};

#[derive(Debug, Clone)]
pub struct ZcashZip317FeeRule(FeeRule);

impl ZcashZip317FeeRule {
    /// Construct a new FeeRule using the standard [ZIP 317] constants.
    ///
    /// [ZIP 317]: https//zips.z.cash/zip-0317
    pub fn standard() -> Self {
        FeeRule::standard().into()
    }

    /// Construct a new FeeRule instance with the specified parameter values.
    ///
    /// Returns `None` if either `p2pkh_standard_input_size` or `p2pkh_standard_output_size` are
    /// zero.
    pub fn non_standard(
        marginal_fee: Arc<ZcashAmount>,
        grace_actions: u64,
        p2pkh_standard_input_size: u64,
        p2pkh_standard_output_size: u64,
    ) -> ZcashResult<Self> {
        match FeeRule::non_standard(
            (*marginal_fee).into(),
            grace_actions as usize,
            p2pkh_standard_input_size as usize,
            p2pkh_standard_output_size as usize,
        ) {
            Some(fee) => Ok(fee.into()),
            None => Err("Constructor returned no value".into()),
        }
    }

    /// Returns the ZIP 317 marginal fee.
    pub fn marginal_fee(&self) -> Arc<ZcashAmount> {
        Arc::new(self.0.marginal_fee().into())
    }
}

impl From<FeeRule> for ZcashZip317FeeRule {
    fn from(inner: FeeRule) -> Self {
        ZcashZip317FeeRule(inner)
    }
}

impl From<ZcashZip317FeeRule> for FeeRule {
    fn from(value: ZcashZip317FeeRule) -> Self {
        value.0
    }
}
