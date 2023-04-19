use orchard::circuit::{ProvingKey, VerifyingKey};

pub struct ZcashVerifyingKey(pub(crate) VerifyingKey);

impl ZcashVerifyingKey {
    pub fn new() -> Self {
        VerifyingKey::build().into()
    }
}

impl From<VerifyingKey> for ZcashVerifyingKey {
    fn from(inner: VerifyingKey) -> Self {
        ZcashVerifyingKey(inner)
    }
}

impl Default for ZcashVerifyingKey {
    fn default() -> Self {
        ZcashVerifyingKey::new()
    }
}

pub struct ZcashProvingKey(pub(crate) ProvingKey);

impl ZcashProvingKey {
    pub fn new() -> Self {
        ProvingKey::build().into()
    }
}

impl From<ProvingKey> for ZcashProvingKey {
    fn from(inner: ProvingKey) -> Self {
        ZcashProvingKey(inner)
    }
}

impl Default for ZcashProvingKey {
    fn default() -> Self {
        ZcashProvingKey::new()
    }
}
