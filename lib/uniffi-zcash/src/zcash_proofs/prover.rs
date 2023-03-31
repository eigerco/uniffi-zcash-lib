use std::path::Path;

use zcash_proofs::prover::LocalTxProver;

use crate::ZcashResult;
pub struct ZcashLocalTxProver(LocalTxProver);

impl ZcashLocalTxProver {
    /// Creates a `LocalTxProver` using parameters from the given local paths.
    ///
    /// This function will panic if the paths do not point to valid parameter files with
    /// the expected hashes.
    pub fn new(spend_path: &str, output_path: &str) -> Self {
        LocalTxProver::new(Path::new(spend_path), Path::new(output_path)).into()
    }

    /// Creates a `LocalTxProver` using parameters specified as byte arrays.
    pub fn from_bytes(spend_param_bytes: &[u8], output_param_bytes: &[u8]) -> Self {
        LocalTxProver::from_bytes(spend_param_bytes, output_param_bytes).into()
    }

    /// Attempts to create a `LocalTxProver` using parameters from the default local
    /// location.
    pub fn with_default_location() -> ZcashResult<Self> {
        match LocalTxProver::with_default_location() {
            Some(prover) => Ok(prover.into()),
            None => Err("Parameters cannot be found in default location".into()),
        }
    }

    /// Creates a `LocalTxProver` using Sapling parameters bundled inside the binary.
    ///
    /// This requires the `bundled-prover` feature, which will increase the binary size by
    /// around 50 MiB.
    pub fn bundled() -> Self {
        LocalTxProver::bundled().into()
    }
}

impl From<LocalTxProver> for ZcashLocalTxProver {
    fn from(inner: LocalTxProver) -> Self {
        ZcashLocalTxProver(inner)
    }
}
