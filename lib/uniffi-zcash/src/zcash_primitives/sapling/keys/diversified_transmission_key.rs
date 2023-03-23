use zcash_primitives::sapling::keys::DiversifiedTransmissionKey;

pub struct ZcashSaplingDiversifiedTransmissionKey(DiversifiedTransmissionKey);

impl From<DiversifiedTransmissionKey> for ZcashSaplingDiversifiedTransmissionKey {
    fn from(inner: DiversifiedTransmissionKey) -> Self {
        ZcashSaplingDiversifiedTransmissionKey(inner)
    }
}
