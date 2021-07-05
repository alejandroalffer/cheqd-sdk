use indy::IndyError;
use indy::future::Future;
use indy::verim_pool;

pub struct VerimPool {}

impl VerimPool {
    pub fn add(alias: &str, rpc_address: &str, chain_id: &str) -> Result<String, IndyError> {
        verim_pool::add(alias, rpc_address, chain_id).wait()
    }

    pub fn get_config(alias: &str) -> Result<String, IndyError> {
        verim_pool::get_config(alias).wait()
    }

    pub fn broadcast_tx_commit(pool_alias: &str, signed_tx: &[u8]) -> Result<String, IndyError> {
        verim_pool::broadcast_tx_commit(pool_alias, signed_tx).wait()
    }

    pub fn abci_query(pool_alias: &str, req_json: &str) -> Result<String, IndyError> {
        verim_pool::abci_query(pool_alias, req_json).wait()
    }

    pub fn abci_info(pool_alias: &str) -> Result<String, IndyError> {
        verim_pool::abci_info(pool_alias).wait()
    }
}