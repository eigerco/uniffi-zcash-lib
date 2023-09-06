use std::sync::Arc;

use crate::ZcashExtractedNoteCommitment;
use crate::{utils::cast_slice, ZcashResult};

use orchard::tree::{MerkleHashOrchard, MerklePath};

use orchard::Anchor;

pub struct ZcashAnchor(Anchor);

impl ZcashAnchor {
    pub fn from_bytes(bytes: &[u8]) -> ZcashResult<Self> {
        let opt: Option<Anchor> = Anchor::from_bytes(cast_slice(bytes)?).into();
        match opt {
            Some(anchor) => Ok(anchor.into()),
            None => Err("Error parsing bytes".into()),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

impl From<&Anchor> for ZcashAnchor {
    fn from(inner: &Anchor) -> Self {
        ZcashAnchor(*inner)
    }
}

impl From<Anchor> for ZcashAnchor {
    fn from(inner: Anchor) -> Self {
        ZcashAnchor(inner)
    }
}

impl From<&ZcashAnchor> for Anchor {
    fn from(value: &ZcashAnchor) -> Self {
        value.0
    }
}

/// The Merkle path from a leaf of the note commitment tree
/// to its anchor.
pub struct ZcashOrchardMerklePath {
    inner: MerklePath,
    /// We hold here the creation values, for later
    /// conversions through the From trait. This is
    /// because currently inner type does not implement
    /// Clone nor Copy traits.
    position: u32,
    auth_path: [MerkleHashOrchard; 32],
}

impl ZcashOrchardMerklePath {
    /// Instantiates a new Merkle path given a leaf position and authentication path.
    pub fn from_parts(
        position: u32,
        auth_path: Vec<Arc<ZcashOrchardMerkleHash>>,
    ) -> ZcashResult<Self> {
        let inner_auth_type: Vec<MerkleHashOrchard> = auth_path.iter().map(|v| v.0).collect();
        let casted_auth_path: [MerkleHashOrchard; 32] = cast_slice(inner_auth_type.as_slice())?;
        let inner = MerklePath::from_parts(position, casted_auth_path);
        Ok(ZcashOrchardMerklePath {
            inner,
            position,
            auth_path: casted_auth_path,
        })
    }

    /// <https://zips.z.cash/protocol/protocol.pdf#orchardmerklecrh>
    /// The layer with 2^n nodes is called "layer n":
    ///      - leaves are at layer MERKLE_DEPTH_ORCHARD = 32;
    ///      - the root is at layer 0.
    /// `l` is MERKLE_DEPTH_ORCHARD - layer - 1.
    ///      - when hashing two leaves, we produce a node on the layer above the leaves, i.e.
    ///        layer = 31, l = 0
    ///      - when hashing to the final root, we produce the anchor with layer = 0, l = 31.
    pub fn root(&self, cmx: Arc<ZcashExtractedNoteCommitment>) -> Arc<ZcashAnchor> {
        Arc::new(self.inner.root(cmx.as_ref().into()).into())
    }
}

impl From<&ZcashOrchardMerklePath> for MerklePath {
    fn from(value: &ZcashOrchardMerklePath) -> Self {
        MerklePath::from_parts(value.position, value.auth_path)
    }
}

/// A newtype wrapper for leaves and internal nodes in the Orchard
/// incremental note commitment tree.
pub struct ZcashOrchardMerkleHash(MerkleHashOrchard);

impl ZcashOrchardMerkleHash {
    pub fn from_bytes(data: &[u8]) -> ZcashResult<Self> {
        let opt: Option<MerkleHashOrchard> =
            MerkleHashOrchard::from_bytes(&cast_slice(data)?).into();
        match opt {
            Some(merkle_hash) => Ok(merkle_hash.into()),
            None => Err("Error parsing bytes".into()),
        }
    }

    pub fn from_cmx(cmx: &ZcashExtractedNoteCommitment) -> Self {
        MerkleHashOrchard::from_cmx(&cmx.into()).into()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

impl From<MerkleHashOrchard> for ZcashOrchardMerkleHash {
    fn from(inner: MerkleHashOrchard) -> Self {
        ZcashOrchardMerkleHash(inner)
    }
}
