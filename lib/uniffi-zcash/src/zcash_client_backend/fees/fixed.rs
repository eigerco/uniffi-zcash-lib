use std::sync::Arc;

use zcash_client_backend::fees::fixed::SingleOutputChangeStrategy;

use crate::ZcashFixedFeeRule;

pub struct ZcashFixedSingleOutputChangeStrategy(SingleOutputChangeStrategy);

impl ZcashFixedSingleOutputChangeStrategy {
    /// Constructs a new [`SingleOutputChangeStrategy`] with the specified ZIP 317
    /// fee parameters.
    pub fn new(fee_rule: Arc<ZcashFixedFeeRule>) -> Self {
        Self(SingleOutputChangeStrategy::new((*fee_rule).clone().into()))
    }
}