/*use indy_api_types::{CommandHandle, ErrorCode, WalletHandle};
use indy_api_types::errors::prelude::*;
use crate::services::{Ledger2Service, CommandMetric};
use std::os::raw::c_char;
use cosmos_sdk::tx::Msg;
use indy_utils::ctypes;
use crate::Locator;

#[no_mangle]
pub extern "C" fn indy_build_msg_bank_send(
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

    check_useful_c_str!(sender_account_id, ErrorCode::CommonInvalidParam1);
    check_useful_c_str!(recipient_account_id, ErrorCode::CommonInvalidParam2);
    check_useful_opt_u64!(amount, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(denom, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

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


/// Builds a NYM request. Request to create a new NYM record for a specific user.
///
/// #Params
/// creator: An account identifier (address) of the transaction author as string.
/// target_did: Target DID as base58-encoded string for 16 or 32 bit DID value.
/// verkey: Target identity verification key as base58-encoded string.
/// alias: NYM's alias.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Request result as json.
///
/// #Errors
/// Common*
#[no_mangle]
pub extern "C" fn indy_build_nym_request(
    creator: *const c_char,
    target_did: *const c_char,
    verkey: *const c_char,
    alias: *const c_char,
    role: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, request_json: *const c_char),
    >,
) -> ErrorCode {
    debug!(
        "indy_build_nym_request > submitter_did {:?} \
            target_did {:?} verkey {:?} alias {:?} role {:?}",
        submitter_did, target_did, verkey, alias, role
    );

    check_useful_validatable_string!(submitter_did, ErrorCode::CommonInvalidParam2, DidValue);
    check_useful_validatable_string!(target_did, ErrorCode::CommonInvalidParam3, DidValue);
    check_useful_opt_c_str!(verkey, ErrorCode::CommonInvalidParam4);
    check_useful_opt_c_str!(alias, ErrorCode::CommonInvalidParam5);
    check_useful_opt_c_str!(role, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    debug!(
        "indy_build_nym_request? submitter_did {:?} \
            target_did {:?} verkey {:?} alias {:?} role {:?}",
        submitter_did, target_did, verkey, alias, role
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .ledger2_controller
            .build_msg_create_nym(submitter_did, target_did, verkey, alias, role)
            .await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, res) = prepare_result!(res, String::new());
        debug!("indy_build_nym_request ? err {:?} res {:?}", err, res);

        let res = ctypes::string_to_cstring(res);
        cb(command_handle, err, res.as_ptr())
    };

    locator.executor.spawn_ok_instrumented(CommandMetric::Ledger2CommandBuildMsgCreateNym, action, cb);

    let res = ErrorCode::Success;
    boxed_callback_string!("indy_build_msg_create_nym", cb, command_handle);
    res
}

*/