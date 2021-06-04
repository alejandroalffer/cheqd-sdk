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
            msg_raw: *const u8,
            msg_len: u32,
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
        let (msg_raw, msg_len) = ctypes::vec_to_pointer(&msg);
        cb(command_handle, err, msg_raw, msg_len)
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
pub extern "C" fn indy_verim_ledger_parse_msg_create_nym_resp(
    command_handle: CommandHandle,
    commit_resp: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, msg_resp: *const c_char),
    >,
) -> ErrorCode {
    debug!(
        "indy_verim_ledger_parse_msg_create_nym_resp > commit_resp {:?}",
        commit_resp
    );

    check_useful_c_str!(commit_resp, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!(
        "indy_verim_ledger_parse_msg_create_nym_resp > commit_resp {:?}",
        commit_resp
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .verim_ledger_controller
            .parse_msg_create_nym_resp(&commit_resp);
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, msg_resp) = prepare_result!(res, String::new());
        debug!(
            "indy_verim_ledger_parse_msg_create_nym_resp: msg_resp: {:?}",
            msg_resp
        );
        let msg_resp = ctypes::string_to_cstring(msg_resp);
        cb(command_handle, err, msg_resp.as_ptr())
    };

    locator.executor.spawn_ok_instrumented(
        CommandMetric::VerimLedgerCommandParseMsgCreateNymResp,
        action,
        cb,
    );

    let res = ErrorCode::Success;
    debug!("indy_verim_ledger_parse_msg_create_nym_resp < {:?}", res);
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
            msg_raw: *const u8,
            msg_len: u32,
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
        let (msg_raw, msg_len) = ctypes::vec_to_pointer(&msg);
        cb(command_handle, err, msg_raw, msg_len)
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
pub extern "C" fn indy_verim_ledger_parse_msg_update_nym_resp(
    command_handle: CommandHandle,
    commit_resp: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, msg_resp: *const c_char),
    >,
) -> ErrorCode {
    debug!(
        "indy_verim_ledger_parse_msg_update_nym_resp > commit_resp {:?}",
        commit_resp
    );

    check_useful_c_str!(commit_resp, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!(
        "indy_verim_ledger_parse_msg_update_nym_resp > commit_resp {:?}",
        commit_resp
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .verim_ledger_controller
            .parse_msg_update_nym_resp(&commit_resp);
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, msg_resp) = prepare_result!(res, String::new());
        debug!(
            "indy_verim_ledger_parse_msg_update_nym_resp: msg_resp: {:?}",
            msg_resp
        );
        let msg_resp = ctypes::string_to_cstring(msg_resp);
        cb(command_handle, err, msg_resp.as_ptr())
    };

    locator.executor.spawn_ok_instrumented(
        CommandMetric::VerimLedgerCommandParseMsgUpdateNymResp,
        action,
        cb,
    );

    let res = ErrorCode::Success;
    debug!("indy_verim_ledger_parse_msg_update_nym_resp < {:?}", res);
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
            msg_raw: *const u8,
            msg_len: u32,
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
        debug!("indy_verim_ledger_build_msg_update_nym: msg: {:?}", msg);
        let (msg_raw, msg_len) = ctypes::vec_to_pointer(&msg);
        cb(command_handle, err, msg_raw, msg_len)
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

#[no_mangle]
pub extern "C" fn indy_verim_ledger_parse_msg_delete_nym_resp(
    command_handle: CommandHandle,
    commit_resp: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, msg_resp: *const c_char),
    >,
) -> ErrorCode {
    debug!(
        "indy_verim_ledger_parse_msg_delete_nym_resp > commit_resp {:?}",
        commit_resp
    );

    check_useful_c_str!(commit_resp, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!(
        "indy_verim_ledger_parse_msg_delete_nym_resp > commit_resp {:?}",
        commit_resp
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .verim_ledger_controller
            .parse_msg_delete_nym_resp(&commit_resp);
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, msg_resp) = prepare_result!(res, String::new());
        debug!(
            "indy_verim_ledger_parse_msg_delete_nym_resp: msg_resp: {:?}",
            msg_resp
        );
        let msg_resp = ctypes::string_to_cstring(msg_resp);
        cb(command_handle, err, msg_resp.as_ptr())
    };

    locator.executor.spawn_ok_instrumented(
        CommandMetric::VerimLedgerCommandParseMsgDeleteNymResp,
        action,
        cb,
    );

    let res = ErrorCode::Success;
    debug!("indy_verim_ledger_parse_msg_delete_nym_resp < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_verim_ledger_build_query_get_nym(
    command_handle: CommandHandle,
    id: u64,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, query: *const c_char)>,
) -> ErrorCode {
    debug!("indy_verim_ledger_build_query_get_nym > id {:?}", id);

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!("indy_verim_ledger_build_query_get_nym > id {:?}", id);

    let locator = Locator::instance();

    let action = async move {
        let res = locator.verim_ledger_controller.build_query_get_nym(id);
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, query) = prepare_result!(res, String::new());
        debug!("indy_verim_ledger_build_query_get_nym: query: {:?}", query);

        let query = ctypes::string_to_cstring(query);
        cb(command_handle, err, query.as_ptr())
    };

    locator.executor.spawn_ok_instrumented(
        CommandMetric::VerimLedgerCommandBuildQueryGetNym,
        action,
        cb,
    );

    let res = ErrorCode::Success;
    debug!("indy_verim_ledger_build_query_get_nym < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_verim_ledger_parse_query_get_nym_resp(
    command_handle: CommandHandle,
    query_resp: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, resp: *const c_char)>,
) -> ErrorCode {
    debug!(
        "indy_verim_ledger_parse_query_get_nym_resp > query_resp {:?}",
        query_resp
    );

    check_useful_c_str!(query_resp, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!(
        "indy_verim_ledger_parse_query_get_nym_resp > query_resp {:?}",
        query_resp
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .verim_ledger_controller
            .parse_query_get_nym_resp(&query_resp);
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, resp) = prepare_result!(res, String::new());
        debug!(
            "indy_verim_ledger_parse_query_get_nym_resp: resp: {:?}",
            resp
        );
        let resp = ctypes::string_to_cstring(resp);
        cb(command_handle, err, resp.as_ptr())
    };

    locator.executor.spawn_ok_instrumented(
        CommandMetric::VerimLedgerCommandParseQueryGetNymResp,
        action,
        cb,
    );

    let res = ErrorCode::Success;
    debug!("indy_verim_ledger_parse_query_get_nym_resp < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_verim_ledger_build_query_all_nym(
    command_handle: CommandHandle,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, query: *const c_char)>,
) -> ErrorCode {
    debug!("indy_verim_ledger_build_query_all_nym >");

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam2);

    debug!("indy_verim_ledger_build_query_all_nym >");

    let locator = Locator::instance();

    let action = async move {
        let res = locator.verim_ledger_controller.build_query_all_nym();
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, query) = prepare_result!(res, String::new());
        debug!("indy_verim_ledger_build_query_all_nym: query: {:?}", query);

        let query = ctypes::string_to_cstring(query);
        cb(command_handle, err, query.as_ptr())
    };

    locator.executor.spawn_ok_instrumented(
        CommandMetric::VerimLedgerCommandBuildQueryAllNym,
        action,
        cb,
    );

    let res = ErrorCode::Success;
    debug!("indy_verim_ledger_build_query_all_nym < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_verim_ledger_parse_query_all_nym_resp(
    command_handle: CommandHandle,
    query_resp: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, resp: *const c_char)>,
) -> ErrorCode {
    debug!(
        "indy_verim_ledger_parse_query_all_nym_resp > query_resp {:?}",
        query_resp
    );

    check_useful_c_str!(query_resp, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!(
        "indy_verim_ledger_parse_query_all_nym_resp > query_resp {:?}",
        query_resp
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .verim_ledger_controller
            .parse_query_all_nym_resp(&query_resp);
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, resp) = prepare_result!(res, String::new());
        debug!(
            "indy_verim_ledger_parse_query_all_nym_resp: resp: {:?}",
            resp
        );
        let resp = ctypes::string_to_cstring(resp);
        cb(command_handle, err, resp.as_ptr())
    };

    locator.executor.spawn_ok_instrumented(
        CommandMetric::VerimLedgerCommandParseQueryAllNymResp,
        action,
        cb,
    );

    let res = ErrorCode::Success;
    debug!("indy_verim_ledger_parse_query_all_nym_resp < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_verim_ledger_build_query_cosmos_auth_account(
    command_handle: CommandHandle,
    address: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, query: *const c_char)>,
) -> ErrorCode {
    debug!("indy_verim_ledger_build_query_cosmos_auth_account > address {:?}",
        address
    );

    check_useful_c_str!(address, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!("indy_verim_ledger_build_query_cosmos_auth_account > address {:?}",
        address
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator.verim_ledger_controller
            .build_query_cosmos_auth_account(&address);
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, query) = prepare_result!(res, String::new());
        debug!("indy_verim_ledger_build_query_cosmos_auth_account: query: {:?}", query);

        let query = ctypes::string_to_cstring(query);
        cb(command_handle, err, query.as_ptr())
    };

    locator.executor.spawn_ok_instrumented(
        CommandMetric::VerimLedgerCommandBuildQueryCosmosAuthAccount,
        action,
        cb,
    );

    let res = ErrorCode::Success;
    debug!("indy_verim_ledger_build_query_cosmos_auth_account < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_verim_ledger_parse_query_cosmos_auth_account_resp(
    command_handle: CommandHandle,
    query_resp: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, resp: *const c_char)>,
) -> ErrorCode {
    debug!(
        "indy_verim_ledger_parse_query_cosmos_auth_account_resp > query_resp {:?}",
        query_resp
    );

    check_useful_c_str!(query_resp, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!(
        "indy_verim_ledger_parse_query_cosmos_auth_account_resp > query_resp {:?}",
        query_resp
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .verim_ledger_controller
            .parse_query_cosmos_auth_account_resp(&query_resp);
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, resp) = prepare_result!(res, String::new());
        debug!(
            "indy_verim_ledger_parse_query_cosmos_auth_account_resp: resp: {:?}",
            resp
        );
        let resp = ctypes::string_to_cstring(resp);
        cb(command_handle, err, resp.as_ptr())
    };

    locator.executor.spawn_ok_instrumented(
        CommandMetric::VerimLedgerCommandParseQueryCosmosAuthAccountResp,
        action,
        cb,
    );

    let res = ErrorCode::Success;
    debug!("indy_verim_ledger_parse_query_cosmos_auth_account_resp < {:?}", res);
    res
}