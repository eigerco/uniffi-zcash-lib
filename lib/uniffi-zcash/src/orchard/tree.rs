use crate::{utils::cast_slice, ZcashResult};

use orchard::Anchor;

pub struct ZcashAnchor(Anchor);

impl ZcashAnchor {
    pub fn from_bytes(bytes: &[u8]) -> ZcashResult<Self> {
        let opt: Option<Anchor> = Anchor::from_bytes(cast_slice(bytes)?).into();
        match opt {
            Some(anchor) => Ok(anchor.into()),
            None => Err("Error parsing bytes".into()),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

impl From<Anchor> for ZcashAnchor {
    fn from(inner: Anchor) -> Self {
        ZcashAnchor(inner)
    }
}

impl From<&ZcashAnchor> for Anchor {
    fn from(value: &ZcashAnchor) -> Self {
        value.0
    }
}
