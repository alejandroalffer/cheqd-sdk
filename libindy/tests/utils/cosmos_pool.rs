use indyrs::{cosmos_keys, cosmos_pool, future::Future, IndyError, PoolHandle, WalletHandle};

use crate::utils::{constants::DEFAULT_METHOD_NAME, ledger, pool, types::ResponseType};

pub fn add(alias: &str, rpc_address: &str, chain_id: &str) -> Result<String, IndyError> {
    cosmos_pool::add(alias, rpc_address, chain_id).wait()
}

pub fn get_config(alias: &str) -> Result<String, IndyError> {
    cosmos_pool::get_config(alias).wait()
}

pub fn build_tx(
    pool_alias: &str,
    sender_alias: &str,
    msg: &[u8],
    account_number: u64,
    sequence_number: u64,
    max_gas: u64,
    max_coin_amount: u64,
    max_coin_denom: &str,
    timeout_height: u64,
    memo: &str,
) -> Result<Vec<u8>, IndyError> {
    cosmos_pool::build_tx(
        pool_alias,
        sender_alias,
        msg,
        account_number,
        sequence_number,
        max_gas,
        max_coin_amount,
        max_coin_denom,
        timeout_height,
        memo,
    )
    .wait()
}

pub fn broadcast_tx_commit(pool_alias: &str, signed_tx: &[u8]) -> Result<String, IndyError> {
    cosmos_pool::broadcast_tx_commit(pool_alias, signed_tx).wait()
}

pub fn abci_query(pool_alias: &str, req_json: &str) -> Result<String, IndyError> {
    cosmos_pool::abci_query(pool_alias, req_json).wait()
}
