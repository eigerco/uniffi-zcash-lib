use zcash_primitives::legacy::keys::{ExternalIvk, InternalIvk};

// ExternalIVK (all methods private)
pub struct ZcashExternalIvk(ExternalIvk);

impl ZcashExternalIvk {}

impl From<ExternalIvk> for ZcashExternalIvk {
    fn from(inner: ExternalIvk) -> Self {
        ZcashExternalIvk(inner)
    }
}

// InternalIVK (all methods private)
pub struct ZcashInternalIvk(InternalIvk);

impl ZcashInternalIvk {}

impl From<InternalIvk> for ZcashInternalIvk {
    fn from(inner: InternalIvk) -> Self {
        ZcashInternalIvk(inner)
    }
}
