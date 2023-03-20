use zcash_primitives::legacy::keys::{ExternalOvk, InternalOvk};

/// ExternalOVK
pub struct ZcashExternalOvk(ExternalOvk);

impl ZcashExternalOvk {
    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}

impl From<ExternalOvk> for ZcashExternalOvk {
    fn from(inner: ExternalOvk) -> Self {
        ZcashExternalOvk(inner)
    }
}

/// InternalOVK
pub struct ZcashInternalOvk(InternalOvk);

impl ZcashInternalOvk {
    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}

impl From<InternalOvk> for ZcashInternalOvk {
    fn from(inner: InternalOvk) -> Self {
        ZcashInternalOvk(inner)
    }
}
