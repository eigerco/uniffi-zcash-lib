use std::sync::Arc;

use zcash_client_backend::data_api::wallet::input_selection::GreedyInputSelector;

use crate::{ZcashSingleOutputChangeStrategy, ZcashDustOutputPolicy};

pub struct ZcashGreedyInputSelector(GreedyInputSelector);

impl ZcashGreedyInputSelector {
	// use trait to generalize ZcashSingleOutputChangeStrategy
	pub fn new(change_strategy: Arc<ZcashSingleOutputChangeStrategy>, dust_output_policy: Arc<ZcashDustOutputPolicy>) -> Self {
		GreedyInputSelector {
			change_strategy: (*change_strategy).into(),
			dust_output_policy: (*dust_output_policy).into()
		}
	}
}