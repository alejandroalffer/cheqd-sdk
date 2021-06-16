use {ErrorCode, IndyError};

use std::ffi::CString;

use futures::Future;

use ffi::verim_pool;
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
        verim_pool::indy_verim_pool_add(
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
        verim_pool::indy_verim_pool_get_config(command_handle, alias.as_ptr(), cb)
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
        verim_pool::indy_verim_pool_broadcast_tx_commit(
            command_handle,
            pool_alias.as_ptr(),
            signed_tx.as_ptr() as *const u8,
            signed_tx.len() as u32,
            cb,
        )
    })
}

pub fn abci_query(
    pool_alias: &str,
    req_json: &str,
) -> Box<dyn Future<Item = String, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _abci_query(command_handle, pool_alias, req_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _abci_query(
    command_handle: CommandHandle,
    pool_alias: &str,
    req_json: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let pool_alias = c_str!(pool_alias);
    let req_json = c_str!(req_json);

    ErrorCode::from(unsafe {
        verim_pool::indy_verim_pool_abci_query(
            command_handle,
            pool_alias.as_ptr(),
            req_json.as_ptr(),
            cb,
        )
    })
}

pub fn abci_info(
    pool_alias: &str,
) -> Box<dyn Future<Item = String, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _abci_info(command_handle, pool_alias, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _abci_info(
    command_handle: CommandHandle,
    pool_alias: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let pool_alias = c_str!(pool_alias);

    ErrorCode::from(unsafe {
        verim_pool::indy_verim_pool_abci_info(
            command_handle,
            pool_alias.as_ptr(),
            cb,
        )
    })
}
