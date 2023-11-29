use crate::{ZcashError, ZcashResult};
use derive_more::{From, Into};
use prost::Message;
use zcash_client_backend::proto::service::TreeState;

#[derive(Clone, From, Into)]
pub struct ZcashTreeState(TreeState);

impl ZcashTreeState {
    pub fn new(
        network: String,
        height: u64,
        hash: String,
        time: u32,
        sapling_tree: String,
        orchard_tree: String,
    ) -> Self {
        Self(TreeState {
            network,
            height,
            hash,
            time,
            sapling_tree,
            orchard_tree,
        })
    }

    pub fn from_bytes(bytes: Vec<u8>) -> ZcashResult<Self> {
        let treestate = TreeState::decode(&bytes[..]).map_err(|e| ZcashError::Message {
            error: format!("Invalid TreeState: {}", e),
        })?;
        Ok(Self(treestate))
    }
}
