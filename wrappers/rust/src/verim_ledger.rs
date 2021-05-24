use {ErrorCode, IndyError};

use std::ffi::CString;

use futures::Future;

use ffi::verim_ledger;
use ffi::{ResponseSliceCB, ResponseStringCB};

use utils::callbacks::{ClosureHandler, ResultHandler};
use CommandHandle;

pub fn build_msg_create_nym(
    did: &str,
    creator: &str,
    verkey: &str,
    alias: &str,
    role: &str,
) -> Box<dyn Future<Item = (Vec<u8>), Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err = _build_msg_create_nym(command_handle, did, creator, verkey, alias, role, cb);

    ResultHandler::slice(command_handle, err, receiver)
}

fn _build_msg_create_nym(
    command_handle: CommandHandle,
    did: &str,
    creator: &str,
    verkey: &str,
    alias: &str,
    role: &str,
    cb: Option<ResponseSliceCB>,
) -> ErrorCode {
    let did = c_str!(did);
    let creator = c_str!(creator);
    let verkey = c_str!(verkey);
    let alias = c_str!(alias);
    let role = c_str!(role);

    ErrorCode::from(unsafe {
        verim_ledger::indy_verim_ledger_build_msg_create_nym(
            command_handle,
            did.as_ptr(),
            creator.as_ptr(),
            verkey.as_ptr(),
            alias.as_ptr(),
            role.as_ptr(),
            cb,
        )
    })
}

pub fn parse_msg_create_nym_resp(
    commit_resp: &str,
) -> Box<dyn Future<Item = (String), Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_msg_create_nym_resp(command_handle, commit_resp, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_msg_create_nym_resp(
    command_handle: CommandHandle,
    commit_resp: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let commit_resp = c_str!(commit_resp);

    ErrorCode::from(unsafe {
        verim_ledger::indy_verim_ledger_parse_msg_create_nym_resp(
            command_handle,
            commit_resp.as_ptr(),
            cb,
        )
    })
}

pub fn build_msg_update_nym(
    did: &str,
    creator: &str,
    verkey: &str,
    alias: &str,
    role: &str,
    id: u64,
) -> Box<dyn Future<Item = (Vec<u8>), Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err = _build_msg_update_nym(command_handle, did, creator, verkey, alias, role, id, cb);

    ResultHandler::slice(command_handle, err, receiver)
}

fn _build_msg_update_nym(
    command_handle: CommandHandle,
    did: &str,
    creator: &str,
    verkey: &str,
    alias: &str,
    role: &str,
    id: u64,
    cb: Option<ResponseSliceCB>,
) -> ErrorCode {
    let did = c_str!(did);
    let creator = c_str!(creator);
    let verkey = c_str!(verkey);
    let alias = c_str!(alias);
    let role = c_str!(role);

    ErrorCode::from(unsafe {
        verim_ledger::indy_verim_ledger_build_msg_update_nym(
            command_handle,
            did.as_ptr(),
            creator.as_ptr(),
            verkey.as_ptr(),
            alias.as_ptr(),
            role.as_ptr(),
            id,
            cb,
        )
    })
}

pub fn parse_msg_update_nym_resp(
    commit_resp: &str,
) -> Box<dyn Future<Item = (String), Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_msg_update_nym_resp(command_handle, commit_resp, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_msg_update_nym_resp(
    command_handle: CommandHandle,
    commit_resp: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let commit_resp = c_str!(commit_resp);

    ErrorCode::from(unsafe {
        verim_ledger::indy_verim_ledger_parse_msg_update_nym_resp(
            command_handle,
            commit_resp.as_ptr(),
            cb,
        )
    })
}

pub fn build_msg_delete_nym(
    creator: &str,
    id: u64,
) -> Box<dyn Future<Item = (Vec<u8>), Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err = _build_msg_delete_nym(command_handle, creator, id, cb);

    ResultHandler::slice(command_handle, err, receiver)
}

fn _build_msg_delete_nym(
    command_handle: CommandHandle,
    creator: &str,
    id: u64,
    cb: Option<ResponseSliceCB>,
) -> ErrorCode {
    let creator = c_str!(creator);

    ErrorCode::from(unsafe {
        verim_ledger::indy_verim_ledger_build_msg_delete_nym(
            command_handle,
            creator.as_ptr(),
            id,
            cb,
        )
    })
}

pub fn parse_msg_delete_nym_resp(
    commit_resp: &str,
) -> Box<dyn Future<Item = (String), Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_msg_delete_nym_resp(command_handle, commit_resp, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_msg_delete_nym_resp(
    command_handle: CommandHandle,
    commit_resp: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let commit_resp = c_str!(commit_resp);

    ErrorCode::from(unsafe {
        verim_ledger::indy_verim_ledger_parse_msg_delete_nym_resp(
            command_handle,
            commit_resp.as_ptr(),
            cb,
        )
    })
}

pub fn build_query_get_nym(id: u64) -> Box<dyn Future<Item = String, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_query_get_nym(command_handle, id, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_query_get_nym(
    command_handle: CommandHandle,
    id: u64,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    ErrorCode::from(unsafe {
        verim_ledger::indy_verim_ledger_build_query_get_nym(command_handle, id, cb)
    })
}

pub fn parse_query_get_nym_resp(
    query_resp: &str,
) -> Box<dyn Future<Item = String, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_query_get_nym_resp(command_handle, query_resp, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_query_get_nym_resp(
    command_handle: CommandHandle,
    query_resp: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let query_resp = c_str!(query_resp);

    ErrorCode::from(unsafe {
        verim_ledger::indy_verim_ledger_parse_query_get_nym_resp(
            command_handle,
            query_resp.as_ptr(),
            cb,
        )
    })
}
