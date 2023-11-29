use std::sync::Arc;

use zcash_client_backend::fees::zip317::SingleOutputChangeStrategy;
use zcash_client_backend::fees::ChangeStrategy;

use crate::ZcashZip317FeeRule;

use derive_more::{From, Into};

#[derive(From, Into)]
pub struct ZcashZip317SingleOutputChangeStrategy(SingleOutputChangeStrategy);

impl ZcashZip317SingleOutputChangeStrategy {
    /// Constructs a new [`SingleOutputChangeStrategy`] with the specified ZIP 317
    /// fee parameters.
    pub fn new(fee_rule: Arc<ZcashZip317FeeRule>) -> Self {
        Self(SingleOutputChangeStrategy::new((*fee_rule).clone().into()))
    }
}

impl Clone for ZcashZip317SingleOutputChangeStrategy {
    fn clone(&self) -> Self {
        let strategy: &SingleOutputChangeStrategy = &self.0;
        Self(SingleOutputChangeStrategy::new(strategy.fee_rule().clone()))
    }
}
