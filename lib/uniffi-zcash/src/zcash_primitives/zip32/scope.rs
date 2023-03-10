use zcash_primitives::zip32::Scope;

/// The scope of a viewing key or address.
///
/// A "scope" narrows the visibility or usage to a level below "full".
///
/// Consistent usage of `Scope` enables the user to provide consistent views over a wallet
/// to other people. For example, a user can give an external [SaplingIvk] to a merchant
/// terminal, enabling it to only detect "real" transactions from customers and not
/// internal transactions from the wallet.
///
/// [SaplingIvk]: ZcashSaplingIvk
pub enum ZcashScope {
    /// A scope used for wallet-external operations, namely deriving addresses to give to
    /// other users in order to receive funds.
    External,
    /// A scope used for wallet-internal operations, such as creating change notes,
    /// auto-shielding, and note management.
    Internal,
}

impl From<Scope> for ZcashScope {
    fn from(scope: Scope) -> Self {
        match scope {
            Scope::External => ZcashScope::External,
            Scope::Internal => ZcashScope::Internal,
        }
    }
}

impl From<ZcashScope> for Scope {
    fn from(value: ZcashScope) -> Self {
        match value {
            ZcashScope::External => Scope::External,
            ZcashScope::Internal => Scope::Internal,
        }
    }
}
