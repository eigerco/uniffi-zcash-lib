// use std::sync::Arc;
use std::{fmt, path::Path};

use zcash_proofs::prover::LocalTxProver;

use crate::ZcashResult;

pub struct ZcashLocalTxProver {
    pub(crate) internal: LocalTxProver,
    spend_path: String,
    output_path: String,
}

impl ZcashLocalTxProver {
    /// Creates a `LocalTxProver` using parameters from the given local paths.
    ///
    /// This function will panic if the paths do not point to valid parameter files with
    /// the expected hashes.
    pub fn new(spend_path: &str, output_path: &str) -> Self {
        Self {
            internal: LocalTxProver::new(Path::new(spend_path), Path::new(output_path)),
            spend_path: spend_path.to_string(),
            output_path: output_path.to_string(),
        }
    }

    /// Creates a `LocalTxProver` using parameters specified as byte arrays.
    pub fn from_bytes(spend_param_bytes: &[u8], output_param_bytes: &[u8]) -> Self {
        Self {
            internal: LocalTxProver::from_bytes(spend_param_bytes, output_param_bytes),
            spend_path: "".to_string(),
            output_path: "".to_string(),
        }
    }

    /// Attempts to create a `LocalTxProver` using parameters from the default local
    /// location.
    pub fn with_default_location() -> ZcashResult<Self> {
        match LocalTxProver::with_default_location() {
            Some(prover) => Ok(prover.into()),
            None => Err("Parameters cannot be found in default location".into()),
        }
    }
}

// NOTE change this
impl fmt::Debug for ZcashLocalTxProver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZcashLocalTxProver")
    }
}

impl Clone for ZcashLocalTxProver {
    fn clone(&self) -> Self {
        Self::new(&self.spend_path, &self.output_path)
    }
}

impl From<LocalTxProver> for ZcashLocalTxProver {
    fn from(inner: LocalTxProver) -> Self {
        ZcashLocalTxProver {
            internal: inner,
            spend_path: "".to_string(),
            output_path: "".to_string(),
        }
    }
}

impl From<ZcashLocalTxProver> for LocalTxProver {
    fn from(value: ZcashLocalTxProver) -> Self {
        value.internal
    }
}

// impl From<ZcashLocalTxProver> for LocalTxProver {
//     fn from(value: ZcashLocalTxProver) -> Self {
//         value.0
//     }
// }
