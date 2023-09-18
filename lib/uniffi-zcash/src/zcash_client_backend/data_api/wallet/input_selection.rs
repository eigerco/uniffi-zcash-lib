use std::sync::{Arc, Mutex};

use zcash_client_backend::data_api::wallet::input_selection::GreedyInputSelector;
use zcash_client_backend::fees::fixed::SingleOutputChangeStrategy;
use zcash_client_sqlite::WalletDb;
use rusqlite::Connection;
use zcash_primitives::consensus::{MainNetwork, TestNetwork};

use crate::fixed::ZcashFixedSingleOutputChangeStrategy;

use crate::{ZcashDustOutputPolicy};

pub type MainGreedyInputSelector = GreedyInputSelector<WalletDb<Connection, MainNetwork>, SingleOutputChangeStrategy>;

pub struct ZcashMainGreedyInputSelector(Mutex<MainGreedyInputSelector>);

impl ZcashMainGreedyInputSelector {
	// use trait to generalize ZcashSingleOutputChangeStrategy
	pub fn new(change_strategy: Arc<ZcashFixedSingleOutputChangeStrategy>, dust_output_policy: Arc<ZcashDustOutputPolicy>) -> Self {
		Self(Mutex::new(GreedyInputSelector::new((*change_strategy).clone().into(), (*dust_output_policy).into())))
	}
}

impl From<ZcashMainGreedyInputSelector> for MainGreedyInputSelector {
    fn from(outer: ZcashMainGreedyInputSelector) -> Self {
        outer.0.into_inner().unwrap()
    }
}

impl From<&dyn ZcashGreedyInputSelector> for ZcashMainGreedyInputSelector {
	fn from(z_insel: &dyn ZcashGreedyInputSelector) -> ZcashMainGreedyInputSelector {
		z_insel.into()
	}
}


pub type TestGreedyInputSelector = GreedyInputSelector<WalletDb<Connection, TestNetwork>, SingleOutputChangeStrategy>;

pub struct ZcashTestGreedyInputSelector(Mutex<TestGreedyInputSelector>);

impl ZcashTestGreedyInputSelector {
	// use trait to generalize ZcashSingleOutputChangeStrategy
	pub fn new(change_strategy: Arc<ZcashFixedSingleOutputChangeStrategy>, dust_output_policy: Arc<ZcashDustOutputPolicy>) -> Self {
		Self(Mutex::new(GreedyInputSelector::new((*change_strategy).clone().into(), (*dust_output_policy).into())))
	}
}

impl From<ZcashTestGreedyInputSelector> for TestGreedyInputSelector {
    fn from(outer: ZcashTestGreedyInputSelector) -> Self {
        outer.0.into_inner().unwrap()
    }
}

impl From<&dyn ZcashGreedyInputSelector> for ZcashTestGreedyInputSelector {
	fn from(z_insel: &dyn ZcashGreedyInputSelector) -> ZcashTestGreedyInputSelector {
		z_insel.into()
	}
}


// NOTE Send + Sync because UniFFI requires it for Traits
pub trait ZcashGreedyInputSelector: Send + Sync {}

impl ZcashGreedyInputSelector for ZcashMainGreedyInputSelector {}
impl ZcashGreedyInputSelector for ZcashTestGreedyInputSelector {}
