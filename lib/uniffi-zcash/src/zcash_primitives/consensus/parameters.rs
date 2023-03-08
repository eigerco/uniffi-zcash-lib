use delegate::delegate;

use zcash_address::Network;
use zcash_primitives::consensus::{
    BlockHeight, MainNetwork, NetworkUpgrade, Parameters, TestNetwork,
};

/// Zcash consensus parameters.
#[derive(Copy, Clone)]
pub enum ZcashConsensusParameters {
    /// Marker struct for the production network.
    MainNetwork,

    /// Marker struct for the test network.
    TestNetwork,
    // todo: expose a way to use other types of parameters
}

impl Parameters for ZcashConsensusParameters {
    delegate! {
        to match self {
            ZcashConsensusParameters::MainNetwork => MainNetwork,
            ZcashConsensusParameters::TestNetwork => TestNetwork,
        } {
            fn activation_height(
                &self,
                nu: NetworkUpgrade,
            ) -> Option<BlockHeight>;

            fn coin_type(&self) -> u32;

            fn address_network(&self) -> Option<Network>;

            fn hrp_sapling_extended_spending_key(&self) -> &str;

            fn hrp_sapling_extended_full_viewing_key(&self) -> &str;

            fn hrp_sapling_payment_address(&self) -> &str;

            fn b58_pubkey_address_prefix(&self) -> [u8; 2];

            fn b58_script_address_prefix(&self) -> [u8; 2];
        }
    }
}
