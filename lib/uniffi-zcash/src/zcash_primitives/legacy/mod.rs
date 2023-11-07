mod keys;
pub use self::keys::*;

mod transparent_address;
pub use self::transparent_address::*;

use zcash_primitives::legacy::Script;

use crate::ZcashResult;

pub struct ZcashScript(Script);

impl ZcashScript {
    pub fn from_bytes(data: &[u8]) -> ZcashResult<Self> {
        Ok(Script::read(data)?.into())
    }

    pub fn to_bytes(&self) -> ZcashResult<Vec<u8>> {
        let mut data = Vec::new();
        self.0.write(&mut data)?;
        Ok(data)
    }
}

impl From<Script> for ZcashScript {
    fn from(inner: Script) -> Self {
        ZcashScript(inner)
    }
}

impl From<&ZcashScript> for Script {
    fn from(value: &ZcashScript) -> Self {
        value.0.clone()
    }
}
