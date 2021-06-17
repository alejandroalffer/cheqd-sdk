use indyrs::{future::Future, IndyError, verim_ledger};

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
    verim_ledger::auth::build_tx(
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

pub fn build_query_account(address: &str) -> Result<String, IndyError> {
    verim_ledger::auth::build_query_account(address).wait()
}

pub fn parse_query_account_resp(query_resp: &str) -> Result<String, IndyError> {
    verim_ledger::auth::parse_query_account_resp(query_resp).wait()
}