use std::sync::Arc;

use orchard::{
    bundle::{Authorized, Flags},
    Bundle,
};
use zcash_primitives::transaction::components::Amount;

use crate::{ZcashAmount, ZcashAnchor, ZcashOrchardAction, ZcashResult};

/// A bundle of actions to be applied to the ledger.
pub struct ZcashOrchardBundle(Bundle<Authorized, Amount>);

impl ZcashOrchardBundle {
    /// The list of actions that make up this bundle.
    pub fn actions(&self) -> Vec<Arc<ZcashOrchardAction>> {
        self.0
            .actions()
            .iter()
            .map(|a| a.into())
            .map(Arc::new)
            .collect()
    }

    /// Returns the Orchard-specific transaction-level flags for this bundle.
    pub fn flags(&self) -> Arc<ZcashOrchardFlags> {
        Arc::new(self.0.flags().into())
    }

    /// Returns the net value moved into or out of the Orchard shielded pool.
    ///
    /// This is the sum of Orchard spends minus the sum Orchard outputs.
    pub fn value_balance(&self) -> Arc<ZcashAmount> {
        Arc::new(self.0.value_balance().into())
    }

    /// Returns the root of the Orchard commitment tree that this bundle commits to.
    pub fn anchor(&self) -> Arc<ZcashAnchor> {
        Arc::new(self.0.anchor().into())
    }
}

impl From<&Bundle<Authorized, Amount>> for ZcashOrchardBundle {
    fn from(inner: &Bundle<Authorized, Amount>) -> Self {
        ZcashOrchardBundle(inner.clone())
    }
}

/// Orchard-specific flags.
pub struct ZcashOrchardFlags(Flags);

impl ZcashOrchardFlags {
    /// Construct a set of flags from its constituent parts
    pub fn from_parts(spends_enabled: bool, outputs_enabled: bool) -> Self {
        Flags::from_parts(spends_enabled, outputs_enabled).into()
    }

    /// Flag denoting whether Orchard spends are enabled in the transaction.
    ///
    /// If `false`, spent notes within [`Action`]s in the transaction's [`Bundle`] are
    /// guaranteed to be dummy notes. If `true`, the spent notes may be either real or
    /// dummy notes.
    pub fn spends_enabled(&self) -> bool {
        self.0.spends_enabled()
    }

    /// Flag denoting whether Orchard outputs are enabled in the transaction.
    ///
    /// If `false`, created notes within [`Action`]s in the transaction's [`Bundle`] are
    /// guaranteed to be dummy notes. If `true`, the created notes may be either real or
    /// dummy notes.
    pub fn outputs_enabled(&self) -> bool {
        self.0.outputs_enabled()
    }

    /// Serialize flags to a byte as defined in [Zcash Protocol Spec § 7.1: Transaction
    /// Encoding And Consensus][txencoding].
    ///
    /// [txencoding]: https://zips.z.cash/protocol/protocol.pdf#txnencoding
    pub fn to_byte(&self) -> u8 {
        self.0.to_byte()
    }

    /// Parses flags from a single byte as defined in [Zcash Protocol Spec § 7.1:
    /// Transaction Encoding And Consensus][txencoding].
    ///
    /// Returns `None` if unexpected bits are set in the flag byte.
    ///
    /// [txencoding]: https://zips.z.cash/protocol/protocol.pdf#txnencoding
    pub fn from_byte(v: u8) -> ZcashResult<Self> {
        match Flags::from_byte(v) {
            Some(flags) => Ok(flags.into()),
            None => Err("Error parsing flags bits".into()),
        }
    }
}

impl From<Flags> for ZcashOrchardFlags {
    fn from(inner: Flags) -> Self {
        ZcashOrchardFlags(inner)
    }
}

impl From<&Flags> for ZcashOrchardFlags {
    fn from(inner: &Flags) -> Self {
        ZcashOrchardFlags(*inner)
    }
}
