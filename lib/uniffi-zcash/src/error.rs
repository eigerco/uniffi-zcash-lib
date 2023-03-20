use orchard::zip32;

/// Zcash error.
#[derive(Debug, thiserror::Error)]
pub enum ZcashError {
    #[error("hdwallet error occurred: {error:?}")]
    HDWalletError { error: hdwallet::error::Error },

    #[error("derivation error occurred: {error:?}")]
    DerivationError {
        error: zcash_client_backend::keys::DerivationError,
    },

    #[error("error occurred: {error}")]
    Message { error: String },

    #[error("expected {expected} elements, got {got}")]
    ArrayLengthMismatch { expected: u64, got: u64 },

    #[error("Value {val} out of range, should be within {from}..{to}")]
    ValueOutOfRange { val: i64, from: i64, to: i64 },

    #[error("Secp256k1 error occurred: {error:?}")]
    Secp256k1Error { error: secp256k1::Error },

    #[error("unknown error occurred")]
    Unknown,
}

impl From<hdwallet::error::Error> for ZcashError {
    fn from(error: hdwallet::error::Error) -> Self {
        ZcashError::HDWalletError { error }
    }
}

impl From<zcash_client_backend::keys::DerivationError> for ZcashError {
    fn from(error: zcash_client_backend::keys::DerivationError) -> Self {
        ZcashError::DerivationError { error }
    }
}

impl From<zip32::Error> for ZcashError {
    fn from(error: zip32::Error) -> Self {
        error.to_string().into()
    }
}

impl From<String> for ZcashError {
    fn from(error: String) -> Self {
        ZcashError::Message { error }
    }
}

impl From<secp256k1::Error> for ZcashError {
    fn from(error: secp256k1::Error) -> Self {
        ZcashError::Secp256k1Error { error }
    }
}
