mod error;
pub use error::*;

mod payment;
pub use self::payment::*;

mod hdwallet;
pub use self::hdwallet::*;

mod secp256k1;
pub use self::secp256k1::*;

mod orchard;
pub use self::orchard::*;

mod zcash_client_backend;
pub use self::zcash_client_backend::*;

mod zcash_primitives;
pub use self::zcash_primitives::*;

mod utils;

#[cfg(feature = "rustler")]
mod beam;

pub type ZcashResult<T> = Result<T, ZcashError>;

uniffi::include_scaffolding!("zcash");
