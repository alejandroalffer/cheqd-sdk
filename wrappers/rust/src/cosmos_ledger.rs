use {ErrorCode, IndyError};

use futures::Future;
use std::ffi::CString;
use ffi::cosmos_ledger;
use ffi::{ResponseSliceCB, ResponseStringCB};
use utils::callbacks::{ClosureHandler, ResultHandler};
use CommandHandle;

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
) -> Box<dyn Future<Item = Vec<u8>, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err = _build_tx(
        command_handle,
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
        cb,
    );

    ResultHandler::slice(command_handle, err, receiver)
}

fn _build_tx(
    command_handle: CommandHandle,
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
    cb: Option<ResponseSliceCB>,
) -> ErrorCode {
    let pool_alias = c_str!(pool_alias);
    let sender_alias = c_str!(sender_alias);
    let max_coin_denom = c_str!(max_coin_denom);
    let memo = c_str!(memo);

    ErrorCode::from(unsafe {
        cosmos_ledger::indy_cosmos_ledger_build_tx(
            command_handle,
            pool_alias.as_ptr(),
            sender_alias.as_ptr(),
            msg.as_ptr() as *const u8,
            msg.len() as u32,
            account_number,
            sequence_number,
            max_gas,
            max_coin_amount,
            max_coin_denom.as_ptr(),
            timeout_height,
            memo.as_ptr(),
            cb,
        )
    })
}

pub fn build_query_cosmos_auth_account(address: &str) -> Box<dyn Future<Item = String, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_query_cosmos_auth_account(command_handle, address, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_query_cosmos_auth_account(
    command_handle: CommandHandle,
    address: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let address = c_str!(address);

    ErrorCode::from(unsafe {
        cosmos_ledger::indy_cosmos_ledger_build_query_cosmos_auth_account(command_handle, address.as_ptr(), cb)
    })
}

pub fn parse_query_cosmos_auth_account_resp(
    query_resp: &str,
) -> Box<dyn Future<Item = String, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_query_cosmos_auth_account_resp(command_handle, query_resp, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_query_cosmos_auth_account_resp(
    command_handle: CommandHandle,
    query_resp: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let query_resp = c_str!(query_resp);

    ErrorCode::from(unsafe {
        cosmos_ledger::indy_cosmos_ledger_parse_query_cosmos_auth_account_resp(
            command_handle,
            query_resp.as_ptr(),
            cb,
        )
    })
}