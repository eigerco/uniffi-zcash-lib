use std::sync::Arc;

use zcash_client_backend::fees::fixed::SingleOutputChangeStrategy;
use zcash_client_backend::fees::ChangeStrategy;

use crate::ZcashFixedFeeRule;

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZcashFixedSingleOutputChangeStrategy(SingleOutputChangeStrategy);

impl ZcashFixedSingleOutputChangeStrategy {
    /// Constructs a new [`SingleOutputChangeStrategy`] with the specified ZIP 317
    /// fee parameters.
    pub fn new(fee_rule: Arc<ZcashFixedFeeRule>) -> Self {
        Self(SingleOutputChangeStrategy::new((*fee_rule).clone().into()))
    }
}

impl From<SingleOutputChangeStrategy> for ZcashFixedSingleOutputChangeStrategy {
    fn from(inner: SingleOutputChangeStrategy) -> Self {
        ZcashFixedSingleOutputChangeStrategy(inner)
    }
}

impl From<ZcashFixedSingleOutputChangeStrategy> for SingleOutputChangeStrategy {
    fn from(outer: ZcashFixedSingleOutputChangeStrategy) -> Self {
        outer.0
    }
}


// NOTE apparently implementing Copy would be more difficult, so I did this instead
impl Clone for ZcashFixedSingleOutputChangeStrategy {
    fn clone(&self) -> Self {
        let strategy: &SingleOutputChangeStrategy = &(*self).0;
        let fee_rule = strategy.fee_rule().clone();
        Self(SingleOutputChangeStrategy::new(fee_rule))
    }
}