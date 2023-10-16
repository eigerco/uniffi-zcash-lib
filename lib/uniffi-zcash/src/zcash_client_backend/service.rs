use zcash_client_backend::proto::service::TreeState;

// substitutes TreeState from the proto Service
// pub struct ZcashTreeState {
//     /// "main" or "test"
//     pub network: String,
//     /// block height
//     pub height: u64,
//     pub hash: String,
//     /// Unix epoch time when the block was mined
//     pub time: u32,
//     /// sapling commitment tree state
//     pub sapling_tree: String,
//     /// orchard commitment tree state
//     pub orchard_tree: String,

//     internal: TreeState
// }

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
