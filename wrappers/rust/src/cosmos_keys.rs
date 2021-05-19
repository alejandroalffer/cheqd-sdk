use {ErrorCode, IndyError};

use std::ffi::CString;

use futures::Future;

use ffi::cosmos_keys;
use ffi::ResponseStringCB;

use utils::callbacks::{ClosureHandler, ResultHandler};
use CommandHandle;

pub fn add_random(alias: &str) -> Box<dyn Future<Item = (String), Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _add_random(command_handle, alias, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _add_random(
    command_handle: CommandHandle,
    alias: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let alias = c_str!(alias);

    ErrorCode::from(unsafe {
        cosmos_keys::indy_cosmos_keys_add_random(command_handle, alias.as_ptr(), cb)
    })
}

pub fn add_from_mnemonic(
    alias: &str,
    mnemonic: &str,
) -> Box<dyn Future<Item = (String), Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _add_from_mnemonic(command_handle, alias, mnemonic, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _add_from_mnemonic(
    command_handle: CommandHandle,
    alias: &str,
    mnemonic: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let alias = c_str!(alias);
    let mnemonic = c_str!(mnemonic);

    ErrorCode::from(unsafe {
        cosmos_keys::indy_cosmos_keys_add_from_mnemonic(
            command_handle,
            alias.as_ptr(),
            mnemonic.as_ptr(),
            cb,
        )
    })
}

pub fn key_info(alias: &str) -> Box<dyn Future<Item = (String), Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _key_info(command_handle, alias, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _key_info(
    command_handle: CommandHandle,
    alias: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let alias = c_str!(alias);

    ErrorCode::from(unsafe {
        cosmos_keys::indy_cosmos_keys_key_info(command_handle, alias.as_ptr(), cb)
    })
}
