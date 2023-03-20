use std::sync::Arc;

use zcash_primitives::legacy::keys::AccountPrivKey;

use crate::{
    SecpSecretKey, ZcashAccountId, ZcashAccountPubKey, ZcashConsensusParameters, ZcashError,
    ZcashExtendedPrivKey, ZcashResult,
};

/// A type representing a BIP-44 private key at the account path level
/// `m/44'/<coin_type>'/<account>'
pub struct ZcashAccountPrivKey(AccountPrivKey);

impl From<AccountPrivKey> for ZcashAccountPrivKey {
    fn from(key: AccountPrivKey) -> Self {
        ZcashAccountPrivKey(key)
    }
}

impl ZcashAccountPrivKey {
    /// Performs derivation of the extended private key for the BIP-44 path:
    /// `m/44'/<coin_type>'/<account>'`.
    ///
    /// This produces the root of the derivation tree for transparent
    /// viewing keys and addresses for the for the provided account.
    pub fn from_seed(
        params: ZcashConsensusParameters,
        seed: Vec<u8>,
        account: ZcashAccountId,
    ) -> ZcashResult<Self> {
        let key = AccountPrivKey::from_seed(&params, &seed, account.into())?;

        Ok(key.into())
    }

    pub fn from_extended_privkey(ext_privkey: Arc<ZcashExtendedPrivKey>) -> Self {
        let key = AccountPrivKey::from_extended_privkey(ext_privkey.0.clone());
        key.into()
    }

    pub fn to_account_pubkey(&self) -> Arc<ZcashAccountPubKey> {
        Arc::new(self.0.to_account_pubkey().into())
    }

    /// Derives the BIP-44 private spending key for the external (incoming payment) child path
    /// `m/44'/<coin_type>'/<account>'/0/<child_index>`.
    pub fn derive_external_secret_key(&self, child_index: u32) -> ZcashResult<Arc<SecpSecretKey>> {
        let secret = self
            .0
            .derive_external_secret_key(child_index)
            .map_err(ZcashError::from)?;
        Ok(Arc::new(secret.into()))
    }

    /// Derives the BIP-44 private spending key for the internal (change) child path
    /// `m/44'/<coin_type>'/<account>'/1/<child_index>`.
    pub fn derive_internal_secret_key(&self, child_index: u32) -> ZcashResult<Arc<SecpSecretKey>> {
        let secret = self
            .0
            .derive_internal_secret_key(child_index)
            .map_err(ZcashError::from)?;
        Ok(Arc::new(secret.into()))
    }

    /// Returns the `AccountPrivKey` serialized using the encoding for a
    /// [BIP 32](https://en.bitcoin.it/wiki/BIP_0032) ExtendedPrivKey
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }

    /// Decodes the `AccountPrivKey` from the encoding specified for a
    /// [BIP 32](https://en.bitcoin.it/wiki/BIP_0032) ExtendedPrivKey
    pub fn from_bytes(bytes: Vec<u8>) -> ZcashResult<Self> {
        zcash_primitives::legacy::keys::AccountPrivKey::from_bytes(&bytes)
            .map(ZcashAccountPrivKey::from)
            .ok_or(ZcashError::Unknown)
    }
}
