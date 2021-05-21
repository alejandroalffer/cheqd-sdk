use indy_api_types::{errors::prelude::*, CommandHandle, ErrorCode};

use crate::services::CommandMetric;
use crate::Locator;
use indy_utils::ctypes;
use libc::c_char;
use serde_json;

#[no_mangle]
pub extern "C" fn indy_verim_ledger_build_msg_create_nym(
    command_handle: CommandHandle,
    did: *const c_char,
    creator: *const c_char,
    verkey: *const c_char,
    alias: *const c_char,
    role: *const c_char,
    cb: Option<
        extern "C" fn(
            command_handle_: CommandHandle,
            err: ErrorCode,
            signature_raw: *const u8,
            signature_len: u32,
        ),
    >,
) -> ErrorCode {
    debug!(
        "indy_verim_ledger_build_msg_create_nym > did {:?} creator {:?} verkey {:?} alias {:?} role {:?}",
        did, creator, verkey, alias, role
    );

    check_useful_c_str!(did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(creator, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(verkey, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(alias, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(role, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    debug!(
        "indy_verim_ledger_build_msg_create_nym > did {:?} creator {:?} verkey {:?} alias {:?} role {:?}",
        did, creator, verkey, alias, role
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .verim_ledger_controller
            .build_msg_create_nym(&did, &creator, &verkey, &alias, &role);
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, msg) = prepare_result!(res, Vec::new());
        debug!(
            "indy_verim_ledger_build_msg_create_nym: signature: {:?}",
            msg
        );
        let (signature_raw, signature_len) = ctypes::vec_to_pointer(&msg);
        cb(command_handle, err, signature_raw, signature_len)
    };

    locator.executor.spawn_ok_instrumented(
        CommandMetric::VerimLedgerCommandBuildMsgCreateNym,
        action,
        cb,
    );

    let res = ErrorCode::Success;
    debug!("indy_verim_ledger_build_msg_create_nym < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_verim_ledger_build_msg_update_nym(
    command_handle: CommandHandle,
    did: *const c_char,
    creator: *const c_char,
    verkey: *const c_char,
    alias: *const c_char,
    role: *const c_char,
    id: u64,
    cb: Option<
        extern "C" fn(
            command_handle_: CommandHandle,
            err: ErrorCode,
            signature_raw: *const u8,
            signature_len: u32,
        ),
    >,
) -> ErrorCode {
    debug!(
        "indy_verim_ledger_build_msg_update_nym > did {:?} creator {:?} verkey {:?} alias {:?} role {:?} id {:?}",
        did, creator, verkey, alias, role, id,
    );
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(creator, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(verkey, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(alias, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(role, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam8);

    debug!(
        "indy_verim_ledger_build_msg_update_nym > did {:?} creator {:?} verkey {:?} alias {:?} role {:?} id {:?}",
        did, creator, verkey, alias, role, id,
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .verim_ledger_controller
            .build_msg_update_nym(&did, &creator, &verkey, &alias, &role, id);
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, msg) = prepare_result!(res, Vec::new());
        debug!(
            "indy_verim_ledger_build_msg_update_nym: signature: {:?}",
            msg
        );
        let (signature_raw, signature_len) = ctypes::vec_to_pointer(&msg);
        cb(command_handle, err, signature_raw, signature_len)
    };

    locator.executor.spawn_ok_instrumented(
        CommandMetric::VerimLedgerCommandBuildMsgUpdateNym,
        action,
        cb,
    );

    let res = ErrorCode::Success;
    debug!("indy_verim_ledger_build_msg_update_nym < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_verim_ledger_build_msg_delete_nym(
    command_handle: CommandHandle,
    creator: *const c_char,
    id: u64,
    cb: Option<
        extern "C" fn(
            command_handle_: CommandHandle,
            err: ErrorCode,
            signature_raw: *const u8,
            signature_len: u32,
        ),
    >,
) -> ErrorCode {
    debug!(
        "indy_verim_ledger_build_msg_update_nym > creator {:?} id {:?}",
        creator, id
    );

    check_useful_c_str!(creator, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    debug!(
        "indy_verim_ledger_build_msg_update_nym > creator {:?} id {:?}",
        creator, id
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .verim_ledger_controller
            .build_msg_delete_nym(&creator, id);
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, msg) = prepare_result!(res, Vec::new());
        debug!(
            "indy_verim_ledger_build_msg_update_nym: signature: {:?}",
            msg
        );
        let (signature_raw, signature_len) = ctypes::vec_to_pointer(&msg);
        cb(command_handle, err, signature_raw, signature_len)
    };

    locator.executor.spawn_ok_instrumented(
        CommandMetric::VerimLedgerCommandBuildMsgDeleteNym,
        action,
        cb,
    );

    let res = ErrorCode::Success;
    debug!("indy_verim_ledger_build_msg_update_nym < {:?}", res);
    res
}
