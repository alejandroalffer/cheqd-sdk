use crate::{ErrorCode, IndyError};

use std::ffi::CString;

use futures::Future;

use crate::ffi::cheqd_ledger;
use crate::ffi::{ResponseSliceCB, ResponseStringCB};

use crate::utils::callbacks::{ClosureHandler, ResultHandler};
use crate::CommandHandle;

pub fn build_msg_send(
    from: &str,
    to: &str,
    amount: &str,
    denom: &str,
) -> Box<dyn Future<Item = Vec<u8>, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err = _build_msg_send(command_handle, from, to, amount, denom, cb);

    ResultHandler::slice(command_handle, err, receiver)
}

fn _build_msg_send(
    command_handle: CommandHandle,
    from: &str,
    to: &str,
    amount: &str,
    denom: &str,
    cb: Option<ResponseSliceCB>,
) -> ErrorCode {
    let from = c_str!(from);
    let to = c_str!(to);
    let amount = c_str!(amount);
    let denom = c_str!(denom);

    ErrorCode::from(unsafe {
        cheqd_ledger::bank::indy_cheqd_ledger_bank_build_msg_send(
            command_handle,
            from.as_ptr(),
            to.as_ptr(),
            amount.as_ptr(),
            denom.as_ptr(),
            cb,
        )
    })
}

pub fn parse_msg_send_resp(
    commit_resp: &str,
) -> Box<dyn Future<Item = String, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_msg_send_resp(command_handle, commit_resp, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_msg_send_resp(
    command_handle: CommandHandle,
    commit_resp: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let commit_resp = c_str!(commit_resp);

    ErrorCode::from(unsafe {
        cheqd_ledger::bank::indy_cheqd_ledger_bank_parse_msg_send_resp(
            command_handle,
            commit_resp.as_ptr(),
            cb,
        )
    })
}

pub fn build_query_balance(
    address: &str,
    denom: &str,
) -> Box<dyn Future<Item = String, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_query_balance(command_handle, address, denom, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_query_balance(
    command_handle: CommandHandle,
    address: &str,
    denom: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let address = c_str!(address);
    let denom = c_str!(denom);

    ErrorCode::from(unsafe {
        cheqd_ledger::bank::indy_cheqd_ledger_bank_build_query_balance(
            command_handle,
            address.as_ptr(),
            denom.as_ptr(),
            cb,
        )
    })
}

pub fn parse_query_balance_resp(
    commit_resp: &str,
) -> Box<dyn Future<Item = String, Error = IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_query_balance_resp(command_handle, commit_resp, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_query_balance_resp(
    command_handle: CommandHandle,
    commit_resp: &str,
    cb: Option<ResponseStringCB>,
) -> ErrorCode {
    let commit_resp = c_str!(commit_resp);

    ErrorCode::from(unsafe {
        cheqd_ledger::bank::indy_cheqd_ledger_bank_parse_query_balance_resp(
            command_handle,
            commit_resp.as_ptr(),
            cb,
        )
    })
}
