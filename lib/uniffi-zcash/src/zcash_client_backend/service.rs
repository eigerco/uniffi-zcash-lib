use crate::{ZcashError, ZcashResult};
use prost::Message;
use zcash_client_backend::proto::service::TreeState;

#[derive(Clone)]
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

impl From<TreeState> for ZcashTreeState {
    fn from(e: TreeState) -> Self {
        Self(e)
    }
}

impl From<ZcashTreeState> for TreeState {
    fn from(inner: ZcashTreeState) -> Self {
        inner.0
    }
}
