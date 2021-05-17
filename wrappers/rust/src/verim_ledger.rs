use {ErrorCode, IndyError};

use std::ffi::CString;

use futures::Future;

use ffi::cosmos_keys;
use ffi::{ResponseKeyInfoCB};

use utils::callbacks::{ClosureHandler, ResultHandler};
use {CommandHandle};

pub fn build_msg_create_nym(did: &str, creator: &str, verkey: &str, alias: &str, role: &str, ) -> Box<dyn Future<Item=(String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_msg_create_nym(command_handle, did, creator, verkey, alias, role, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_msg_create_nym(command_handle: CommandHandle, did: &str, creator: &str, verkey: &str, alias: &str, role: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let did = c_str!(did);
    let creator = c_str!(creator);
    let verkey = c_str!(verkey);
    let alias = c_str!(alias);
    let role = c_str!(role);

    ErrorCode::from(unsafe { cosmos_keys::indy_build_msg_create_nym(command_handle, did.as_ptr(), creator.as_ptr(), verkey.as_ptr(), alias.as_ptr(), role.as_ptr(), cb) })
}

pub fn build_msg_update_nym(did: &str, creator: &str, verkey: &str, alias: &str, role: &str) -> Box<dyn Future<Item=(String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_msg_update_nym(command_handle, did, creator, verkey, alias, role, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_msg_update_nym(command_handle: CommandHandle, did: &str, creator: &str, verkey: &str, alias: &str, role: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let did = c_str!(did);
    let creator = c_str!(creator);
    let verkey = c_str!(verkey);
    let alias = c_str!(alias);
    let role = c_str!(role);

    ErrorCode::from(unsafe { cosmos_keys::indy_build_msg_create_nym(command_handle, did.as_ptr(), creator.as_ptr(), verkey.as_ptr(), alias.as_ptr(), role.as_ptr(), cb) })
}

pub fn build_msg_delete_nym(id: &str, creator: &str) -> Box<dyn Future<Item=(String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_msg_delete_nym(command_handle, id, creator);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_msg_delete_nym(command_handle: CommandHandle, id: &str, creator: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let creator = c_str!(creator);
    let id = c_str!(did);

    ErrorCode::from(unsafe { cosmos_keys::indy_build_msg_delete_nym(command_handle, id.as_ptr(), creator.as_ptr(), cb) })
}