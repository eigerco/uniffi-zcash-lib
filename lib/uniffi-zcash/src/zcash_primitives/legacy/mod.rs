mod keys;
pub use self::keys::*;

mod transparent_address;
pub use self::transparent_address::*;

use zcash_primitives::legacy::Script;

pub struct ZcashScript(Script);

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
