use zcash_primitives::transaction::components::sapling::builder::SaplingMetadata;

/// Metadata about a transaction created by a [`SaplingBuilder`].
pub struct ZcashSaplingMetadata(SaplingMetadata);

impl ZcashSaplingMetadata {
    pub fn new() -> Self {
        SaplingMetadata::empty().into()
    }

    /// Returns the index within the transaction of the [`SpendDescription`] corresponding
    /// to the `n`-th call to [`SaplingBuilder::add_spend`].
    ///
    /// Note positions are randomized when building transactions for indistinguishability.
    /// This means that the transaction consumer cannot assume that e.g. the first spend
    /// they added (via the first call to [`SaplingBuilder::add_spend`]) is the first
    pub fn spend_index(&self, n: u64) -> Option<u64> {
        match self.0.spend_index(n as usize) {
            Some(r) => Some(r as u64),
            None => None,
        }
    }

    /// Returns the index within the transaction of the [`OutputDescription`] corresponding
    /// to the `n`-th call to [`SaplingBuilder::add_output`].
    ///
    /// Note positions are randomized when building transactions for indistinguishability.
    /// This means that the transaction consumer cannot assume that e.g. the first output
    /// they added (via the first call to [`SaplingBuilder::add_output`]) is the first
    /// [`OutputDescription`] in the transaction.
    pub fn output_index(&self, n: u64) -> Option<u64> {
        match self.0.output_index(n as usize) {
            Some(r) => Some(r as u64),
            None => None,
        }
    }
}

impl From<SaplingMetadata> for ZcashSaplingMetadata {
    fn from(inner: SaplingMetadata) -> Self {
        ZcashSaplingMetadata(inner)
    }
}