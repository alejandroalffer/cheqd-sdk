use {ErrorCode, IndyError};

use std::ffi::CString;

use futures::Future;

use ffi::verim_ledger;
use ffi::{ResponseSliceCB};

use utils::callbacks::{ClosureHandler, ResultHandler};
use {CommandHandle};

pub fn build_msg_create_nym(did: &str, creator: &str, verkey: &str, alias: &str, role: &str, ) -> Box<dyn Future<Item=(Vec<u8>), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err = _build_msg_create_nym(command_handle, did, creator, verkey, alias, role, cb);

    ResultHandler::slice(command_handle, err, receiver)
}

fn _build_msg_create_nym(command_handle: CommandHandle, did: &str, creator: &str, verkey: &str, alias: &str, role: &str, cb: Option<ResponseSliceCB>) -> ErrorCode {
    let did = c_str!(did);
    let creator = c_str!(creator);
    let verkey = c_str!(verkey);
    let alias = c_str!(alias);
    let role = c_str!(role);

    ErrorCode::from(unsafe { verim_ledger::indy_build_msg_create_nym(command_handle, did.as_ptr(), creator.as_ptr(), verkey.as_ptr(), alias.as_ptr(), role.as_ptr(), cb) })
}

pub fn build_msg_update_nym( did: &str, creator: &str, verkey: &str, alias: &str, role: &str, id: u64) -> Box<dyn Future<Item=(Vec<u8>), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err = _build_msg_update_nym(command_handle, did, creator, verkey, alias, role, id, cb);

    ResultHandler::slice(command_handle, err, receiver)
}

fn _build_msg_update_nym(command_handle: CommandHandle, did: &str, creator: &str, verkey: &str, alias: &str, role: &str, id: u64, cb: Option<ResponseSliceCB>) -> ErrorCode {
    let did = c_str!(did);
    let creator = c_str!(creator);
    let verkey = c_str!(verkey);
    let alias = c_str!(alias);
    let role = c_str!(role);

    ErrorCode::from(unsafe { verim_ledger::indy_build_msg_update_nym(command_handle, did.as_ptr(), creator.as_ptr(), verkey.as_ptr(), alias.as_ptr(), role.as_ptr(), id, cb) })
}

pub fn build_msg_delete_nym(creator: &str, id: u64) -> Box<dyn Future<Item=(Vec<u8>), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err = _build_msg_delete_nym(command_handle, creator, id, cb);

    ResultHandler::slice(command_handle, err, receiver)
}

fn _build_msg_delete_nym(command_handle: CommandHandle, creator: &str, id: u64, cb: Option<ResponseSliceCB>) -> ErrorCode {
    let creator = c_str!(creator);

    ErrorCode::from(unsafe { verim_ledger::indy_build_msg_delete_nym(command_handle, creator.as_ptr(), id, cb) })
}