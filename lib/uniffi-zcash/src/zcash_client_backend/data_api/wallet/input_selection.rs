use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use zcash_client_backend::data_api::wallet::input_selection::GreedyInputSelector;
use zcash_client_backend::fees::fixed::SingleOutputChangeStrategy as FixedSingleOutputChangeStrategy;
use zcash_client_backend::fees::zip317::SingleOutputChangeStrategy as Zip317SingleOutputChangeStrategy;
use zcash_client_sqlite::WalletDb;
use zcash_primitives::consensus::{MainNetwork, TestNetwork};

use crate::{
    ZcashDustOutputPolicy, ZcashFixedSingleOutputChangeStrategy,
    ZcashZip317SingleOutputChangeStrategy,
};

// Fixed

pub type MainFixedGreedyInputSelector =
    GreedyInputSelector<WalletDb<Connection, MainNetwork>, FixedSingleOutputChangeStrategy>;

pub struct ZcashMainFixedGreedyInputSelector {
    internal: Mutex<MainFixedGreedyInputSelector>,
    change_strategy: ZcashFixedSingleOutputChangeStrategy,
    dust_output_policy: ZcashDustOutputPolicy,
}

impl ZcashMainFixedGreedyInputSelector {
    pub fn new(
        change_strategy: Arc<ZcashFixedSingleOutputChangeStrategy>,
        dust_output_policy: Arc<ZcashDustOutputPolicy>,
    ) -> Self {
        let insel: MainFixedGreedyInputSelector = GreedyInputSelector::new(
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

impl Clone for ZcashMainFixedGreedyInputSelector {
    fn clone(&self) -> Self {
        Self::new(
            Arc::new(self.change_strategy.clone()),
            Arc::new(self.dust_output_policy),
        )
    }
}

impl From<ZcashMainFixedGreedyInputSelector> for MainFixedGreedyInputSelector {
    fn from(outer: ZcashMainFixedGreedyInputSelector) -> Self {
        outer.internal.into_inner().unwrap()
    }
}

pub type TestFixedGreedyInputSelector =
    GreedyInputSelector<WalletDb<Connection, TestNetwork>, FixedSingleOutputChangeStrategy>;

pub struct ZcashTestFixedGreedyInputSelector {
    internal: Mutex<TestFixedGreedyInputSelector>,
    change_strategy: ZcashFixedSingleOutputChangeStrategy,
    dust_output_policy: ZcashDustOutputPolicy,
}

impl ZcashTestFixedGreedyInputSelector {
    pub fn new(
        change_strategy: Arc<ZcashFixedSingleOutputChangeStrategy>,
        dust_output_policy: Arc<ZcashDustOutputPolicy>,
    ) -> Self {
        let insel: TestFixedGreedyInputSelector = GreedyInputSelector::new(
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

impl Clone for ZcashTestFixedGreedyInputSelector {
    fn clone(&self) -> Self {
        Self::new(
            Arc::new(self.change_strategy.clone()),
            Arc::new(self.dust_output_policy),
        )
    }
}

impl From<ZcashTestFixedGreedyInputSelector> for TestFixedGreedyInputSelector {
    fn from(outer: ZcashTestFixedGreedyInputSelector) -> Self {
        outer.internal.into_inner().unwrap()
    }
}

// ZIP317

pub type MainZip317GreedyInputSelector =
    GreedyInputSelector<WalletDb<Connection, MainNetwork>, Zip317SingleOutputChangeStrategy>;

pub struct ZcashMainZip317GreedyInputSelector {
    internal: Mutex<MainZip317GreedyInputSelector>,
    change_strategy: ZcashZip317SingleOutputChangeStrategy,
    dust_output_policy: ZcashDustOutputPolicy,
}

impl ZcashMainZip317GreedyInputSelector {
    pub fn new(
        change_strategy: Arc<ZcashZip317SingleOutputChangeStrategy>,
        dust_output_policy: Arc<ZcashDustOutputPolicy>,
    ) -> Self {
        let insel: MainZip317GreedyInputSelector = GreedyInputSelector::new(
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

impl Clone for ZcashMainZip317GreedyInputSelector {
    fn clone(&self) -> Self {
        Self::new(
            Arc::new(self.change_strategy.clone()),
            Arc::new(self.dust_output_policy),
        )
    }
}

impl From<ZcashMainZip317GreedyInputSelector> for MainZip317GreedyInputSelector {
    fn from(outer: ZcashMainZip317GreedyInputSelector) -> Self {
        outer.internal.into_inner().unwrap()
    }
}

pub type TestZip317GreedyInputSelector =
    GreedyInputSelector<WalletDb<Connection, TestNetwork>, Zip317SingleOutputChangeStrategy>;

pub struct ZcashTestZip317GreedyInputSelector {
    internal: Mutex<TestZip317GreedyInputSelector>,
    change_strategy: ZcashZip317SingleOutputChangeStrategy,
    dust_output_policy: ZcashDustOutputPolicy,
}

impl ZcashTestZip317GreedyInputSelector {
    pub fn new(
        change_strategy: Arc<ZcashZip317SingleOutputChangeStrategy>,
        dust_output_policy: Arc<ZcashDustOutputPolicy>,
    ) -> Self {
        let insel: TestZip317GreedyInputSelector = GreedyInputSelector::new(
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

impl Clone for ZcashTestZip317GreedyInputSelector {
    fn clone(&self) -> Self {
        Self::new(
            Arc::new(self.change_strategy.clone()),
            Arc::new(self.dust_output_policy),
        )
    }
}

impl From<ZcashTestZip317GreedyInputSelector> for TestZip317GreedyInputSelector {
    fn from(outer: ZcashTestZip317GreedyInputSelector) -> Self {
        outer.internal.into_inner().unwrap()
    }
}
