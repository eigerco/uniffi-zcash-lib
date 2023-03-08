use zcash_primitives::zip32::AccountId;

/// A type-safe wrapper for account identifiers.
#[derive(Copy, Clone)]
pub struct ZcashAccountId {
    pub id: u32,
}

impl From<ZcashAccountId> for AccountId {
    fn from(value: ZcashAccountId) -> Self {
        value.id.into()
    }
}
