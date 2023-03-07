use zcash_primitives::legacy::keys::AccountPubKey;

/// A type representing a BIP-44 public key at the account path level
/// `m/44'/<coin_type>'/<account>'`.
///
/// This provides the necessary derivation capability for the transparent component of a unified
/// full viewing key.
pub struct ZcashAccountPubKey(AccountPubKey);

impl From<AccountPubKey> for ZcashAccountPubKey {
    fn from(key: AccountPubKey) -> Self {
        ZcashAccountPubKey(key)
    }
}
