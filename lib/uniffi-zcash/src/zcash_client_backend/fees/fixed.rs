use std::sync::Arc;

use zcash_client_backend::fees::fixed::SingleOutputChangeStrategy;
use zcash_client_backend::fees::ChangeStrategy;

use crate::ZcashFixedFeeRule;

use derive_more::{From, Into};

#[derive(From, Into)]
pub struct ZcashFixedSingleOutputChangeStrategy(SingleOutputChangeStrategy);

impl ZcashFixedSingleOutputChangeStrategy {
    /// Constructs a new [`SingleOutputChangeStrategy`] with the specified ZIP 317
    /// fee parameters.
    pub fn new(fee_rule: Arc<ZcashFixedFeeRule>) -> Self {
        Self(SingleOutputChangeStrategy::new((*fee_rule).clone().into()))
    }
}

impl Clone for ZcashFixedSingleOutputChangeStrategy {
    fn clone(&self) -> Self {
        let strategy: &SingleOutputChangeStrategy = &self.0;
        let fee_rule = *strategy.fee_rule();
        Self(SingleOutputChangeStrategy::new(fee_rule))
    }
}
