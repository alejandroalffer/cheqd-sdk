use {ErrorCode, IndyError};

use std::ffi::CString;

use futures::Future;

use ffi::cosmos_pool;
use ffi::{ResponseSliceCB, ResponseStringCB};

use utils::callbacks::{ClosureHandler, ResultHandler};
use CommandHandle;

pub fn add(
    alias: &str,
    rpc_address: &str,
    chain_id: &str,
) -> Box<dyn Future<Item = (String), Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _add(command_handle, alias, rpc_address, chain_id, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _add(
    command_handle: CommandHandle,
    alias: &str,
    rpc_address: &str,
    chain_id: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let alias = c_str!(alias);
    let rpc_address = c_str!(rpc_address);
    let chain_id = c_str!(chain_id);

    ErrorCode::from(unsafe {
        cosmos_pool::indy_cosmos_pool_add(
            command_handle,
            alias.as_ptr(),
            rpc_address.as_ptr(),
            chain_id.as_ptr(),
            cb,
        )
    })
}

pub fn get_config(alias: &str) -> Box<dyn Future<Item = (String), Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _get_config(command_handle, alias, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _get_config(
    command_handle: CommandHandle,
    alias: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let alias = c_str!(alias);

    ErrorCode::from(unsafe {
        cosmos_pool::indy_cosmos_pool_get_config(command_handle, alias.as_ptr(), cb)
    })
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
        cosmos_pool::indy_cosmos_pool_build_tx(
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

pub fn broadcast_tx_commit(
    pool_alias: &str,
    signed_tx: &[u8],
) -> Box<dyn Future<Item = String, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _broadcast_tx_commit(command_handle, pool_alias, signed_tx, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _broadcast_tx_commit(
    command_handle: CommandHandle,
    pool_alias: &str,
    signed_tx: &[u8],
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let pool_alias = c_str!(pool_alias);

    ErrorCode::from(unsafe {
        cosmos_pool::indy_cosmos_pool_broadcast_tx_commit(
            command_handle,
            pool_alias.as_ptr(),
            signed_tx.as_ptr() as *const u8,
            signed_tx.len() as u32,
            cb,
        )
    })
}
