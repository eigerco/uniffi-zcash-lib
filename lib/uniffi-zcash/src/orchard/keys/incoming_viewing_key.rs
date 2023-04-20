use std::sync::Arc;

use orchard::keys::{DiversifierIndex, IncomingViewingKey};

use crate::{
    utils, ZcashError, ZcashOrchardAddress, ZcashOrchardDiversifier, ZcashOrchardDiversifierIndex,
    ZcashResult,
};

/// A key that provides the capability to detect and decrypt incoming notes from the block
/// chain, without being able to spend the notes or detect when they are spent.
///
/// This key is useful in situations where you only need the capability to detect inbound
/// payments, such as merchant terminals.
pub struct ZcashOrchardIncomingViewingKey(pub(crate) IncomingViewingKey);

impl ZcashOrchardIncomingViewingKey {
    /// Serializes an Orchard incoming viewing key to its raw encoding as specified in [Zcash Protocol Spec § 5.6.4.3: Orchard Raw Incoming Viewing Keys][orchardrawinviewingkeys]
    ///
    /// [orchardrawinviewingkeys]: https://zips.z.cash/protocol/protocol.pdf#orchardinviewingkeyencoding
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    /// Parses an Orchard incoming viewing key from its raw encoding.
    pub fn from_bytes(bytes: Vec<u8>) -> ZcashResult<Self> {
        let array = utils::cast_slice(&bytes)?;
        let key: Option<_> = IncomingViewingKey::from_bytes(&array).into();
        Ok(ZcashOrchardIncomingViewingKey(
            key.ok_or(ZcashError::Unknown)?,
        ))
    }

    /// Checks whether the given address was derived from this incoming viewing
    /// key, and returns the diversifier index used to derive the address if
    /// so. Returns `None` if the address was not derived from this key.
    pub fn diversifier_index(
        &self,
        addr: Arc<ZcashOrchardAddress>,
    ) -> Option<Arc<ZcashOrchardDiversifierIndex>> {
        self.0
            .diversifier_index(&addr.as_ref().into())
            .map(From::from)
            .map(Arc::new)
    }

    /// Returns the payment address for this key at the given index.
    pub fn address_at(&self, j: Arc<ZcashOrchardDiversifierIndex>) -> Arc<ZcashOrchardAddress> {
        let j: DiversifierIndex = j.as_ref().into();
        Arc::new(self.0.address_at(j).into())
    }

    /// Returns the payment address for this key corresponding to the given diversifier.
    pub fn address(&self, diversifier: Arc<ZcashOrchardDiversifier>) -> Arc<ZcashOrchardAddress> {
        Arc::new(self.0.address(diversifier.0).into())
    }
}

impl From<IncomingViewingKey> for ZcashOrchardIncomingViewingKey {
    fn from(key: IncomingViewingKey) -> Self {
        Self(key)
    }
}


impl From<&ZcashOrchardIncomingViewingKey> for IncomingViewingKey {
    fn from(value: &ZcashOrchardIncomingViewingKey) -> Self {
        value.0.clone()
    }
}