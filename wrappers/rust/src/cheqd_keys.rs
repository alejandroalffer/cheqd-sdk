use crate::{ErrorCode, IndyError};

use std::ffi::CString;

use futures::Future;

use crate::ffi::cheqd_keys;
use crate::ffi::{ResponseSliceCB, ResponseStringCB};

use crate::utils::callbacks::{ClosureHandler, ResultHandler};
use crate::{CommandHandle, WalletHandle};

pub fn add_random(wallet_handle: WalletHandle, alias: &str) -> Box<dyn Future<Item = String, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _add_random(command_handle, wallet_handle, alias, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _add_random(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    alias: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let alias = c_str!(alias);

    ErrorCode::from(unsafe {
        cheqd_keys::indy_cheqd_keys_add_random(command_handle, wallet_handle,alias.as_ptr(), cb)
    })
}

pub fn add_from_mnemonic(
    wallet_handle: WalletHandle,
    alias: &str,
    mnemonic: &str,
) -> Box<dyn Future<Item = String, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _add_from_mnemonic(command_handle, wallet_handle, alias, mnemonic, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _add_from_mnemonic(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    alias: &str,
    mnemonic: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let alias = c_str!(alias);
    let mnemonic = c_str!(mnemonic);

    ErrorCode::from(unsafe {
        cheqd_keys::indy_cheqd_keys_add_from_mnemonic(
            command_handle,
            wallet_handle,
            alias.as_ptr(),
            mnemonic.as_ptr(),
            cb,
        )
    })
}

pub fn get_info(wallet_handle: WalletHandle, alias: &str) -> Box<dyn Future<Item = String, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _get_info(command_handle, wallet_handle, alias, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _get_info(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    alias: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let alias = c_str!(alias);

    ErrorCode::from(unsafe {
        cheqd_keys::indy_cheqd_keys_get_info(command_handle, wallet_handle, alias.as_ptr(), cb)
    })
}

pub fn get_list_keys(wallet_handle: WalletHandle) -> Box<dyn Future<Item = String, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _get_list_keys(command_handle, wallet_handle, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _get_list_keys(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {

    ErrorCode::from(unsafe {
        cheqd_keys::indy_cheqd_keys_get_list_keys(command_handle, wallet_handle, cb)
    })
}

pub fn sign(wallet_handle: WalletHandle, alias: &str, tx: &[u8]) -> Box<dyn Future<Item = Vec<u8>, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err = _sign(command_handle, wallet_handle, alias, tx, cb);

    ResultHandler::slice(command_handle, err, receiver)
}

fn _sign(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    alias: &str,
    tx: &[u8],
    cb: Option<ResponseSliceCB>,
) -> ErrorCode {
    let alias = c_str!(alias);

    ErrorCode::from(unsafe {
        cheqd_keys::indy_cheqd_keys_sign(
            command_handle,
            wallet_handle,
            alias.as_ptr(),
            tx.as_ptr() as *const u8,
            tx.len() as u32,
            cb,
        )
    })
}
