mod chain;
pub use self::chain::*;

mod wallet;
pub use self::wallet::*;

use zcash_client_backend::data_api::ShieldedProtocol;

pub enum ZcashShieldedProtocol {
	// Orchard
	Sapling
}

impl From<ZcashShieldedProtocol> for ShieldedProtocol {
	fn from(e: ZcashShieldedProtocol) -> Self {
		match e {
			ZcashShieldedProtocol::Sapling => ShieldedProtocol::Sapling
		}
	}
}

impl From<ShieldedProtocol> for ZcashShieldedProtocol {
	fn from(e: ShieldedProtocol) -> Self {
		match e {
			ShieldedProtocol::Sapling => ZcashShieldedProtocol::Sapling
		}
	}
}