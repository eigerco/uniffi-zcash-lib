use std::fmt;
use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use zcash_client_backend::data_api::wallet::input_selection::GreedyInputSelector;
use zcash_client_backend::fees::fixed::SingleOutputChangeStrategy;
use zcash_client_sqlite::WalletDb;
use zcash_primitives::consensus::{MainNetwork, TestNetwork};

use crate::{ZcashDustOutputPolicy, ZcashFixedSingleOutputChangeStrategy};

pub type MainGreedyInputSelector =
    GreedyInputSelector<WalletDb<Connection, MainNetwork>, SingleOutputChangeStrategy>;

pub struct ZcashMainGreedyInputSelector {
    internal: Mutex<MainGreedyInputSelector>,
    change_strategy: ZcashFixedSingleOutputChangeStrategy,
    dust_output_policy: ZcashDustOutputPolicy,
}

impl ZcashMainGreedyInputSelector {
    // use trait to generalize ZcashSingleOutputChangeStrategy
    pub fn new(
        change_strategy: Arc<ZcashFixedSingleOutputChangeStrategy>,
        dust_output_policy: Arc<ZcashDustOutputPolicy>,
    ) -> Self {
        let insel: MainGreedyInputSelector = GreedyInputSelector::new(
            (*change_strategy).clone().into(),
            (*dust_output_policy).into(),
        );
        Self {
            internal: Mutex::new(insel),
            change_strategy: (*change_strategy).clone(),
            dust_output_policy: *dust_output_policy,
        }
    }
}

impl Clone for ZcashMainGreedyInputSelector {
    fn clone(&self) -> Self {
        Self::new(Arc::new(self.change_strategy.clone()), Arc::new(self.dust_output_policy))
    }
}

// NOTE change this
impl fmt::Debug for ZcashMainGreedyInputSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZcashMainGreedyInputSelector")
    }
}

impl From<ZcashMainGreedyInputSelector> for MainGreedyInputSelector {
    fn from(outer: ZcashMainGreedyInputSelector) -> Self {
        outer.internal.into_inner().unwrap()
    }
}

impl From<&dyn ZcashGreedyInputSelector> for ZcashMainGreedyInputSelector {
    fn from(z_insel: &dyn ZcashGreedyInputSelector) -> ZcashMainGreedyInputSelector {
        z_insel.into()
    }
}

pub type TestGreedyInputSelector =
    GreedyInputSelector<WalletDb<Connection, TestNetwork>, SingleOutputChangeStrategy>;

pub struct ZcashTestGreedyInputSelector {
    internal: Mutex<TestGreedyInputSelector>,
    change_strategy: ZcashFixedSingleOutputChangeStrategy,
    dust_output_policy: ZcashDustOutputPolicy,
}

impl ZcashTestGreedyInputSelector {
    // use trait to generalize ZcashSingleOutputChangeStrategy
    pub fn new(
        change_strategy: Arc<ZcashFixedSingleOutputChangeStrategy>,
        dust_output_policy: Arc<ZcashDustOutputPolicy>,
    ) -> Self {
        let insel: TestGreedyInputSelector = GreedyInputSelector::new(
            (*change_strategy).clone().into(),
            (*dust_output_policy).into(),
        );
        Self {
            internal: Mutex::new(insel),
            change_strategy: (*change_strategy).clone(),
            dust_output_policy: *dust_output_policy,
        }
    }
}


impl Clone for ZcashTestGreedyInputSelector {
    fn clone(&self) -> Self {
        Self::new(Arc::new(self.change_strategy.clone()), Arc::new(self.dust_output_policy))
    }
}

// NOTE change this
impl fmt::Debug for ZcashTestGreedyInputSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZcashTestGreedyInputSelector")
    }
}

impl From<ZcashTestGreedyInputSelector> for TestGreedyInputSelector {
    fn from(outer: ZcashTestGreedyInputSelector) -> Self {
        outer.internal.into_inner().unwrap()
    }
}

impl From<&dyn ZcashGreedyInputSelector> for ZcashTestGreedyInputSelector {
    fn from(z_insel: &dyn ZcashGreedyInputSelector) -> ZcashTestGreedyInputSelector {
        z_insel.into()
    }
}

pub trait ZcashGreedyInputSelector: Send + Sync {}

impl ZcashGreedyInputSelector for ZcashMainGreedyInputSelector {}
impl ZcashGreedyInputSelector for ZcashTestGreedyInputSelector {}
