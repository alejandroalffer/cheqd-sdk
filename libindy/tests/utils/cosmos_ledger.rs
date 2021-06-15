use indyrs::{verim_keys, cosmos_ledger, future::Future, IndyError, PoolHandle, WalletHandle};

use crate::utils::{constants::DEFAULT_METHOD_NAME, ledger, pool, types::ResponseType};

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
    cosmos_ledger::build_tx(
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

pub fn build_query_cosmos_auth_account(address: &str) -> Result<String, IndyError> {
    cosmos_ledger::build_query_cosmos_auth_account(address).wait()
}

pub fn parse_query_cosmos_auth_account_resp(query_resp: &str) -> Result<String, IndyError> {
    cosmos_ledger::parse_query_cosmos_auth_account_resp(query_resp).wait()
}