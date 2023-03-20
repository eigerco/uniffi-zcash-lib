use orchard::keys::Diversifier;

use crate::{utils, ZcashResult};

/// A diversifier that can be used to derive a specific [`Address`] from a
/// [`FullViewingKey`] or [`IncomingViewingKey`].
pub struct ZcashOrchardDiversifier(pub Diversifier);

impl ZcashOrchardDiversifier {
    pub fn from_bytes(data: Vec<u8>) -> ZcashResult<Self> {
        let array = utils::cast_slice(&data)?;
        Ok(Diversifier::from_bytes(array).into())
    }
}

impl From<&ZcashOrchardDiversifier> for Diversifier {
    fn from(diversifier: &ZcashOrchardDiversifier) -> Self {
        diversifier.0
    }
}

impl From<Diversifier> for ZcashOrchardDiversifier {
    fn from(diversifier: Diversifier) -> Self {
        Self(diversifier)
    }
}
