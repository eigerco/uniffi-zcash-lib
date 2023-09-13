use zcash_client_backend::data_api::chain;

use crate::{
    ZcashConsensusParameters,
    ZcashFsBlockDb,
    ZcashWalletDb,
    ZcashResult,
    ZcashError,
    ZcashBlockHeight
};


pub fn scan_cached_blocks(params: ZcashConsensusParameters, z_db_cache: ZcashFsBlockDb, z_db_data: ZcashWalletDb, height: ZcashBlockHeight, limit: u32) -> ZcashResult<()> {

    let db_cache = z_db_cache.fs_block_db.into_inner().unwrap();

    match params {
        ZcashConsensusParameters::MainNetwork => {
            let mut main_db_data = z_db_data.sup.main.lock().unwrap();
            match chain::scan_cached_blocks(&params, &db_cache, &mut (*main_db_data), height.into(), limit as usize) {
                Ok(_) => Ok(()),
                Err(_) => Err(ZcashError::Unknown)
            }
        },

        ZcashConsensusParameters::TestNetwork => {
            let mut test_db_data = z_db_data.sup.test.lock().unwrap();
            match chain::scan_cached_blocks(&params, &db_cache, &mut (*test_db_data), height.into(), limit as usize) {
                Ok(_) => Ok(()),
                Err(_) => Err(ZcashError::Unknown)
            }
        }
    }
}