use crate::ZcashAmount;
use std::sync::Arc;
use zcash_client_backend::fees::{DustAction, DustOutputPolicy};

pub mod fixed;
pub use self::fixed::*;

pub mod zip317;
pub use self::zip317::*;

pub enum ZcashDustAction {
    /// Do not allow creation of dust outputs; instead, require that additional inputs be provided.
    Reject,
    /// Explicitly allow the creation of dust change amounts greater than the specified value.
    AllowDustChange,
    /// Allow dust amounts to be added to the transaction fee
    AddDustToFee,
}

impl From<ZcashDustAction> for DustAction {
    fn from(value: ZcashDustAction) -> Self {
        match value {
            ZcashDustAction::Reject => DustAction::Reject,
            ZcashDustAction::AllowDustChange => DustAction::AllowDustChange,
            ZcashDustAction::AddDustToFee => DustAction::AddDustToFee,
        }
    }
}

impl From<DustAction> for ZcashDustAction {
    fn from(value: DustAction) -> Self {
        match value {
            DustAction::Reject => ZcashDustAction::Reject,
            DustAction::AllowDustChange => ZcashDustAction::AllowDustChange,
            DustAction::AddDustToFee => ZcashDustAction::AddDustToFee,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZcashDustOutputPolicy(DustOutputPolicy);

impl ZcashDustOutputPolicy {
    pub fn new(action: ZcashDustAction, dust_threshold: Option<Arc<ZcashAmount>>) -> Self {
        ZcashDustOutputPolicy(DustOutputPolicy::new(
            action.into(),
            dust_threshold.as_deref().map(From::from),
        ))
    }

    pub fn action(&self) -> ZcashDustAction {
        self.0.action().into()
    }

    pub fn dust_threshold(&self) -> Option<Arc<ZcashAmount>> {
        self.0.dust_threshold().map(From::from).map(Arc::new)
    }
}

impl From<ZcashDustOutputPolicy> for DustOutputPolicy {
    fn from(inner: ZcashDustOutputPolicy) -> Self {
        inner.0
    }
}

impl Default for ZcashDustOutputPolicy {
    fn default() -> Self {
        ZcashDustOutputPolicy::new(ZcashDustAction::Reject, None)
    }
}
