use std::sync::Arc;

use zcash_client_backend::fees::zip317::SingleOutputChangeStrategy;

use crate::ZcashZip317FeeRule;

pub struct ZcashZip317SingleOutputChangeStrategy(SingleOutputChangeStrategy);

impl ZcashZip317SingleOutputChangeStrategy {
    /// Constructs a new [`SingleOutputChangeStrategy`] with the specified ZIP 317
    /// fee parameters.
    pub fn new(fee_rule: Arc<ZcashZip317FeeRule>) -> Self {
        Self(SingleOutputChangeStrategy::new((*fee_rule).clone().into()))
    }
}
