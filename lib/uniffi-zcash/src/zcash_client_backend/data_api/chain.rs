// use zcash_primitives::consensus::MainNetwork;
// use zcash_primitives::consensus::TestNetwork;
use zcash_client_backend::data_api::chain;
use zcash_client_backend::data_api::chain::BlockSource;
use zcash_client_sqlite::{FsBlockDb, WalletDb};

use crate::{
    ZcashConsensusParameters,
    ZcashFsBlockDb,
    ZcashWalletDb,
    ZcashResult,
    ZcashError
};


pub fn scan_cached_blocks(params: ZcashConsensusParameters, z_db_cache: ZcashFsBlockDb, z_db_data: ZcashWalletDb, limit: u32) -> ZcashResult<()> {

    let db_cache = z_db_cache.fs_block_db.into_inner().unwrap();

    let main = &z_db_data.sup.main.lock().unwrap();
    let test = &z_db_data.sup.test.lock().unwrap();

    match params {
        ZcashConsensusParameters::MainNetwork => {
            let mut db_data = main.get_update_ops().unwrap();
            match chain::scan_cached_blocks(&params, &db_cache, &mut db_data, Some(limit)) {
                Ok(_) => Ok(()),
                Err(_) => Err(ZcashError::Unknown)
            }
        },

        ZcashConsensusParameters::TestNetwork => {
            let mut db_data = test.get_update_ops().unwrap();
            match chain::scan_cached_blocks(&params, &db_cache, &mut db_data, Some(limit)) {
                Ok(_) => Ok(()),
                Err(_) => Err(ZcashError::Unknown)
            }
        }
    }
}


// // https://github.com/zcash/librustzcash/blob/224e0215584c7bc997e2e552a45f78dad702bb8b/zcash_client_backend/src/data_api/chain.rs#L237

// #[tracing::instrument(skip(params, block_source, data_db))]
// #[allow(clippy::type_complexity)]
// pub fn scan_cached_blocks<ParamsT, DbT, BlockSourceT>(

// #[tracing::instrument(skip(params, block_source, data_db))]
// #[allow(clippy::type_complexity)]
// pub fn scan_cached_blocks<ParamsT, DbT, BlockSourceT>(
//     params: &ParamsT,
//     block_source: &BlockSourceT,
//     data_db: &mut DbT,
//     limit: Option<u32>,
// ) -> Result<(), Error<DbT::Error, BlockSourceT::Error, DbT::NoteRef>>
// where
//     ParamsT: consensus::Parameters + Send + 'static,
//     BlockSourceT: BlockSource,
//     DbT: WalletWrite,
// {




// pub struct MockBlockSource;

// impl BlockSource for MockBlockSource {
//     type Error = Infallible;

//     fn with_blocks<F, DbErrT, NoteRef>(
//         &self,
//         _from_height: BlockHeight,
//         _limit: Option<u32>,
//         _with_row: F,
//     ) -> Result<(), Error<DbErrT, Infallible, NoteRef>>
//     where
//         F: FnMut(CompactBlock) -> Result<(), Error<DbErrT, Infallible, NoteRef>>,
//     {
//         Ok(())
//     }
// }


// // https://github.com/zcash/librustzcash/blob/224e0215584c7bc997e2e552a45f78dad702bb8b/zcash_client_backend/src/data_api/chain.rs#L168C1-L173C32

// pub struct CommitmentTreeRoot<H> {
//     subtree_end_height: BlockHeight,
//     root_hash: H,
// }

// impl<H> CommitmentTreeRoot<H> {

