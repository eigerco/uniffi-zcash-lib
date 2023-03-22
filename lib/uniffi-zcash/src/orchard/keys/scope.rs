use orchard::keys::Scope;

/// The scope of a viewing key or address.
pub enum ZcashOrchardScope {
    /// A scope used for wallet-external operations, namely deriving addresses to give to
    /// other users in order to receive funds.
    External,
    /// A scope used for wallet-internal operations, such as creating change notes,
    /// auto-shielding, and note management.
    Internal,
}

impl From<ZcashOrchardScope> for Scope {
    fn from(value: ZcashOrchardScope) -> Self {
        match value {
            ZcashOrchardScope::External => Scope::External,
            ZcashOrchardScope::Internal => Scope::Internal,
        }
    }
}

impl From<Scope> for ZcashOrchardScope {
    fn from(value: Scope) -> Self {
        match value {
            Scope::External => ZcashOrchardScope::External,
            Scope::Internal => ZcashOrchardScope::Internal,
        }
    }
}
