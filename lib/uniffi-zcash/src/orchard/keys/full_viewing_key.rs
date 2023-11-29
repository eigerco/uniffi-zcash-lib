use std::sync::Arc;

use orchard::keys::{DiversifierIndex, FullViewingKey};

use crate::{
    utils, ZcashError, ZcashOrchardAddress, ZcashOrchardDiversifier, ZcashOrchardDiversifierIndex,
    ZcashOrchardIncomingViewingKey, ZcashOrchardOutgoingViewingKey, ZcashOrchardScope,
    ZcashOrchardSpendingKey, ZcashResult,
};

use derive_more::{From, Into};

/// A key that provides the capability to view incoming and outgoing transactions.
///
/// This key is useful anywhere you need to maintain accurate balance, but do not want the
/// ability to spend funds (such as a view-only wallet).
#[derive(From, Into)]
pub struct ZcashOrchardFullViewingKey(FullViewingKey);

impl ZcashOrchardFullViewingKey {
    /// Returns the payment address for this key at the given index.
    pub fn address_at(
        &self,
        j: Arc<ZcashOrchardDiversifierIndex>,
        scope: ZcashOrchardScope,
    ) -> Arc<ZcashOrchardAddress> {
        let j: DiversifierIndex = j.as_ref().into();
        Arc::new(self.0.address_at(j, scope.into()).into())
    }

    /// Returns the payment address for this key corresponding to the given diversifier.
    pub fn address(
        &self,
        d: Arc<ZcashOrchardDiversifier>,
        scope: ZcashOrchardScope,
    ) -> Arc<ZcashOrchardAddress> {
        Arc::new(self.0.address(d.as_ref().into(), scope.into()).into())
    }

    /// Returns the scope of the given address, or `None` if the address is not derived
    /// from this full viewing key.
    pub fn scope_for_address(
        &self,
        address: Arc<ZcashOrchardAddress>,
    ) -> Option<ZcashOrchardScope> {
        self.0
            .scope_for_address(&(*address.as_ref()).clone().into())
            .map(From::from)
    }

    /// Serializes the full viewing key as specified in [Zcash Protocol Spec § 5.6.4.4: Orchard Raw Full Viewing Keys][orchardrawfullviewingkeys]
    ///
    /// [orchardrawfullviewingkeys]: https://zips.z.cash/protocol/protocol.pdf#orchardfullviewingkeyencoding
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    /// Parses a full viewing key from its "raw" encoding as specified in [Zcash Protocol Spec § 5.6.4.4: Orchard Raw Full Viewing Keys][orchardrawfullviewingkeys]
    ///
    /// [orchardrawfullviewingkeys]: https://zips.z.cash/protocol/protocol.pdf#orchardfullviewingkeyencoding
    pub fn from_bytes(bytes: Vec<u8>) -> ZcashResult<Self> {
        let array = utils::cast_slice(&bytes)?;
        let fvk = FullViewingKey::from_bytes(&array).ok_or(ZcashError::Unknown)?;
        Ok(ZcashOrchardFullViewingKey(fvk))
    }

    /// Derives an `IncomingViewingKey` for this full viewing key.
    pub fn to_ivk(&self, scope: ZcashOrchardScope) -> Arc<ZcashOrchardIncomingViewingKey> {
        Arc::new(self.0.to_ivk(scope.into()).into())
    }

    /// Derives an `OutgoingViewingKey` for this full viewing key.
    pub fn to_ovk(&self, scope: ZcashOrchardScope) -> Arc<ZcashOrchardOutgoingViewingKey> {
        Arc::new(self.0.to_ovk(scope.into()).into())
    }
}

impl From<&ZcashOrchardSpendingKey> for ZcashOrchardFullViewingKey {
    fn from(value: &ZcashOrchardSpendingKey) -> Self {
        let inner_fvk: FullViewingKey = (&value.0).into();
        ZcashOrchardFullViewingKey(inner_fvk)
    }
}

impl From<&ZcashOrchardFullViewingKey> for FullViewingKey {
    fn from(key: &ZcashOrchardFullViewingKey) -> Self {
        key.0.clone()
    }
}
