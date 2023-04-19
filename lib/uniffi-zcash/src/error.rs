use std::{convert::Infallible, num::TryFromIntError};

use orchard::zip32;
use zcash_primitives::transaction::{self, fees};

/// Zcash error.
#[derive(Debug, thiserror::Error)]
pub enum ZcashError {
    #[error("hdwallet error occurred: {error:?}")]
    HDWalletError { error: hdwallet::error::Error },

    #[error("derivation error occurred: {error:?}")]
    DerivationError {
        error: zcash_client_backend::keys::DerivationError,
    },

    #[error("decoding error occurred: {error:?}")]
    DecodingError {
        error: zcash_client_backend::keys::DecodingError,
    },

    #[error("could not decode the `ask` bytes to a jubjub field element")]
    InvalidAsk,

    #[error("could not decode the `nsk` bytes to a jubjub field element")]
    InvalidNsk,

    #[error("error occurred: {error}")]
    Message { error: String },

    #[error("expected {expected} elements, got {got}")]
    ArrayLengthMismatch { expected: u64, got: u64 },

    #[error("value {val} out of range, should be within {from}..{to}")]
    ValueOutOfRange { val: i64, from: i64, to: i64 },

    #[error("Secp256k1 error occurred: {error:?}")]
    Secp256k1Error { error: secp256k1::Error },

    #[error("Bech32 decoding error occurred: {error}")]
    Bech32DecodeError {
        error: zcash_client_backend::encoding::Bech32DecodeError,
    },

    #[error("Base58 decoding error occurred: {error}")]
    Bs58Error { error: bs58::decode::Error },

    #[error("General builder error occurred: {error:?}")]
    BuilderError {
        error: transaction::builder::Error<fees::zip317::FeeError>,
    },

    #[error("Transparent builder error occurred: {error:?}")]
    TransparentBuilderError {
        error: transaction::components::transparent::builder::Error,
    },

    #[error("Sapling builder error occurred: {error:?}")]
    SaplingBuilderError {
        error: transaction::components::sapling::builder::Error,
    },

    #[error("Orchard builder error occurred: {error:?}")]
    OrchardBuilderError { error: orchard::builder::Error },

    #[error("Insufficient founds error: {amount}")]
    InsufficientFundsError { amount: u64 },

    #[error("Change required error: {amount}")]
    ChangeRequiredError { amount: u64 },

    #[error("IO error occurred: {error:?}")]
    IOError { error: std::io::Error },

    #[error("unknown error occurred")]
    Unknown,
}

impl From<hdwallet::error::Error> for ZcashError {
    fn from(error: hdwallet::error::Error) -> Self {
        ZcashError::HDWalletError { error }
    }
}

impl From<zcash_client_backend::keys::DecodingError> for ZcashError {
    fn from(error: zcash_client_backend::keys::DecodingError) -> Self {
        ZcashError::DecodingError { error }
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

impl From<zcash_primitives::sapling::keys::DecodingError> for ZcashError {
    fn from(error: zcash_primitives::sapling::keys::DecodingError) -> Self {
        match error {
            zcash_primitives::sapling::keys::DecodingError::LengthInvalid { expected, actual } => {
                ZcashError::ArrayLengthMismatch {
                    expected: expected as u64,
                    got: actual as u64,
                }
            }
            zcash_primitives::sapling::keys::DecodingError::InvalidAsk => ZcashError::InvalidAsk,
            zcash_primitives::sapling::keys::DecodingError::InvalidNsk => ZcashError::InvalidNsk,
        }
    }
}

impl From<String> for ZcashError {
    fn from(error: String) -> Self {
        ZcashError::Message { error }
    }
}

impl From<&str> for ZcashError {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<secp256k1::Error> for ZcashError {
    fn from(error: secp256k1::Error) -> Self {
        ZcashError::Secp256k1Error { error }
    }
}

impl From<zcash_client_backend::encoding::Bech32DecodeError> for ZcashError {
    fn from(error: zcash_client_backend::encoding::Bech32DecodeError) -> Self {
        ZcashError::Bech32DecodeError { error }
    }
}

impl From<bs58::decode::Error> for ZcashError {
    fn from(error: bs58::decode::Error) -> Self {
        ZcashError::Bs58Error { error }
    }
}

impl From<transaction::builder::Error<Infallible>> for ZcashError {
    fn from(error: transaction::builder::Error<Infallible>) -> Self {
        error.to_string().into()
    }
}

impl From<transaction::builder::Error<fees::zip317::FeeError>> for ZcashError {
    fn from(error: transaction::builder::Error<fees::zip317::FeeError>) -> Self {
        match error {
            transaction::builder::Error::InsufficientFunds(amount) => {
                ZcashError::ChangeRequiredError {
                    amount: amount.into(),
                }
            }
            transaction::builder::Error::ChangeRequired(amount) => {
                ZcashError::ChangeRequiredError {
                    amount: amount.into(),
                }
            }
            transaction::builder::Error::Fee(_) => ZcashError::BuilderError { error },
            transaction::builder::Error::Balance(_) => ZcashError::BuilderError { error },
            transaction::builder::Error::TransparentBuild(_) => ZcashError::BuilderError { error },
            transaction::builder::Error::SaplingBuild(_) => ZcashError::BuilderError { error },
        }
    }
}

impl From<transaction::components::transparent::builder::Error> for ZcashError {
    fn from(error: transaction::components::transparent::builder::Error) -> Self {
        ZcashError::TransparentBuilderError { error }
    }
}

impl From<transaction::components::sapling::builder::Error> for ZcashError {
    fn from(error: transaction::components::sapling::builder::Error) -> Self {
        ZcashError::SaplingBuilderError { error }
    }
}

impl From<orchard::builder::Error> for ZcashError {
    fn from(error: orchard::builder::Error) -> Self {
        ZcashError::OrchardBuilderError { error }
    }
}

impl From<std::io::Error> for ZcashError {
    fn from(error: std::io::Error) -> Self {
        ZcashError::IOError { error }
    }
}

impl From<TryFromIntError> for ZcashError {
    fn from(value: TryFromIntError) -> Self {
        value.to_string().into()
    }
}
