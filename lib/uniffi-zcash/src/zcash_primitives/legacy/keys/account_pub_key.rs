use std::sync::Arc;

use zcash_primitives::legacy::keys::{AccountPubKey, ExternalOvk, InternalOvk};

use crate::{
    utils::cast_slice, ZcashError, ZcashExternalIvk, ZcashExternalOvk, ZcashInternalIvk,
    ZcashInternalOvk, ZcashResult,
};

/// A type representing a BIP-44 public key at the account path level
/// `m/44'/<coin_type>'/<account>'`.
///
/// This provides the necessary derivation capability for the transparent component of a unified
/// full viewing key.
pub struct ZcashAccountPubKey(AccountPubKey);

impl ZcashAccountPubKey {
    pub fn new(data: Vec<u8>) -> ZcashResult<Self> {
        let casted_data = cast_slice(data.as_slice())?;
        let key = AccountPubKey::deserialize(&casted_data).map_err(ZcashError::from)?;
        Ok(key.into())
    }

    /// Derives the BIP-44 public key at the external "change level" path
    /// `m/44'/<coin_type>'/<account>'/0`.
    pub fn derive_external_ivk(&self) -> ZcashResult<Arc<ZcashExternalIvk>> {
        let ivk = self.0.derive_external_ivk().map_err(ZcashError::from)?;
        Ok(Arc::new(ivk.into()))
    }

    /// Derives the BIP-44 public key at the internal "change level" path
    /// `m/44'/<coin_type>'/<account>'/1`.
    pub fn derive_internal_ivk(&self) -> ZcashResult<Arc<ZcashInternalIvk>> {
        let ivk = self.0.derive_internal_ivk().map_err(ZcashError::from)?;
        Ok(Arc::new(ivk.into()))
    }

    /// Derives the internal ovk and external ovk corresponding to this
    /// transparent fvk. As specified in [ZIP 316][transparent-ovk].
    ///
    /// [transparent-ovk]: https://zips.z.cash/zip-0316#deriving-internal-keys
    pub fn ovks_for_shielding(&self) -> ZcashInternalOvkExternalOvk {
        self.0.ovks_for_shielding().into()
    }

    /// Derives the internal ovk corresponding to this transparent fvk.
    pub fn internal_ovk(&self) -> Arc<ZcashInternalOvk> {
        Arc::new(self.0.ovks_for_shielding().0.into())
    }

    /// Derives the external ovk corresponding to this transparent fvk.
    pub fn external_ovk(&self) -> Arc<ZcashExternalOvk> {
        Arc::new(self.0.ovks_for_shielding().1.into())
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.0.serialize()
    }
}

impl From<AccountPubKey> for ZcashAccountPubKey {
    fn from(inner: AccountPubKey) -> Self {
        ZcashAccountPubKey(inner)
    }
}

impl From<&ZcashAccountPubKey> for AccountPubKey {
    fn from(key: &ZcashAccountPubKey) -> Self {
        key.0.clone()
    }
}

pub struct ZcashInternalOvkExternalOvk {
    pub internal_ovk: Arc<ZcashInternalOvk>,
    pub external_ovk: Arc<ZcashExternalOvk>,
}

impl From<(InternalOvk, ExternalOvk)> for ZcashInternalOvkExternalOvk {
    fn from((internal_ovk, external_ovk): (InternalOvk, ExternalOvk)) -> Self {
        ZcashInternalOvkExternalOvk {
            internal_ovk: Arc::new(internal_ovk.into()),
            external_ovk: Arc::new(external_ovk.into()),
        }
    }
}
