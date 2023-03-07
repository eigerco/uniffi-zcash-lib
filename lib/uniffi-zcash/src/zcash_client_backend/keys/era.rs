use zcash_client_backend::keys::Era;

/// A version identifier for the encoding of unified spending keys.
///
/// Each era corresponds to a range of block heights. During an era, the unified spending key
/// parsed from an encoded form tagged with that era's identifier is expected to provide
/// sufficient spending authority to spend any non-Sprout shielded note created in a transaction
/// within the era's block range.
pub enum ZcashKeysEra {
    /// The Orchard era begins at Orchard activation, and will end if a new pool that requires a
    /// change to unified spending keys is introduced.
    Orchard,
}

impl From<ZcashKeysEra> for Era {
    fn from(value: ZcashKeysEra) -> Self {
        match value {
            ZcashKeysEra::Orchard => Era::Orchard,
        }
    }
}
