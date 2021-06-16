use {ErrorCode, IndyError};

use std::ffi::CString;

use futures::Future;

use ffi::verim_keys;
use ffi::{ResponseSliceCB, ResponseStringCB};

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
        verim_keys::indy_verim_keys_add_random(command_handle, alias.as_ptr(), cb)
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
        verim_keys::indy_verim_keys_add_from_mnemonic(
            command_handle,
            alias.as_ptr(),
            mnemonic.as_ptr(),
            cb,
        )
    })
}

pub fn get_info(alias: &str) -> Box<dyn Future<Item = (String), Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _get_info(command_handle, alias, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _get_info(
    command_handle: CommandHandle,
    alias: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let alias = c_str!(alias);

    ErrorCode::from(unsafe {
        verim_keys::indy_verim_keys_get_info(command_handle, alias.as_ptr(), cb)
    })
}

pub fn sign(alias: &str, tx: &[u8]) -> Box<dyn Future<Item = Vec<u8>, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err = _sign(command_handle, alias, tx, cb);

    ResultHandler::slice(command_handle, err, receiver)
}

fn _sign(
    command_handle: CommandHandle,
    alias: &str,
    tx: &[u8],
    cb: Option<ResponseSliceCB>,
) -> ErrorCode {
    let alias = c_str!(alias);

    ErrorCode::from(unsafe {
        verim_keys::indy_verim_keys_sign(
            command_handle,
            alias.as_ptr(),
            tx.as_ptr() as *const u8,
            tx.len() as u32,
            cb,
        )
    })
}
