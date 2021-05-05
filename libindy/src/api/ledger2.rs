use indy_api_types::{CommandHandle, ErrorCode, WalletHandle};
use indy_api_types::errors::prelude::*;
use crate::services::{Ledger2Service, CommandMetric};
use std::os::raw::c_char;
use cosmos_sdk::tx::Msg;
use crate::Locator;

pub fn indy_build_msg_bank_send(
    command_handle: CommandHandle,
    sender_account_id: &str,
    recipient_account_id: &str,
    amount: u64,
    denom: &str,
    cb: Option<extern fn(command_handle_: CommandHandle,
                         err: ErrorCode,
                         req_with_fees_json: *const c_char,
                         payment_method: *const c_char)>
) -> ErrorCode {

    debug!("indy_build_msg_bank_send: >>> sender_account_id: {:?}, recipient_account_id: {:?}, amount: {:?}, denom: {:?}",
           sender_account_id, recipient_account_id, amount, denom);

    check_useful_validatable_opt_string!(submitter_did, ErrorCode::CommonInvalidParam2, DidValue);
    check_useful_c_str!(sender_account_id, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(recipient_account_id, ErrorCode::CommonInvalidParam4);
    check_useful_opt_u64!(amount, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(denom, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    debug!("indy_build_msg_bank_send: >>> sender_account_id: {:?}, recipient_account_id: {:?}, amount: {:?}, denom: {:?}",
           sender_account_id, recipient_account_id, amount, denom);

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .ledger2_controller
            .build_msg_bank_send().await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let err = prepare_result!(res);
        debug!("indy_build_msg_bank_send ? err {:?}", err);

        cb(command_handle, err);
    };

    locator.executor.spawn_ok_instrumented(CommandMetric::Ledger2CommandBuildMsgBankSend, action, cb);

    boxed_callback_string!("indy_build_msg_bank_send", cb, command_handle);
}

pub fn indy_build_msg_create_nym(
    command_handle: CommandHandle,
    alias: &str,
    verkey: &str,
    did: &str,
    role: &str,
    from: &str,
    cb: Option<extern fn(command_handle_: CommandHandle,
                         err: ErrorCode,
                         req_with_fees_json: *const c_char,
                         payment_method: *const c_char)>
) -> ErrorCode {
    debug!("indy_build_msg_create_nym: >>> alias: {:?}, verkey: {:?}, did: {:?}, role: {:?}, from: {:?}",
           alias, verkey, did, role, from);

    check_useful_c_str!(alias, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(verkey, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(role, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(from, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    debug!("indy_build_msg_create_nym: >>> sender_account_id: {:?}, recipient_account_id: {:?}, amount: {:?}, denom: {:?}",
           sender_account_id, recipient_account_id, amount, denom);

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .ledger2_controller
            .build_msg_create_nym().await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let err = prepare_result!(res);
        debug!("indy_build_msg_create_nym ? err {:?}", err);

        cb(command_handle, err);
    };

    locator.executor.spawn_ok_instrumented(CommandMetric::Ledger2CommandBuildMsgCreateNym, action, cb);

    boxed_callback_string!("indy_build_msg_create_nym", cb, command_handle);
}